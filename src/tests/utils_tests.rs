use crate::tests::csv_tests::csv_utils::generate_csv_file;
use datelint::structs::anomalie::Anomalie;
use datelint::structs::json_output::JsonOutput;
use datelint::utils::util::{file_exists, generate_json_file, get_file_from_args, get_file_name};

#[tokio::test]
async fn test_get_file_from_args() {
    const FILE_NAME: &str = "test_get_file_from_args.csv";
    const TEST: &str = "test";
    const JSON_FILE: &str = r"\Test\Example.json";

    generate_csv_file(FILE_NAME, b',');

    let args_1: Vec<String> = vec![FILE_NAME.into()];
    let args_2: Vec<String> = vec![TEST.into(), FILE_NAME.into()];
    let args_3: Vec<String> = vec![TEST.into(), FILE_NAME.into(), JSON_FILE.into()];
    let args_4: Vec<String> = vec![TEST.into(), TEST.into(), TEST.into(), FILE_NAME.into()];
    let args_5: Vec<String> = vec![TEST.into(), JSON_FILE.into(), JSON_FILE.into()];

    assert!(get_file_from_args(&args_1).is_err());
    assert!(get_file_from_args(&args_2).is_err());
    assert!(get_file_from_args(&args_3).is_ok());
    assert!(get_file_from_args(&args_4).is_err());
    assert!(get_file_from_args(&args_5).is_err());

    let result: [String; 2] = get_file_from_args(&args_3).unwrap();
    assert_eq!(result[0], FILE_NAME);
    assert_eq!(result[1], "Example");

    delete_file(FILE_NAME);
}

#[tokio::test]
async fn test_get_file_name() {
    assert_eq!(get_file_name("C:\\Users\\test\\file.csv"), "file");
    assert_eq!(get_file_name("C:\\Users\\test\\file"), "file");
    assert_eq!(get_file_name("C:\\Users\\test\\file.txt"), "file");
    assert_eq!(get_file_name("C:\\Users\\test\\file.json"), "file");
    assert_eq!(get_file_name("C:\\Users\\test\\file.pdf"), "file");
    assert_eq!(get_file_name("C:\\Users\\test\\file.docx"), "file");
    assert_eq!(get_file_name("C:\\Users\\test\\file.doc"), "file");
    assert_eq!(get_file_name("Test/file.docx"), "file");
    assert_eq!(get_file_name("Test/file.doc"), "file");
    assert_eq!(get_file_name("Test/file"), "file");
    assert_eq!(get_file_name("Test/file.txt"), "file");
    assert_eq!(get_file_name("\\Test\\Test/file.txt"), "file");
    assert_eq!(get_file_name("\\Test\\Test/file"), "file");
    assert_eq!(get_file_name("\\Test\\Test/file.docx"), "file");
}

#[tokio::test]
async fn test_generate_json_file() {
    const ZERO_PROB: f32 = 0.0;
    const ZERO: u32 = 0;
    const JSON_FILE: &str = "test_generate_json_file";

    let json_response: JsonOutput = JsonOutput {
        analysed_file: String::from("test.csv"),
        ai_analyze: 15,
        regex_analyze: 10,
        time_ms : 100,
        anomalies: vec![
            Anomalie::new(
                String::from("Danger1"),
                String::from("Colonne1"),
                ZERO,
                ZERO_PROB,
            ),
            Anomalie::new(
                String::from("Danger2"),
                String::from("Colonne2"),
                ZERO,
                ZERO_PROB,
            ),
        ],
    };

    generate_json_file(json_response.anomalies.clone(), 0, 0, "test.csv", JSON_FILE, 100);

    assert!(std::path::Path::new("json/test_generate_json_file.json").exists());

    let content: JsonOutput = serde_json::from_str(
        &std::fs::read_to_string("json/test_generate_json_file.json").unwrap(),
    )
    .unwrap();

    assert_eq!(content.analysed_file, "test.csv");
    assert_eq!(content.anomalies.len(), 2);

    for (i, anomaly) in content.anomalies.iter().enumerate() {
        assert_eq!(anomaly.colonne, json_response.anomalies[i].colonne);
        assert_eq!(anomaly.valeur, json_response.anomalies[i].valeur);
        assert_eq!(anomaly.score, json_response.anomalies[i].score);
        assert_eq!(anomaly.ligne, json_response.anomalies[i].ligne);
    }

    delete_file("json/test_generate_json_file.json");
}

#[tokio::test]
async fn tets_file_exists() {
    const FILE_NAME: &str = "test_file_exists.csv";
    generate_csv_file(FILE_NAME, b',');
    assert!(file_exists(FILE_NAME));
    assert!(!file_exists("windows"));
    delete_file(FILE_NAME);
}

pub fn delete_file(file_name: &str) {
    if let Err(e) = std::fs::remove_file(file_name) {
        eprintln!("Une erreur est survenue lors de la suppression du fichier: {e}");
    }
}
