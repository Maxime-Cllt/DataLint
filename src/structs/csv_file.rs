use crate::enums::log_level::LogLevel;
use crate::enums::separator::SeparatorType;
use crate::structs::inferable_value::InferableValue;
use crate::structs::logger::{log_and_print_message, print_message};
use crate::utils::regex::{get_safe_regex_set, get_unsafe_value_regex_set};
use crate::utils::util::get_file_name;
use csv::{Reader, ReaderBuilder, StringRecord};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use regex::RegexSet;
use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub struct CsvFile {
    pub csv_file_path: String,
    pub separator: u8,
}

impl CsvFile {
    /// Create a new CsvFile instance with the given path and separator.
    pub fn new(csv_file_path: &str, separator: u8) -> Self {
        CsvFile {
            csv_file_path: String::from(csv_file_path),
            separator,
        }
    }

    /// Return the separator as a char.
    pub fn find_separator_in_file(csv_file_path: &str) -> SeparatorType {
        const POSSIBLE_SEPARATORS: [SeparatorType; 6] = [
            SeparatorType::Semicolon,
            SeparatorType::Tab,
            SeparatorType::Pipe,
            SeparatorType::Null,
            SeparatorType::Comma,
            SeparatorType::Invalid,
        ];

        let first_line: String = Self::read_first_line(csv_file_path).unwrap();

        POSSIBLE_SEPARATORS
            .par_iter()
            .find_any(|sep| first_line.contains(sep.as_char()))
            .cloned()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Error: No valid separator found in the CSV file.",
                )
            })
            .unwrap()
    }

    /// Return the headers of the CSV file as a StringRecord.
    pub fn get_headers(&self) -> Result<StringRecord, Box<dyn Error>> {
        let binding: String = Self::read_first_line(&self.csv_file_path)?;
        let first_line: &str = binding.trim();

        let headers: Vec<&str> = first_line.split(self.separator as char).collect();
        let headers: StringRecord = headers.iter().map(|&s| s.to_string()).collect();
        Ok(headers)
    }

    /// Read the first line of a file and return it as a String.
    pub fn read_first_line(file_path: &str) -> io::Result<String> {
        let file: File = File::open(file_path)?;
        let mut reader: BufReader<File> = BufReader::new(file);
        let mut buffer: String = String::new();

        if reader.read_line(&mut buffer)? > 0 {
            Ok(buffer.trim().into())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Empty file or not found"))
        }
    }

    /// Check if a file is encoded in UTF-8.
    pub fn is_file_utf8(file_path: &str) -> Result<bool, Box<dyn Error>> {
        const CHUNK_SIZE: usize = 16 * 1024; // 16 KB

        let file: File = File::open(file_path)?;
        let mut reader: BufReader<File> = BufReader::new(file);
        let mut buffer: Vec<u8> = vec![0u8; CHUNK_SIZE];

        while let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            if std::str::from_utf8(&buffer[..bytes_read]).is_err() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Convert a file to UTF-8 encoding and save it in the "utf8" directory.
    pub fn convert_file_to_utf8(input_path: &str) -> Result<String, Box<dyn Error>> {
        const UTF8: &str = "utf8";

        if !std::path::Path::new(UTF8).exists() {
            if let Err(e) = std::fs::create_dir(UTF8) {
                log_and_print_message(
                    &format!("Error creating 'utf8' directory: {e}"),
                    LogLevel::Error,
                );
            }
        }

        let encoded_file_name: String = format!("utf8/{}_utf8.csv", get_file_name(input_path));

        let output_file: File = match File::create(&encoded_file_name) {
            Ok(file) => file,
            Err(e) => {
                log_and_print_message(&format!("Error creating output file: {e}"), LogLevel::Error);
                std::process::exit(1);
            }
        };

        let input_file: File = File::open(input_path)?;
        let mut reader: BufReader<File> = BufReader::new(input_file);
        let mut writer: BufWriter<File> = BufWriter::new(output_file);

        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let decoded: Cow<str> = String::from_utf8_lossy(&buffer);
        writer.write_all(decoded.as_bytes())?;
        Ok(encoded_file_name)
    }

    /// Create a CsvFile instance from a file path, checking its encoding and separator.
    pub fn from_file(csv_file_path: &str) -> Result<CsvFile, Box<dyn Error>> {
        let is_utf8: bool = Self::is_file_utf8(csv_file_path).map_err(|e| {
            log_and_print_message(
                &format!("Error checking file encoding: {e}",),
                LogLevel::Error,
            );
            e
        })?;

        let csv_file_path: String = if !is_utf8 {
            Self::convert_file_to_utf8(csv_file_path).map_err(|e| {
                log_and_print_message(
                    &format!("Error converting file to UTF-8: {e}"),
                    LogLevel::Error,
                );
                e
            })?
        } else {
            String::from(csv_file_path)
        };

        let separator: u8 = u8::from(match Self::find_separator_in_file(&csv_file_path) {
            SeparatorType::Comma => SeparatorType::Comma,
            SeparatorType::Semicolon => SeparatorType::Semicolon,
            SeparatorType::Tab => SeparatorType::Tab,
            SeparatorType::Pipe => SeparatorType::Pipe,
            SeparatorType::Null => SeparatorType::Null,
            _ => {
                log_and_print_message(
                    "Error: Unable to detect a valid separator in the CSV file.",
                    LogLevel::Error,
                );
                std::process::exit(1);
            }
        });

        Ok(CsvFile::new(&csv_file_path, separator))
    }

    /// Collect unsafe values from the CSV file based on regex patterns.
    pub fn collect_unsafe_value(
        &self,
        csv_file_struct: &CsvFile,
        regex_analyze: &mut u32,
    ) -> Result<Vec<InferableValue>, Box<dyn Error>> {
        let csv_file: File = File::open(&csv_file_struct.csv_file_path)?;
        let mut rdr: Reader<File> = ReaderBuilder::new()
            .delimiter(csv_file_struct.separator)
            .has_headers(true)
            .from_reader(csv_file);

        let safe_regex_set: RegexSet = get_safe_regex_set(); // Regex for safe values
        let unsafe_regex_set: RegexSet = get_unsafe_value_regex_set(); // Regex for unsafe values
        let mut seen_words: HashSet<String> = HashSet::new(); // Store seen words to avoid duplicates
        let mut batch_data: Vec<InferableValue> = Vec::new();

        for (row_number, record) in rdr.records().enumerate() {
            let record: StringRecord = match record {
                Ok(record) => record,
                Err(e) => {
                    print_message(
                        &format!("Error reading record at row {row_number}: {e}"),
                        &LogLevel::Error,
                    );
                    continue;
                }
            };

            for (column_index, raw_value) in record.iter().enumerate() {
                let value: &str = raw_value.trim();

                if value.is_empty() || seen_words.contains(value) || safe_regex_set.is_match(value)
                {
                    *regex_analyze += 1;
                    continue;
                }

                if !unsafe_regex_set.is_match(value) {
                    continue;
                }

                seen_words.insert(value.into());

                batch_data.push(InferableValue {
                    value: value.into(),
                    row_number,
                    column_index,
                });
            }
        }

        Ok(batch_data)
    }
}
