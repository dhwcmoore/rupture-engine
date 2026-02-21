use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuptureError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV parse error: {0}")]
    CsvParse(#[from] csv::Error),

    #[error("JSON serialisation error: {0}")]
    JsonSerialise(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Empty data: {0}")]
    EmptyData(String),

    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
}

pub type Result<T> = std::result::Result<T, RuptureError>;
