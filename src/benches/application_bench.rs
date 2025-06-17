use criterion::{criterion_group, criterion_main, Criterion};
use datelint::enums::log_level::LogLevel;
use datelint::structs::csv_file::CsvFile;
use datelint::structs::logger::log_and_print_message;
use datelint::structs::perfage_model::PerfageModel;
use std::time::Duration;

fn test_analyse_file() {
    let filepath = r"";

    let perfage_iae: PerfageModel = match PerfageModel::from_config_file("config.json") {
        Ok(perfage) => perfage,
        Err(e) => {
            log_and_print_message(
                &format!("Error loading model configuration: {e}"),
                LogLevel::Error,
            );
            std::process::exit(1);
        }
    };

    // Structure de données représentant le fichier CSV à analyser
    let csv_struct: CsvFile = match CsvFile::from_file(filepath) {
        Ok(csv) => csv,
        Err(e) => {
            log_and_print_message(
                &format!("Error reading CSV file: {e}"),
                LogLevel::Error,
            );
            std::process::exit(1);
        }
    };

    // Récupération des anomalies détectées dans le fichier CSV
    perfage_iae.analyse_file(&csv_struct).unwrap_or_else(|e| {
        log_and_print_message(
            &format!("Error during file analysis: {e}"),
            LogLevel::Error,
        );
        return (vec![], 42, 42);
    });
}

fn benchmark_application(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark_application");

    group.sample_size(15);
    group.measurement_time(Duration::from_secs(15));
    group.warm_up_time(Duration::from_secs(10));

    group.bench_function("test_analyse_file", |b| {
        b.iter(|| {
            test_analyse_file();
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_application);
criterion_main!(benches);
