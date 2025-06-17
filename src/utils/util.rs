use crate::enums::color::Color;
use crate::enums::log_level::LogLevel;
use crate::structs::anomaly::Anomaly;
use crate::structs::json_output::JsonOutput;
use crate::structs::logger::{log_and_print_message, log_message, print_message};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::time::Instant;

/// Create a JSON file with the analysis results.
pub fn generate_json_file(
    dangerous_output: Vec<Anomaly>,
    regex_analyze: u32,
    ai_analyze: u32,
    analysed_file: &str,
    output_file_name: &str,
    time_ms: u128,
) {
    const JSON_DIR: &str = "json";
    if !std::path::Path::new(JSON_DIR).exists() {
        if let Err(e) = std::fs::create_dir(JSON_DIR) {
            log_and_print_message(
                format!(
                    "Error while creating the JSON directory: {e}. Please create the directory manually at: {}",
                    JSON_DIR
                )
                .as_str(),
                LogLevel::Error,
            );
        }
    }

    let binding: PathBuf = std::env::current_dir().unwrap();
    let current_dir: &str = binding.to_str().unwrap();
    let save_path: String = format!("{JSON_DIR}/{output_file_name}.{JSON_DIR}");

    let json_response: JsonOutput = JsonOutput::new(
        dangerous_output,
        String::from(analysed_file),
        ai_analyze,
        regex_analyze,
        time_ms,
    );

    if let Err(e) = json_response.save_to_file(&save_path) {
        log_and_print_message(
            format!("Error while saving the JSON file: {e}").as_str(),
            LogLevel::Error,
        );
        return;
    }

    if cfg!(debug_assertions) {
        print_message(
            format!(
                "JSON file created at : {}{}/{JSON_DIR}{}",
                Color::Red,
                &current_dir,
                Color::Reset
            )
            .as_str(),
            &LogLevel::Info,
        );
    }
}

/// Get the CSV file path and output JSON file name from command line arguments.
pub fn get_file_from_args(args: &[String]) -> Result<[String; 2], Error> {
    if args.len() != 3 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Error you must provide exactly 2 arguments: the CSV file path and the output JSON file name.",
        ));
    }

    if !file_exists(&args[1]) {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("File {} does not exist", &args[1]),
        ));
    }

    if !args[1].ends_with(".csv") {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("File {} must be in CSV format", &args[1]),
        ));
    }

    if !args[2].ends_with(".json") {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("File {} must be in JSON format", &args[2]),
        ));
    }

    let csv_file_path: String = String::from(&args[1].replace("\\", "/"));
    let output_name: String = String::from(get_file_name(&args[2]));
    Ok([csv_file_path, output_name])
}

/// Display a report of the analysis results, including the number of anomalies found, the CSV file analyzed, and the execution time.
pub fn print_report(
    start_time: &Instant,
    dangerous_output: &[Anomaly],
    csv_file_path: &str,
    debug: bool,
) {
    let anomalies_count: usize = dangerous_output.len();
    if debug {
        Anomaly::print_result(dangerous_output);
        print_message(
            format!(
                "Number of anomalies found: {}{anomalies_count}{}",
                Color::Red,
                Color::Reset
            )
            .as_str(),
            &LogLevel::Info,
        );
        print_message(
            format!(
                "Analyzed file: {}{csv_file_path}{}",
                Color::Red,
                Color::Reset
            )
            .as_str(),
            &LogLevel::Info,
        );
        print_message(
            format!(
                "Execution time: {}{:?}{}",
                Color::Blue,
                start_time.elapsed(),
                Color::Reset
            )
            .as_str(),
            &LogLevel::Info,
        );
    }
    log_message(
        format!("Anomalies: [{anomalies_count}] in file : {}, Execution time: [{:?}]", get_file_name(csv_file_path), start_time.elapsed()).as_str(), &LogLevel::Info,
    );
}

/// Execute post-execution tasks, such as deleting temporary files.
pub fn run_post_execution(file_path: &str) {
    if file_path.ends_with("_utf8.csv") {
        std::fs::remove_file(file_path).unwrap_or_else(|e| {
            log_and_print_message(
                format!("Error while deleting the temporary file {e}",).as_str(),
                LogLevel::Error,
            )
        });
    }
}

/// Extract the file name without the extension from a given file path.
pub fn get_file_name(file_path: &str) -> &str {
    file_path
        .rsplit(&['\\', '/'] as &[char])
        .next()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
}

/// Check if a file exists at the given path and log an error message if it does not.
pub fn file_exists(file_path: &str) -> bool {
    let exist: bool = std::path::Path::new(file_path).exists();
    if !exist {
        log_and_print_message(
            format!("File {file_path} does not exist").as_str(),
            LogLevel::Error,
        );
    }
    exist
}
