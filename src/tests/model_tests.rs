use datelint::structs::perfage_model::PerfageModel;
use std::fs::File;
use std::io::Write;

#[tokio::test]
async fn test_from_config_file() {
    const CONFIG_JSON: &str = r#"
    {
        "model_path": "model.pb",
        "vocabulary_path": "vectorizer_data.json",
        "max_features": 1000
    }
    "#;
    const CONFIG_JSON_PATH: &str = "test_from_config_file.json";

    let mut file: File = File::create(CONFIG_JSON_PATH).unwrap();
    file.write_all(CONFIG_JSON.as_bytes()).unwrap();

    let model: PerfageModel = PerfageModel::from_config_file(CONFIG_JSON_PATH).unwrap();
    assert_eq!(model.model_path, "model.pb");
    assert_eq!(model.vocabulary_path, "vectorizer_data.json");

    if let Err(e) = std::fs::remove_file(CONFIG_JSON_PATH) {
        eprintln!("Une erreur est survenue lors de la suppression du fichier: {e}");
    }
}
