use crate::core::{Board, Result};

/// Trait for file operations - enables dependency injection and testing
pub trait FileService: std::fmt::Debug {
    /// Load a board from a file
    fn load_board(&self, file_name: &str) -> Result<Board>;
    
    /// Save a board to a file
    fn save_board(&self, board: &Board, file_name: &str) -> Result<()>;
}

use ratatui::{buffer::Buffer, layout::Rect};
use crate::core::Card;
use thiserror::Error;

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

/// Trait for card selection operations - enables dependency injection and testing
pub trait CardSelector: std::fmt::Debug {
    /// Get the current selection as (column_index, card_index)
    /// Returns None if selection is disabled
    fn get(&self) -> Option<(usize, usize)>;
    
    /// Set the selection to specific column and card indices
    fn set(&mut self, column_index: usize, card_index: usize);
    
    /// Get the currently selected card
    fn get_selected_card(&self) -> Option<Card>;
    
    /// Select the next column and return the new position
    fn select_next_column(&mut self) -> (usize, usize);
    
    /// Select the previous column and return the new position
    fn select_prev_column(&mut self) -> (usize, usize);
    
    /// Select the next card and return the new position
    fn select_next_card(&mut self) -> (usize, usize);
    
    /// Select the previous card and return the new position
    fn select_prev_card(&mut self) -> (usize, usize);
    
    /// Disable selection
    fn disable_selection(&mut self);
    
    /// Get a reference to the concrete type (for testing)
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Errors that can occur during AppBuilder construction
#[derive(Error, Debug)]
pub enum AppBuilderError {
    #[error("File name is required but not provided")]
    MissingFileName,
    
    #[error("Failed to load board from file '{file_name}': {error}")]
    BoardLoadError { file_name: String, error: String },
}