use crate::core::io::tracing::log_level::LogLevel;
use crate::detection::anomaly::Anomaly;
use crate::core::io::file::csv_file::CsvFile;
use crate::detection::inferable_value::InferableValue;
use crate::model::tokenizer::ModelTokenizer;
use csv::StringRecord;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use tch::{CModule, Device, Tensor};
use tokenizers::{Encoding, Tokenizer};
use crate::core::io::tracing::logger::print_message;

/// Represents the model configuration for the anomaly detection system.
/// It contains the paths to the model and vocabulary files.
/// The model is used for inference, while the vocabulary is used for tokenization.
/// The model is expected to be a PyTorch model, and the vocabulary is expected to be a tokenizer configuration file.
#[derive(Deserialize)]
pub struct Model {
    pub model_path: String,
    pub vocabulary_path: String,
}

/// Initialized model with loaded PyTorch model and tokenizer.
/// This struct should be reused across multiple file analyses to avoid
/// the expensive model reloading overhead.
pub struct InitializedModel {
    model: CModule,
    device: Device,
    tokenizer: Tokenizer,
}

impl Model {
    /// Load the model configuration from a JSON file and return a Model instance.
    pub fn from_config_file(json_path: &str) -> Result<Self, Box<dyn Error>> {
        let json_file: File = File::open(json_path)?;
        let model = serde_json::from_reader(json_file).unwrap_or_else(|e| {
            print_message(
                &format!("Error reading model configuration from JSON: {e}"),
                &LogLevel::Error,
            );
            std::process::exit(1);
        });

        Ok(model)
    }

    /// Init the model, device, and tokenizer based on the model path and vocabulary path.
    fn init_model(&self) -> Result<(CModule, Device, Tokenizer), Box<dyn Error>> {
        let device: Device = Device::cuda_if_available();
        let model: CModule =
            CModule::load_on_device(&self.model_path, device).unwrap_or_else(|e| {
                print_message(&format!("Error loading model: {e}"), &LogLevel::Error);
                std::process::exit(1);
            });
        let tokenizer: Tokenizer = ModelTokenizer::from_config_file(&self.vocabulary_path)?;
        Ok((model, device, tokenizer))
    }

    /// Analyse a CSV file and return a tuple containing the detected anomalies,
    /// the number of AI analyses performed, and the number of regex analyses performed.
    ///
    /// **Note**: This method reinitializes the model on every call, which is expensive.
    /// Consider using `InitializedModel::new()` and `InitializedModel::analyse_file()` instead
    /// for better performance when analyzing multiple files.
    pub fn analyse_file(
        &self,
        csv_file_struct: &CsvFile,
    ) -> Result<(Vec<Anomaly>, u32, u32), Box<dyn Error>> {
        let mut regex_analyze: u32 = 0;
        let mut ai_analyze: u32 = 0;

        let batch_data: Vec<InferableValue> =
            csv_file_struct.collect_unsafe_value(csv_file_struct, &mut regex_analyze)?;

        if batch_data.is_empty() {
            return Ok((Vec::new(), ai_analyze, regex_analyze));
        }

        let (mut model, device, tokenizer): (CModule, Device, Tokenizer) = self.init_model()?;

        let (encodings, max_seq_length) = ModelTokenizer::encode_words(&tokenizer, &batch_data);

        let predictions: Tensor =
            Self::run_sigmoid_inference_batched(&encodings, max_seq_length, &mut model, device);

        let anomalies: Vec<Anomaly> = Self::process_output(
            &batch_data,
            &predictions,
            &csv_file_struct.get_headers()?,
            &mut ai_analyze,
        );

        Ok((anomalies, ai_analyze, regex_analyze))
    }

    /// Forward pass through the model with input IDs and attention mask.
    fn forward(model: &CModule, input_ids: Tensor, attention_mask: Tensor) -> Tensor {
        let output: Tensor = tch::no_grad(|| {
            model
                .forward_ts(&[input_ids, attention_mask])
                .unwrap_or_else(|e| {
                    print_message(
                        &format!("Error during model inference: {e}"),
                        &LogLevel::Error,
                    );
                    std::process::exit(1);
                })
        });

        output
    }

    /// Execute the inference in batches using sigmoid activation.
    fn run_sigmoid_inference_batched(
        encodings: &[Encoding],
        max_seq_length: i64,
        model: &mut CModule,
        device: Device,
    ) -> Tensor {
        const MAX_BATCH_SIZE: usize = 512;  // Increased from 32 for better GPU utilization
        model.set_eval();

        // Fast path for small batches (optional performance boost)
        if encodings.len() < 5000 {
            return Self::run_single_batch_inference(encodings, max_seq_length, model, device)
                .sigmoid();
        }

        // Pre-allocate vector with exact capacity to avoid reallocations
        let num_batches = (encodings.len() + MAX_BATCH_SIZE - 1) / MAX_BATCH_SIZE;
        let mut all_outputs: Vec<Tensor> = Vec::with_capacity(num_batches);

        for batch in encodings.chunks(MAX_BATCH_SIZE) {
            let output: Tensor =
                Self::run_single_batch_inference(batch, max_seq_length, model, device);
            all_outputs.push(output);
        }

        Tensor::cat(&all_outputs, 0).sigmoid()
    }

    /// Run inference for a single batch of encodings.
    fn run_single_batch_inference(
        batch: &[Encoding],
        max_seq_length: i64,
        model: &CModule,
        device: Device,
    ) -> Tensor {
        let (padded_ids, attention_masks) = ModelTokenizer::build_tokens(batch, max_seq_length);
        let batch_size: i64 = i64::try_from(batch.len()).unwrap_or(0);

        let input_ids: Tensor = Tensor::from_slice(&padded_ids)
            .view((batch_size, max_seq_length))
            .to_device(device);

        let attention_mask: Tensor = Tensor::from_slice(&attention_masks)
            .view((batch_size, max_seq_length))
            .to_device(device);

        Self::forward(model, input_ids, attention_mask)
    }

    /// Extract anomalies from the model's predictions and batch data.
    fn process_output(
        batch_data: &[InferableValue],
        predictions: &Tensor,
        headers: &StringRecord,
        ai_analyze: &mut u32,
    ) -> Vec<Anomaly> {
        const THRESHOLD: f64 = 0.8;
        // Pre-allocate with conservative estimate (10% anomaly rate)
        let estimated_capacity = batch_data.len() / 10;
        let mut anomalies: Vec<Anomaly> = Vec::with_capacity(estimated_capacity);

        // Get prediction scores as a 1D vector
        let scores = predictions.select(1, 1).iter::<f64>().unwrap();

        for (i, score) in scores.enumerate() {
            *ai_analyze += 1;

            // Check if the score exceeds the threshold and if the corresponding data exists
            if score > THRESHOLD && let Some(data) = batch_data.get(i)
            {
                let column_name: String =
                    headers.get(data.column_index).unwrap_or("unknown").into();
                let row_number: u32 = u32::try_from(data.row_number + 2).unwrap_or(u32::MAX);

                anomalies.push(Anomaly::new(
                    data.value.clone(),
                    column_name,
                    row_number,
                    score as f32,
                ));
            }
        }

        anomalies
    }
}

impl InitializedModel {
    /// Load and initialize the model once from a configuration file.
    /// This is much more efficient than using `Model::analyse_file()` repeatedly,
    /// as the model is loaded only once and can be reused across multiple analyses.
    ///
    /// # Example
    /// ```no_run
    /// use datalib::model::model::InitializedModel;
    /// let mut model = InitializedModel::new("config.json")?;
    /// let (anomalies, ai_count, regex_count) = model.analyse_file(&csv_file)?;
    /// ```
    pub fn new(config_path: &str) -> Result<Self, Box<dyn Error>> {
        let config = Model::from_config_file(config_path)?;
        let device: Device = Device::cuda_if_available();
        let model: CModule =
            CModule::load_on_device(&config.model_path, device).unwrap_or_else(|e| {
                print_message(&format!("Error loading model: {e}"), &LogLevel::Error);
                std::process::exit(1);
            });
        let tokenizer: Tokenizer = ModelTokenizer::from_config_file(&config.vocabulary_path)?;

        Ok(Self {
            model,
            device,
            tokenizer,
        })
    }

    /// Analyse a CSV file using the pre-loaded model.
    /// Returns a tuple containing the detected anomalies, AI analysis count, and regex analysis count.
    /// This method reuses the model already loaded in memory, making it much faster
    /// than calling `Model::analyse_file()` repeatedly.
    pub fn analyse_file(
        &mut self,
        csv_file_struct: &CsvFile,
    ) -> Result<(Vec<Anomaly>, u32, u32), Box<dyn Error>> {
        let mut regex_analyze: u32 = 0;
        let mut ai_analyze: u32 = 0;

        let batch_data: Vec<InferableValue> =
            csv_file_struct.collect_unsafe_value(csv_file_struct, &mut regex_analyze)?;

        if batch_data.is_empty() {
            return Ok((Vec::new(), ai_analyze, regex_analyze));
        }

        let (encodings, max_seq_length) = ModelTokenizer::encode_words(&self.tokenizer, &batch_data);

        let predictions: Tensor = Self::run_sigmoid_inference_batched(
            &encodings,
            max_seq_length,
            &mut self.model,
            self.device,
        );

        let anomalies: Vec<Anomaly> = Self::process_output(
            &batch_data,
            &predictions,
            &csv_file_struct.get_headers()?,
            &mut ai_analyze,
        );

        Ok((anomalies, ai_analyze, regex_analyze))
    }

    /// Execute the inference in batches using sigmoid activation.
    fn run_sigmoid_inference_batched(
        encodings: &[Encoding],
        max_seq_length: i64,
        model: &mut CModule,
        device: Device,
    ) -> Tensor {
        const MAX_BATCH_SIZE: usize = 512;  // Increased from 32 for better GPU utilization
        model.set_eval();

        // Fast path for small batches (optional performance boost)
        if encodings.len() < 5000 {
            return Self::run_single_batch_inference(encodings, max_seq_length, model, device)
                .sigmoid();
        }

        // Pre-allocate vector with exact capacity to avoid reallocations
        let num_batches = (encodings.len() + MAX_BATCH_SIZE - 1) / MAX_BATCH_SIZE;
        let mut all_outputs: Vec<Tensor> = Vec::with_capacity(num_batches);

        for batch in encodings.chunks(MAX_BATCH_SIZE) {
            let output: Tensor =
                Self::run_single_batch_inference(batch, max_seq_length, model, device);
            all_outputs.push(output);
        }

        Tensor::cat(&all_outputs, 0).sigmoid()
    }

    /// Run inference for a single batch of encodings.
    fn run_single_batch_inference(
        batch: &[Encoding],
        max_seq_length: i64,
        model: &CModule,
        device: Device,
    ) -> Tensor {
        let (padded_ids, attention_masks) = ModelTokenizer::build_tokens(batch, max_seq_length);
        let batch_size: i64 = i64::try_from(batch.len()).unwrap_or(0);

        let input_ids: Tensor = Tensor::from_slice(&padded_ids)
            .view((batch_size, max_seq_length))
            .to_device(device);

        let attention_mask: Tensor = Tensor::from_slice(&attention_masks)
            .view((batch_size, max_seq_length))
            .to_device(device);

        Self::forward(model, input_ids, attention_mask)
    }

    /// Forward pass through the model with input IDs and attention mask.
    fn forward(model: &CModule, input_ids: Tensor, attention_mask: Tensor) -> Tensor {
        let output: Tensor = tch::no_grad(|| {
            model
                .forward_ts(&[input_ids, attention_mask])
                .unwrap_or_else(|e| {
                    print_message(
                        &format!("Error during model inference: {e}"),
                        &LogLevel::Error,
                    );
                    std::process::exit(1);
                })
        });

        output
    }

    /// Extract anomalies from the model's predictions and batch data.
    fn process_output(
        batch_data: &[InferableValue],
        predictions: &Tensor,
        headers: &StringRecord,
        ai_analyze: &mut u32,
    ) -> Vec<Anomaly> {
        const THRESHOLD: f64 = 0.8;
        // Pre-allocate with conservative estimate (10% anomaly rate)
        let estimated_capacity = batch_data.len() / 10;
        let mut anomalies: Vec<Anomaly> = Vec::with_capacity(estimated_capacity);

        // Get prediction scores as a 1D vector
        let scores = predictions.select(1, 1).iter::<f64>().unwrap();

        for (i, score) in scores.enumerate() {
            *ai_analyze += 1;

            // Check if the score exceeds the threshold and if the corresponding data exists
            if score > THRESHOLD && let Some(data) = batch_data.get(i)
            {
                let column_name: String =
                    headers.get(data.column_index).unwrap_or("unknown").into();
                let row_number: u32 = u32::try_from(data.row_number + 2).unwrap_or(u32::MAX);

                anomalies.push(Anomaly::new(
                    data.value.clone(),
                    column_name,
                    row_number,
                    score as f32,
                ));
            }
        }

        anomalies
    }
}
