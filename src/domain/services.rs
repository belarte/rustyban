use crate::core::{Board, Result};

/// Trait for file operations - enables dependency injection and testing
pub trait FileService: std::fmt::Debug {
    /// Load a board from a file
    fn load_board(&self, file_name: &str) -> Result<Board>;
    
    /// Save a board to a file
    fn save_board(&self, board: &Board, file_name: &str) -> Result<()>;
}

use ratatui::{buffer::Buffer, layout::Rect};

/// Trait for logging operations - enables dependency injection and testing
/// Note: This trait includes UI rendering concerns for simplicity.
/// In a more complex system, you might want to separate logging from rendering.
pub trait Logger: std::fmt::Debug {
    /// Log a message
    fn log(&mut self, message: &str);
    
    /// Render the logger to the terminal
    /// Does nothing if the logger doesn't support rendering
    fn render(&self, area: Rect, buf: &mut Buffer);
}