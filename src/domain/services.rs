use crate::core::{Board, Result};

/// Trait for file operations - enables dependency injection and testing
pub trait FileService: std::fmt::Debug {
    /// Load a board from a file
    fn load_board(&self, file_name: &str) -> Result<Board>;
    
    /// Save a board to a file
    fn save_board(&self, board: &Board, file_name: &str) -> Result<()>;
}