use crate::enums::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Anomalie {
    pub valeur: String,  // 24 bytes
    pub colonne: String, // 24 bytes
    pub score: f32,      // 4 bytes
    pub ligne: u32,      // 4 bytes
}

impl Anomalie {
    /// Créer une nouvelle instance d'Anomalie.
    pub const fn new(valeur: String, colonne: String, ligne: u32, score: f32) -> Self {
        Anomalie {
            valeur,
            colonne,
            ligne,
            score,
        }
    }

    /// Affiche les anomalies détectées dans le fichier CSV via une HashMap
    pub fn print_result(anomalie_vec: &[Anomalie]) {
        for anomalie in anomalie_vec.iter() {
            println!("{}", anomalie.as_str());
        }
    }

    /// Retourne une chaîne de caractères contenant les informations de l'anomalie
    pub fn as_str(&self) -> String {
        format!(
            "Contenu: {}{}{}, \nColonne: {}{}{}, \nLigne: {}{}{}, \nScore: {}{}{}\n-----",
            Color::Red,
            self.valeur,
            Color::Reset,
            Color::Green,
            self.colonne,
            Color::Reset,
            Color::Blue,
            self.ligne,
            Color::Reset,
            Color::Yellow,
            self.score,
            Color::Reset
        )
    }
}
