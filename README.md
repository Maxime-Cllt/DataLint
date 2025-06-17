<div align="center">
    <h1>📊 DataLint</h1>
    <p><em>High-performance CSV data validation and anomaly detection tool</em></p>
</div>

<div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/PyTorch-EE4C2C?style=for-the-badge&logo=pytorch&logoColor=white" alt="PyTorch" />
    <img src="https://img.shields.io/badge/Version-1.0.0-informational?style=for-the-badge" alt="Version" />
    <img src="https://img.shields.io/badge/License-GPL--3.0-blue?style=for-the-badge" alt="License" />
</div>

## 🚀 Overview

**DataLint** is a production-ready machine learning model designed for the **Perfage** application server. Built with
Rust for optimal performance, it provides powerful CSV file validation capabilities by detecting erroneous, malicious,
or anomalous data patterns using advanced AI techniques.

### ✨ Key Features

- 🔍 **AI-Powered Detection**: Leverages pre-trained neural networks for intelligent data anomaly detection
- ⚡ **High Performance**: Built with Rust for maximum speed and memory efficiency
- 📁 **CSV Processing**: Specialized for CSV file validation and analysis
- 🛡️ **Security Focus**: Identifies potentially dangerous or malicious data patterns
- 🔧 **Production Ready**: Optimized for server-side deployment in production environments
- 📊 **JSON Output**: Generates detailed analysis reports in JSON format

## 🎯 Use Cases

- **Data Quality Assurance**: Validate CSV imports before processing
- **Security Scanning**: Detect potentially malicious data injections
- **Data Pipeline Integration**: Automated validation in ETL processes
- **Compliance Checking**: Ensure data meets quality standards
- **Anomaly Detection**: Identify outliers and unusual patterns

## 📋 Prerequisites

### Required Tools

- **[Rust](https://www.rust-lang.org/tools/install)** (latest stable version)
- **[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)** (included with Rust)

### External Dependencies

- **AI Model**: Pre-trained PyTorch model for data anomaly detection
- **Tokenizer**: JSON-formatted vocabulary file for data indexing and tokenization
- **PyTorch Runtime**: Required DLLs and libraries for model inference

## 🛠️ Installation

### 1. Clone the Repository

```bash
git clone https://github.com/Maxime-Cllt/DataLint.git
cd DataLint
```

### 2. Build the Project

```bash
# Development build
cargo build

# Optimized release build (recommended for production)
cargo build --release
```

## ⚙️ Configuration

Create a `config.json` file in the same directory as the executable:

```json
{
  "model_path": "C:\\Users\\model\\neural\\perfage_ia",
  "vocabulary_path": "C:\\Users\\tokenizer\\tokenizer.json"
}
```

### Configuration Options

<table>
        <thead>
            <tr>
                <th>Option</th>
                <th>Description</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td><code>model_path</code></td>
                <td>Path to the pre-trained PyTorch model directory</td>
            </tr>
            <tr>
                <td><code>vocabulary_path</code></td>
                <td>Path to the tokenizer JSON file for data processing</td>
            </tr>
        </tbody>
</table>

## 🚀 Usage

### Command Line Interface

```bash
# Using cargo (development)
cargo run --release "input_file.csv" "output_report.json"

# Using compiled executable (production)
./target/release/DataLint "input_file.csv" "output_report.json"

# On Windows
.\target\release\DataLint.exe "input_file.csv" "output_report.json"
```

### Parameters

- **Input File**: Path to the CSV file to be validated
- **Output File**: Path where the JSON analysis report will be saved

### Example Usage

```bash
# Analyze a customer data file
./DataLint "data/customers.csv" "reports/customer_analysis.json"

# Validate uploaded user data
./DataLint "uploads/user_data.csv" "validation/results.json"
```

## 📊 Output Format

DataLint generates detailed JSON reports with the following structure:

```json
{
  "analysed_file": "file.csv",
  "ai_analyze": 1000,
  "regex_analyze": 1000,
  "time_ms": 1234,
  "anomalies": [
    {
      "value": "#ERROR!",
      "column": "\"Phone\"",
      "score": 0.9670525,
      "line": 71049
    },
    {
      "value": "??",
      "column": "\"Comment\"",
      "score": 0.90427655,
      "line": 75392
    }
  ]
}
```

## 🏗️ Dependencies Setup

### PyTorch Installation

1. **Install PyTorch**: Follow the [official installation guide](https://pytorch.org/get-started/locally/)
2. **Copy DLLs**: Place all PyTorch DLL files in the same directory as the DataLint executable

### Required PyTorch DLLs (Windows)

- `torch_cpu.dll`
- `torch_cuda.dll` (if using GPU)
- `c10.dll`
- `fbgemm.dll`
- Additional dependency DLLs as required

## 🔧 Development

### Building from Source

```bash
# Check code formatting
cargo fmt --check

# Run linting
cargo clippy

# Run tests
cargo test

# Build optimized release
cargo build --release
```

## 🧪 Testing

Run the test suite to ensure everything is functioning correctly:

```bash
cargo test
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- **Perfage**: The main application server that utilizes DataLint
- **PyTorch**: The underlying machine learning framework

## 📞 Support

For questions, issues, or feature requests, please:

- Open an issue on GitHub
- Contact the development team
- Check the documentation wiki

---

<p align="center">
  Made with 🦀
</p>