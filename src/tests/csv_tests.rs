use crate::tests::csv_tests::csv_utils::generate_csv_file;
use crate::tests::utils_tests::delete_file;
use csv::StringRecord;
use datelint::structs::csv_file::CsvFile;

#[tokio::test]
async fn test_get_headers() {
    const FILE_NAME: &str = "test_get_headers.csv";

    // Virgule comme séparateur
    generate_csv_file(FILE_NAME, b',');
    let csv_file: CsvFile = CsvFile::new(FILE_NAME, b',');
    let headers: StringRecord = csv_file.get_headers().unwrap();
    assert_eq!(headers.len(), 3);
    assert_eq!(headers[0].to_string(), "Name");
    assert_eq!(headers[1].to_string(), "Age");
    assert_eq!(headers[2].to_string(), "City");
    delete_file(FILE_NAME);

    // Point-virgule comme séparateur
    generate_csv_file(FILE_NAME, b';');
    let csv_file: CsvFile = CsvFile::new(FILE_NAME, b';');
    let headers: StringRecord = csv_file.get_headers().unwrap();
    assert_eq!(headers.len(), 3);
    assert_eq!(headers[0].to_string(), "Name");
    assert_eq!(headers[1].to_string(), "Age");
    assert_eq!(headers[2].to_string(), "City");
    delete_file(FILE_NAME);

    // Tabulation comme séparateur
    generate_csv_file(FILE_NAME, b'\t');
    let csv_file: CsvFile = CsvFile::new(FILE_NAME, b'\t');
    let headers: StringRecord = csv_file.get_headers().unwrap();
    assert_eq!(headers.len(), 3);
    assert_eq!(headers[0].to_string(), "Name");
    assert_eq!(headers[1].to_string(), "Age");
    assert_eq!(headers[2].to_string(), "City");
    delete_file(FILE_NAME);

    // Pipe comme séparateur
    generate_csv_file(FILE_NAME, b'|');
    let csv_file: CsvFile = CsvFile::new(FILE_NAME, b'|');
    let headers: StringRecord = csv_file.get_headers().unwrap();
    assert_eq!(headers.len(), 3);
    assert_eq!(headers[0].to_string(), "Name");
    assert_eq!(headers[1].to_string(), "Age");
    assert_eq!(headers[2].to_string(), "City");
    delete_file(FILE_NAME);

    // Espace comme séparateur
    generate_csv_file(FILE_NAME, b' ');
    let csv_file: CsvFile = CsvFile::new(FILE_NAME, b' ');
    let headers: StringRecord = csv_file.get_headers().unwrap();
    assert_eq!(headers.len(), 3);
    assert_eq!(headers[0].to_string(), "Name");
    assert_eq!(headers[1].to_string(), "Age");
    assert_eq!(headers[2].to_string(), "City");
    delete_file(FILE_NAME);
}

#[tokio::test]
async fn test_read_first_line() {
    const FILE_NAME: &str = "test_read_first_line.csv";

    // Virgule comme séparateur
    generate_csv_file(FILE_NAME, b',');
    let first_line: String = CsvFile::read_first_line(FILE_NAME).unwrap();
    assert_eq!(first_line, "Name,Age,City");
    delete_file(FILE_NAME);

    // Point-virgule comme séparateur
    generate_csv_file(FILE_NAME, b';');
    let first_line: String = CsvFile::read_first_line(FILE_NAME).unwrap();
    assert_eq!(first_line, "Name;Age;City");
    delete_file(FILE_NAME);

    // Tabulation comme séparateur
    generate_csv_file(FILE_NAME, b'\t');
    let first_line: String = CsvFile::read_first_line(FILE_NAME).unwrap();
    assert_eq!(first_line, "Name\tAge\tCity");
    delete_file(FILE_NAME);

    // Pipe comme séparateur
    generate_csv_file(FILE_NAME, b'|');
    let first_line: String = CsvFile::read_first_line(FILE_NAME).unwrap();
    assert_eq!(first_line, "Name|Age|City");
    delete_file(FILE_NAME);

    // Espace comme séparateur
    generate_csv_file(FILE_NAME, b' ');
    let first_line: String = CsvFile::read_first_line(FILE_NAME).unwrap();
    assert_eq!(first_line, "Name Age City");
    delete_file(FILE_NAME);
}

#[tokio::test]
async fn test_csv_struct() {
    const FILE_NAME: &str = "test_csv_struct.csv";
    const SEPARATOR: u8 = b',';

    let csv_file: CsvFile = CsvFile::new(FILE_NAME, SEPARATOR);
    assert_eq!(csv_file.csv_file_path, FILE_NAME);
    assert_eq!(csv_file.separator, SEPARATOR);
}

#[cfg(test)]
pub mod csv_utils {
    use std::fs::File;
    use std::io::Write;

    pub fn generate_csv_file(file_name: &str, separator: u8) {
        let sep: char = separator as char;
        let mut file: File = File::create(file_name).unwrap();
        let mut content: String = format!("Name{sep}Age{sep}City\n");
        content.push_str(format!("John{sep}25{sep}New York\n").as_str());
        content.push_str(format!("Jane{sep}30{sep}Los Angeles\n").as_str());
        content.push_str(format!("Alice{sep}22{sep}Chicago\n").as_str());
        file.write_all(content.as_bytes()).unwrap();
    }
}
