use thiserror::Error;

/// Main error type for the rustyban application.
///
/// This enum represents all possible errors that can occur when working with
/// rustyban boards, cards, and file operations. It uses the `thiserror` crate
/// for automatic error message formatting and conversion from standard library errors.
///
/// # Examples
///
/// ## Handling File Operations
///
/// ```rust,no_run
/// use rustyban::{Board, RustybanError};
///
/// match Board::open("nonexistent.json") {
///     Ok(board) => println!("Loaded board with {} columns", board.columns_count()),
///     Err(RustybanError::Io(io_err)) => {
///         println!("File error: {}", io_err);
///     }
///     Err(RustybanError::Serialization(json_err)) => {
///         println!("JSON parsing error: {}", json_err);
///     }
///     Err(e) => println!("Other error: {}", e),
/// }
/// ```
///
/// ## Handling Index Errors
///
/// ```rust
/// use rustyban::{Board, RustybanError};
///
/// let mut board = Board::new();
/// 
/// match board.remove_card(99, 0) {
///     Ok((col, idx)) => println!("Removed card at {}, {}", col, idx),
///     Err(RustybanError::IndexOutOfBounds { index, max }) => {
///         println!("Column {} is out of bounds (max: {})", index, max);
///     }
///     Err(e) => println!("Other error: {}", e),
/// }
/// ```
#[derive(Error, Debug)]
pub enum RustybanError {
    /// IO error occurred during file operations.
    ///
    /// This variant wraps `std::io::Error` and is automatically converted
    /// from IO errors using the `#[from]` attribute.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rustyban::Board;
    ///
    /// // This will return RustybanError::Io if the file doesn't exist
    /// let result = Board::open("missing_file.json");
    /// ```
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON serialization or deserialization error.
    ///
    /// This variant wraps `serde_json::Error` and occurs when loading
    /// or saving board files with invalid JSON format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rustyban::Board;
    ///
    /// // This will return RustybanError::Serialization if the JSON is malformed
    /// let result = Board::open("invalid.json");
    /// ```
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// Board operation failed with a descriptive message.
    ///
    /// Used for general board-level operations that fail due to invalid state
    /// or preconditions not being met.
    #[error("Board operation failed: {message}")]
    BoardOperation { message: String },
    
    /// Invalid file format encountered.
    ///
    /// This error occurs when a file exists and can be read, but doesn't
    /// contain the expected board structure.
    #[error("Invalid file format: {file_name}")]
    InvalidFileFormat { file_name: String },
    
    /// Card operation failed with a descriptive message.
    ///
    /// Used for card-specific operations that fail due to invalid state
    /// or preconditions not being met.
    #[error("Card operation failed: {message}")]
    CardOperation { message: String },
    
    /// Column operation failed with a descriptive message.
    ///
    /// Used for column-specific operations that fail due to invalid state
    /// or preconditions not being met.
    #[error("Column operation failed: {message}")]
    ColumnOperation { message: String },
    
    /// Index out of bounds error.
    ///
    /// This error occurs when trying to access a column or card with an
    /// invalid index. It provides both the attempted index and the maximum
    /// valid index for debugging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Board, RustybanError};
    ///
    /// let board = Board::new();
    /// 
    /// // Trying to access column 10 when only 3 exist (0, 1, 2)
    /// match board.column(10) {
    ///     Some(col) => println!("Found column: {}", col.header()),
    ///     None => println!("Column 10 doesn't exist"),
    /// }
    /// 
    /// // Operations that return Result will use IndexOutOfBounds error
    /// let mut board = Board::new();
    /// match board.remove_card(10, 0) {
    ///     Err(RustybanError::IndexOutOfBounds { index, max }) => {
    ///         assert_eq!(index, 10);
    ///         assert_eq!(max, 2); // Board has 3 columns (0, 1, 2)
    ///     }
    ///     _ => panic!("Expected IndexOutOfBounds error"),
    /// }
    /// ```
    #[error("Index out of bounds: index {index}, max {max}")]
    IndexOutOfBounds { index: usize, max: usize },
    
    /// Invalid operation attempted.
    ///
    /// Used for operations that are not allowed in the current state
    /// or with the given parameters.
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
}

/// Result type alias for rustyban operations.
///
/// This is a convenience type alias that uses [`RustybanError`] as the error type.
/// Most functions in the rustyban crate return this type for operations that can fail.
///
/// # Examples
///
/// ```rust
/// use rustyban::{Result, Board, Card};
/// use chrono::Local;
/// use std::borrow::Cow;
///
/// fn add_task_to_board(board: &mut Board, task: &str) -> Result<()> {
///     let card = Card::new(task, Local::now());
///     board.insert_card(0, 0, Cow::Owned(card))?;
///     Ok(())
/// }
/// 
/// # fn main() -> Result<()> {
/// let mut board = Board::new();
/// add_task_to_board(&mut board, "Complete documentation")?;
/// # Ok(())
/// # }
/// ```
pub type Result<T> = std::result::Result<T, RustybanError>;
