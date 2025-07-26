use crate::enums::color::Color;
use crate::enums::log_level::LogLevel;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Mutex, MutexGuard};

/// Logger struct to handle logging messages to a file
#[non_exhaustive]
pub struct Logger {
    log_file: File,
}

impl Logger {
    /// Create a new Logger instance
    fn new() -> Self {
        let log_file: File = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("DataLint.log")
            .unwrap();

        Self { log_file }
    }

    /// Log a message with the specified log level
    fn log(&self, log_level: &LogLevel, message: &str) {
        let mut log_writer: BufWriter<&File> = BufWriter::new(&self.log_file);
        writeln!(
            log_writer,
            "[{:?}] {} {message}",
            chrono::Local::now(),
            log_level.as_str()
        )
        .unwrap_or_else(|_| panic!("Error writing to log file"));
    }
}

/// Static logger instance
pub static LOGGER: std::sync::LazyLock<Mutex<Logger>> =
    std::sync::LazyLock::new(|| Mutex::new(Logger::new()));

/// Static logger instance
pub fn log_and_print_message(message: &str, log_level: &LogLevel) {
    print_message(message, log_level);
    let logger: MutexGuard<Logger> = LOGGER.lock().unwrap();
    logger.log(log_level, message);
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
