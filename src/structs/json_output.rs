use crate::structs::anomaly::Anomaly;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonOutput {
    pub analysed_file: String,
    pub ai_analyze: u32,
    pub regex_analyze: u32,
    pub time_ms: u128,
    pub anomalies: Vec<Anomaly>,
}

impl JsonOutput {
    /// Create a new instance of JsonOutput
    pub fn new(
        anomalies: Vec<Anomaly>,
        analysed_file: String,
        ai_analyze: u32,
        regex_analyze: u32,
        time_ms: u128,
    ) -> Self {
        JsonOutput {
            anomalies,
            analysed_file,
            ai_analyze,
            regex_analyze,
            time_ms,
        }
    }

    /// Save the JsonOutput to a file in pretty JSON format
    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let json_data: String = serde_json::to_string_pretty(self)?;
        std::fs::write(file_path, json_data)
    }
}
