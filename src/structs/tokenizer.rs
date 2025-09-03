use crate::enums::log_level::LogLevel;
use crate::structs::inferable_value::InferableValue;
use crate::structs::logger::print_message;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::error::Error;
use tokenizers::{Encoding, Tokenizer};

/// Represents a tokenizer for the model, providing methods to encode and decode text data.
#[non_exhaustive]
pub struct ModelTokenizer;

impl ModelTokenizer {
    /// Load the tokenizer from a configuration file.
    pub fn from_config_file(file_path: &str) -> Result<Tokenizer, Box<dyn Error>> {
        let tokenizer: Tokenizer = Tokenizer::from_file(file_path).unwrap_or_else(|e| {
            print_message(
                &format!("Erreur lors de la lecture du fichier de vocabulaire: {e}"),
                &LogLevel::Error,
            );
            std::process::exit(1);
        });
        Ok(tokenizer)
    }

    /// Encode the words from a batch of `InferableValue` into a vector of `Encoding` and returns the maximum sequence length.
    pub fn encode_words(
        tokenizer: &Tokenizer,
        batch_data: &[InferableValue],
    ) -> (Vec<Encoding>, i64) {
        let encodings: Vec<Encoding> = batch_data
            .iter()
            .map(|data| tokenizer.encode(data.value.clone(), true).unwrap())
            .collect();

        let max_seq_length: i64 = encodings
            .iter()
            .map(|e| e.get_ids().len())
            .max()
            .unwrap_or(0) as i64;

        (encodings, max_seq_length)
    }

    /// Convert the IDs from an `Encoding` into a vector of `i64` and returns the sequence length.
    #[inline]
    #[must_use]
    pub fn ids_to_vector(encoding: &Encoding) -> (Vec<i64>, i64) {
        let ids: &[u32] = encoding.get_ids();
        let ids: Vec<i64> = ids.par_iter().map(|&x| i64::from(x)).collect();
        let seq_length: i64 = i64::try_from(ids.len()).unwrap_or(i64::MAX);
        (ids, seq_length)
    }

    /// Build padded token IDs and attention masks from a list of `Encoding` objects.
    #[inline]
    #[must_use]
    pub fn build_tokens(encodings: &[Encoding], max_seq_length: i64) -> (Vec<i64>, Vec<i64>) {
        const PAD_TOKEN_ID: i64 = 0;
        let batch_size: usize = encodings.len();
        let total_len: usize = batch_size * (usize::try_from(max_seq_length).unwrap_or_default());

        let mut padded_ids: Vec<i64> = Vec::with_capacity(total_len);
        let mut attention_mask: Vec<i64> = Vec::with_capacity(total_len);

        for encoding in encodings {
            let (ids, seq_len_i64) = Self::ids_to_vector(encoding);
            let seq_len: usize = usize::try_from(seq_len_i64).unwrap_or_default();
            let pad_len: usize = usize::try_from(max_seq_length)
                .unwrap_or_default()
                .saturating_sub(seq_len);

            padded_ids.extend_from_slice(&ids);
            padded_ids.resize(padded_ids.len() + pad_len, PAD_TOKEN_ID);

            attention_mask.extend(std::iter::repeat_n(1, seq_len));
            attention_mask.extend(std::iter::repeat_n(0, pad_len));
        }

        (padded_ids, attention_mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_ids_to_vector_basic() {
        const WORDS: [&str; 5] = ["TEST", "--", "IN", "", "RUST IS FUN BUT WINDOWS IS NOT"];

        let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("model/tokenizer.json");
        let tokenizer: Tokenizer = Tokenizer::from_file(path).unwrap_or_else(|e| {
            print_message(
                &format!("Error reading vocabulary file: {e}"),
                &LogLevel::Error,
            );
            std::process::exit(1);
        });

        for word in WORDS {
            let encoding: Encoding = tokenizer.encode(word, true).unwrap();
            let (ids, seq_length) = ModelTokenizer::ids_to_vector(&encoding);

            assert_eq!(seq_length, ids.len() as i64);
            assert!(!ids.is_empty(), "Should produce token IDs");

            let encoding_ids: Vec<u32> = encoding.get_ids().to_vec();

            // loop in ids and check if they ids(is) == encoding_ids(i)
            for (i, id) in ids.iter().enumerate() {
                assert_eq!(
                    u32::try_from(*id).unwrap_or_default(),
                    encoding_ids[i],
                    "Token ID should match the encoding ID"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_encode_decode() {
        const WORDS: [&str; 5] = ["TEST", "WORD", "IN", "", "RUST IS FUN BUT WINDOWS IS NOT"];

        let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("model/tokenizer.json");
        let tokenizer: Tokenizer = Tokenizer::from_file(path).unwrap_or_else(|e| {
            print_message(
                &format!("Error reading vocabulary file: {e}"),
                &LogLevel::Error,
            );
            std::process::exit(1);
        });

        for word in WORDS {
            let encoding: Encoding = tokenizer.encode(word, true).unwrap();
            let decoded: String = tokenizer.decode(encoding.get_ids(), true).unwrap();

            assert_eq!(
                decoded,
                word.to_lowercase(),
                "Decoded word should match the original"
            );
        }
    }

    #[tokio::test]
    async fn test_encode_words() {
        const WORDS: [&str; 5] = ["TEST", "WORD", "IN", "", "RUST IS FUN BUT WINDOWS IS NOT"];

        let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("model/tokenizer.json");
        let tokenizer: Tokenizer = Tokenizer::from_file(path).unwrap_or_else(|e| {
            print_message(
                &format!("Error reading vocabulary file: {e}"),
                &LogLevel::Error,
            );
            panic!("Failed to load tokenizer");
        });

        let batch_data: Vec<InferableValue> = WORDS
            .iter()
            .enumerate()
            .map(|(i, &word)| InferableValue {
                value: word.into(),
                row_number: i,
                column_index: 0,
            })
            .collect();

        let (encodings, max_seq_length) = ModelTokenizer::encode_words(&tokenizer, &batch_data);

        assert_eq!(
            encodings.len(),
            batch_data.len(),
            "Encodings should match batch size"
        );
        assert!(
            max_seq_length > 0,
            "Max sequence length should be greater than 0"
        );
    }

    #[tokio::test]
    async fn test_build_token() {
        const WORDS: [&str; 5] = ["TEST", "WORD", "IN", "", "RUST IS FUN BUT WINDOWS IS NOT"];

        let path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("model/tokenizer.json");
        let tokenizer: Tokenizer = Tokenizer::from_file(path).unwrap_or_else(|e| {
            print_message(
                &format!("Error reading vocabulary file: {e}"),
                &LogLevel::Error,
            );
            std::process::exit(1);
        });

        let batch_data: Vec<InferableValue> = WORDS
            .iter()
            .enumerate()
            .map(|(i, &word)| InferableValue {
                value: word.into(),
                row_number: i,
                column_index: 0,
            })
            .collect();

        let (encodings, max_seq_length) = ModelTokenizer::encode_words(&tokenizer, &batch_data);
        let (padded_ids, attention_masks) =
            ModelTokenizer::build_tokens(&encodings, max_seq_length);

        assert_eq!(
            padded_ids.len(),
            batch_data.len() * usize::try_from(max_seq_length).unwrap_or(0),
        );
        assert_eq!(
            attention_masks.len(),
            batch_data.len() * usize::try_from(max_seq_length).unwrap_or(0)
        );
    }
}
