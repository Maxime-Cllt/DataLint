use crate::enums::color::Color;
use crate::enums::log_level::LogLevel;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Mutex, MutexGuard};

pub struct Logger {
    log_file: File,
}

impl Logger {
    /// Crée une nouvelle instance de Logger
    fn new() -> Self {
        let log_file: File = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("DataLint.log")
            .unwrap();

        Self { log_file }
    }

    /// Log un message
    fn log(&self, log_level: &LogLevel, message: &str) {
        let mut log_writer: BufWriter<&File> = BufWriter::new(&self.log_file);
        writeln!(
            log_writer,
            "[{:?}] {} {message}",
            chrono::Local::now(),
            log_level.as_str()
        )
        .unwrap_or_else(|_| panic!("Impossible d'écrire dans le fichier de log"));
    }
}

/// Static logger instance
pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new()));

/// Static logger instance
pub fn log_and_print_message(message: &str, log_level: LogLevel) {
    print_message(message, &log_level);
    let logger: MutexGuard<Logger> = LOGGER.lock().unwrap();
    logger.log(&log_level, message);
}

pub fn log_message(message: &str, log_level: &LogLevel) {
    let logger: MutexGuard<Logger> = LOGGER.lock().unwrap();
    logger.log(log_level, message);
}

pub fn print_message(message: &str, log_level: &LogLevel) {
    match log_level {
        LogLevel::Error => eprintln!("{}[ERROR] {message}{}", Color::Red, Color::Reset),
        LogLevel::Info => println!("{}[INFO] {message}{}", Color::Green, Color::Reset),
    }
}
