use thiserror::Error;

/// Main error type for the rustyban application
#[derive(Error, Debug)]
pub enum RustybanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Board operation failed: {message}")]
    BoardOperation { message: String },
    
    #[error("Invalid file format: {file_name}")]
    InvalidFileFormat { file_name: String },
    
    #[error("Card operation failed: {message}")]
    CardOperation { message: String },
    
    #[error("Column operation failed: {message}")]
    ColumnOperation { message: String },
    
    #[error("Index out of bounds: index {index}, max {max}")]
    IndexOutOfBounds { index: usize, max: usize },
    
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
}

/// Result type alias for rustyban operations
pub type Result<T> = std::result::Result<T, RustybanError>;
