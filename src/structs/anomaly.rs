use crate::enums::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Anomaly {
    pub value: String,
    pub column: String,
    pub score: f32,
    pub line: u32,
}

impl Anomaly {
    /// Create a new instance of Anomaly
    pub const fn new(value: String, column: String, line: u32, score: f32) -> Self {
        Anomaly {
            value,
            column,
            line,
            score,
        }
    }

    /// Display the anomalies in a formatted way
    pub fn print_result(anomalie_vec: &[Anomaly]) {
        for anomalie in anomalie_vec.iter() {
            println!("{}", anomalie.as_str());
        }
    }

    /// Return a formatted string representation of the anomaly
    pub fn as_str(&self) -> String {
        format!(
            "Content: {}{}{}, \nColumn: {}{}{}, \nLine: {}{}{}, \nScore: {}{}{}\n-----",
            Color::Red,
            self.value,
            Color::Reset,
            Color::Green,
            self.column,
            Color::Reset,
            Color::Blue,
            self.line,
            Color::Reset,
            Color::Yellow,
            self.score,
            Color::Reset
        )
    }
}
