use crate::structs::anomalie::Anomalie;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonOutput {
    pub analysed_file: String,    // Nom du fichier analysé
    pub ai_analyze: u32,          // Nombre d'anomalies détectées par l'IA
    pub regex_analyze: u32,       // Nombre d'anomalies détectées par l'IA et par regex
    pub time_ms: u128,               // Temps d'analyse
    pub anomalies: Vec<Anomalie>, // Liste des anomalies détectées
}

impl JsonOutput {
    /// Créer une nouvelle instance de JsonFile.
    pub fn new(
        anomalies: Vec<Anomalie>,
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

    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let json_data: String = serde_json::to_string_pretty(self)?;
        std::fs::write(file_path, json_data)
    }
}
