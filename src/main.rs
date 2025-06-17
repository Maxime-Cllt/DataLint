#[cfg(test)]
mod benches;

#[cfg(test)]
mod tests;

use datelint::enums::log_level::LogLevel;
use datelint::structs::anomaly::Anomaly;
use datelint::structs::csv_file::CsvFile;
use datelint::structs::logger::log_and_print_message;
use datelint::structs::model::Model;
use datelint::utils::util::{
    file_exists, generate_json_file, get_file_from_args, print_report, run_post_execution,
};
use std::process::exit;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: [String; 2] =
        get_file_from_args(&args).expect("Error parsing command line arguments. Usage: DateLint <csv_file> <output_file>");


    let start_time: Instant = Instant::now();

    let perfage_iae: Model =
        Model::from_config_file("config.json").unwrap_or_else(|e| {
            log_and_print_message(
                &format!("Error loading model configuration: {e}"),
                LogLevel::Error,
            );
            exit(1);
        });

    [&perfage_iae.model_path, &perfage_iae.vocabulary_path]
        .iter()
        .for_each(|path| {
            if !file_exists(path) {
                exit(1);
            }
        });

    let csv_struct: CsvFile = CsvFile::from_file(&args[0]).unwrap_or_else(|e| {
        log_and_print_message(
            &format!("Error reading CSV file: {e}"),
            LogLevel::Error,
        );
        exit(1);
    });

    let (dangerous_output, ai_analyze, regex_analyze): (Vec<Anomaly>, u32, u32) =
        perfage_iae.analyse_file(&csv_struct).unwrap_or_else(|e| {
            log_and_print_message(
                &format!("Error analyzing file: {e}"),
                LogLevel::Error,
            );
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
