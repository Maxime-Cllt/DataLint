use crate::enums::log_level::LogLevel;
use crate::structs::anomaly::Anomaly;
use crate::structs::csv_file::CsvFile;
use crate::structs::inferable_value::InferableValue;
use crate::structs::logger::print_message;
use crate::structs::tokenizer::ModelTokenizer;
use csv::StringRecord;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use tch::{CModule, Device, Tensor};
use tokenizers::{Encoding, Tokenizer};

#[derive(Deserialize)]
pub struct PerfageModel {
    pub model_path: String,
    pub vocabulary_path: String,
}

impl PerfageModel {
    /// Load the model configuration from a JSON file and return a PerfageModel instance.
    pub fn from_config_file(json_path: &str) -> Result<Self, Box<dyn Error>> {
        let json_file: File = File::open(json_path)?;
        let model: PerfageModel = serde_json::from_reader(json_file).unwrap_or_else(|e| {
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
                print_message(
                    &format!("Error loading model: {e}"),
                    &LogLevel::Error,
                );
                std::process::exit(1);
            });
        let tokenizer: Tokenizer = ModelTokenizer::from_config_file(&self.vocabulary_path)?;
        Ok((model, device, tokenizer))
    }

    /// Analyse a CSV file and return a tuple containing the detected anomalies,
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

    fn forward(model: &mut CModule, input_ids: Tensor, attention_mask: Tensor) -> Tensor {
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
        const MAX_BATCH_SIZE: usize = 32;
        model.set_eval();

        // Fast path for small batches (optional performance boost)
        if encodings.len() < 5000 {
            return Self::run_single_batch_inference(encodings, max_seq_length, model, device)
                .sigmoid();
        }

        let mut all_outputs: Vec<Tensor> = Vec::new();

        for batch in encodings.chunks(MAX_BATCH_SIZE) {
            let output:Tensor = Self::run_single_batch_inference(batch, max_seq_length, model, device);
            all_outputs.push(output);
        }

        Tensor::cat(&all_outputs, 0).sigmoid()
    }

    /// Run inference for a single batch of encodings.
    fn run_single_batch_inference(
        batch: &[Encoding],
        max_seq_length: i64,
        model: &mut CModule,
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
        let mut anomalies: Vec<Anomaly> = Vec::new();

        // Get prediction scores as a 1D vector
        let scores = predictions.select(1, 1).iter::<f64>().unwrap();

        for (i, score) in scores.enumerate() {
            *ai_analyze += 1;

            if score > THRESHOLD {
                if let Some(data) = batch_data.get(i) {
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
        }

        anomalies
    }
}
