use crate::enums::color::Color;
use crate::enums::log_level::LogLevel;
use crate::structs::anomaly::Anomaly;
use crate::structs::json_output::JsonOutput;
use crate::structs::logger::{log_and_print_message, log_message, print_message};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::time::Instant;

/// Génère un fichier JSON contenant les anomalies détectées
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
                    "Une erreur est survenue lors de la création du dossier {}: {e}",
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
            format!("Une erreur est survenue lors de la sauvegarde du fichier JSON: {e}").as_str(),
            LogLevel::Error,
        );
        return;
    }

    if cfg!(debug_assertions) {
        print_message(
            format!(
                "Fichier JSON généré dans le dossier: {}{}/{JSON_DIR}{}",
                Color::Red,
                &current_dir,
                Color::Reset
            )
            .as_str(),
            &LogLevel::Info,
        );
    }
}

/// Récupère le chemin du fichier CSV à analyser depuis les arguments passés au programme
pub fn get_file_from_args(args: &[String]) -> Result<[String; 2], Error> {
    if args.len() != 3 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Il faut passer en argument le chemin du fichier CSV à analyser et le nom du fichier PDF en sortie",
        ));
    }

    if !file_exists(&args[1]) {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Le fichier {} n'existe pas", &args[1]),
        ));
    }

    if !args[1].ends_with(".csv") {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Le fichier {} doit être au format CSV", &args[1]),
        ));
    }

    if !args[2].ends_with(".json") {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Le fichier {} doit être au format JSON", &args[2]),
        ));
    }

    let csv_file_path: String = String::from(&args[1].replace("\\", "/"));
    let output_name: String = String::from(get_file_name(&args[2]));
    Ok([csv_file_path, output_name])
}

/// Affiche le rapport d'analyse dans la console et le fichier de log
pub fn print_report(
    start_time: &Instant,
    dangerous_output: &[Anomaly],
    csv_file_path: &str,
    debug: bool,
) {
    let nombre_anomalies: usize = dangerous_output.len();
    if debug {
        Anomaly::print_result(dangerous_output);
        print_message(
            format!(
                "Nombre d'anomalies: {}{nombre_anomalies}{}",
                Color::Red,
                Color::Reset
            )
            .as_str(),
            &LogLevel::Info,
        );
        print_message(
            format!(
                "Fichier CSV analysé: {}{csv_file_path}{}",
                Color::Red,
                Color::Reset
            )
            .as_str(),
            &LogLevel::Info,
        );
        print_message(
            format!(
                "Temps d'exécution: {}{:?}{}",
                Color::Blue,
                start_time.elapsed(),
                Color::Reset
            )
            .as_str(),
            &LogLevel::Info,
        );
    }
    log_message(
        format!("Nombre d'anomalies: [{nombre_anomalies}] dans le fichier : {}, Temps d'exécution: [{:?}]", get_file_name(csv_file_path), start_time.elapsed()).as_str(), &LogLevel::Info,
    );
}

/// Exécute les actions post-analyse (suppression des fichiers temporaires)
pub fn run_post_execution(file_path: &str) {
    if file_path.ends_with("_utf8.csv") {
        std::fs::remove_file(file_path).unwrap_or_else(|e| {
            log_and_print_message(
                format!(
                    "Une erreur est survenue lors de la suppression du fichier temporaire: {e}"
                )
                .as_str(),
                LogLevel::Error,
            )
        });
    }
}

/// Depuis un chemin de fichier, retourne le nom du fichier sans son extension.
pub fn get_file_name(file_path: &str) -> &str {
    file_path
        .rsplit(&['\\', '/'] as &[char])
        .next()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
}

/// Vérifie si un fichier existe à un chemin donné.
pub fn file_exists(file_path: &str) -> bool {
    let exist: bool = std::path::Path::new(file_path).exists();
    if !exist {
        log_and_print_message(
            format!("Le fichier {file_path} n'existe pas").as_str(),
            LogLevel::Error,
        );
    }
    exist
}
