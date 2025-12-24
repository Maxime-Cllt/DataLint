#[cfg(test)]
mod benches;

#[cfg(test)]
mod tests;

use datalib::detection::anomaly::Anomaly;
use datalib::core::io::file::csv_file::CsvFile;
use datalib::model::InitializedModel;
use datalib::core::io::tracing::log_level::LogLevel;
use datalib::core::io::tracing::logger::log_and_print_message;
use datalib::core::utils::util::{
    generate_json_file, get_file_from_args, print_report, run_post_execution,
};
use std::process::exit;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: [String; 2] = get_file_from_args(&args)
        .expect("Error parsing command line arguments. Usage: datalib <csv_file> <output_file>");

    let start_time: Instant = Instant::now();

    // Load and initialize model once (much more efficient than Model::analyse_file)
    let mut model: InitializedModel = InitializedModel::new("config.json").unwrap_or_else(|e| {
        log_and_print_message(
            &format!("Error loading model: {e}"),
            &LogLevel::Error,
        );
        exit(1);
    });

    let csv_struct: CsvFile = CsvFile::from_file(&args[0]).unwrap_or_else(|e| {
        log_and_print_message(&format!("Error reading CSV file: {e}"), &LogLevel::Error);
        exit(1);
    });

    let (dangerous_output, ai_analyze, regex_analyze): (Vec<Anomaly>, u32, u32) =
        model.analyse_file(&csv_struct).unwrap_or_else(|e| {
            log_and_print_message(&format!("Error analyzing file: {e}"), &LogLevel::Error);
            exit(1);
        });

    print_report(
        &start_time,
        &dangerous_output,
        &csv_struct.csv_file_path,
        cfg!(debug_assertions),
    );

    generate_json_file(
        dangerous_output,
        regex_analyze,
        ai_analyze,
        &csv_struct.csv_file_path,
        &args[1],
        start_time.elapsed().as_millis(),
    );

    // Clean-up
    run_post_execution(&csv_struct.csv_file_path);
}
