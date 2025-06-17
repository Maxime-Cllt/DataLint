#[cfg(test)]
mod benches;

#[cfg(test)]
mod tests;

use datelint::enums::log_level::LogLevel;
use datelint::structs::anomalie::Anomalie;
use datelint::structs::csv_file::CsvFile;
use datelint::structs::logger::log_and_print_message;
use datelint::structs::perfage_model::PerfageModel;
use datelint::utils::util::{
    file_exists, generate_json_file, get_file_from_args, print_report, run_post_execution,
};
use std::process::exit;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: [String; 2] =
        get_file_from_args(&args).expect("Erreur lors de la récupération des arguments");

    // let filepath = r"C:\Users\HHBL8703\Downloads\reference.csv";

    let start_time: Instant = Instant::now();

    // Structure de données représentant le modèle Perfage
    let perfage_iae: PerfageModel =
        PerfageModel::from_config_file("config.json").unwrap_or_else(|e| {
            log_and_print_message(
                &format!("Une erreur est survenue lors de la récupération du modèle Perfage: {e}"),
                LogLevel::Error,
            );
            exit(1);
        });

    // Vérification de l'existence des fichiers de modèle et de vocabulaire
    [&perfage_iae.model_path, &perfage_iae.vocabulary_path]
        .iter()
        .for_each(|path| {
            if !file_exists(path) {
                exit(1);
            }
        });

    // Structure de données représentant le fichier CSV à analyser
    let csv_struct: CsvFile = CsvFile::from_file(&args[0]).unwrap_or_else(|e| {
        log_and_print_message(
            &format!("Une erreur est survenue lors de la récupération du fichier CSV: {e}"),
            LogLevel::Error,
        );
        exit(1);
    });

    // Récupération des anomalies détectées dans le fichier CSV
    let (dangerous_output, ai_analyze, regex_analyze): (Vec<Anomalie>, u32, u32) =
        perfage_iae.analyse_file(&csv_struct).unwrap_or_else(|e| {
            log_and_print_message(
                &format!("Une erreur est survenue lors de l'analyse du fichier CSV: {e}"),
                LogLevel::Error,
            );
            exit(1);
        });

    // let output = "file";

    // Affichage du rapport
    print_report(
        &start_time,
        &dangerous_output,
        &csv_struct.csv_file_path,
        if cfg!(debug_assertions) { true } else { false },
    );

    // Exécution de l'analyse
    generate_json_file(
        dangerous_output,
        regex_analyze,
        ai_analyze,
        &csv_struct.csv_file_path,
        &args[1],
        start_time.elapsed().as_millis() - 150,
    );

    // Clean-up
    run_post_execution(&csv_struct.csv_file_path);
}
