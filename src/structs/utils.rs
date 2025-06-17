use crate::structs::anomalie::Anomalie;
use crate::structs::color;
use crate::structs::csv_file::CsvFile;
use std::time::Instant;

pub fn run_post_analysis(csv_struct: &CsvFile, dangerous_output: &[Anomalie]) {
    const TEMP_UTF8_FILE: &str = "utf8.csv";

    Anomalie::print_result(dangerous_output);
    Anomalie::generate_json_from_anomalies(dangerous_output, &csv_struct.csv_file_path);

    if csv_struct.csv_file_path == TEMP_UTF8_FILE {
        match std::fs::remove_file(TEMP_UTF8_FILE) {
            Ok(_) => (),
            Err(e) => eprintln!("Une erreur est survenue lors de la suppression du fichier: {e}"),
        }
    }
}

pub fn print_report(start_time: &Instant, dangerous_output: &[Anomalie]) {
    println!(
        "Nombre d'anomalies: {}{}{}",
        color::RED,
        dangerous_output.len(),
        color::RESET
    );
    println!(
        "Temps d'exécution: {}{:?}{}",
        color::BLUE,
        start_time.elapsed(),
        color::RESET
    );
}

/// Récupère le chemin du fichier CSV à analyser depuis les arguments passés au programme
/// # Returns
/// * `Result<String, std::io::Error>` - Le chemin du fichier CSV à analyser.
#[allow(dead_code)]
pub fn get_file_from_args(args: &[String]) -> Result<String, std::io::Error> {
    if args.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Il faut passer en argument le chemin du fichier CSV à analyser",
        ));
    }

    match args.get(1) {
        Some(path) => {
            if !std::path::Path::new(path).exists() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Le fichier spécifié n'existe pas",
                ));
            }
            Ok(path.to_string())
        }
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Il faut passer en argument le chemin du fichier CSV à analyser",
        )),
    }
}
