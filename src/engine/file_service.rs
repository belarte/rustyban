use crate::core::{Board, Result};
use crate::domain::services::FileService;

/// Concrete implementation of FileService using real file operations
#[derive(Debug)]
pub struct ConcreteFileService;

impl ConcreteFileService {
    pub fn new() -> Self {
        Self
    }
}

impl FileService for ConcreteFileService {
    fn load_board(&self, file_name: &str) -> Result<Board> {
        Board::open(file_name)
    }

    fn save_board(&self, board: &Board, file_name: &str) -> Result<()> {
        board.to_file(file_name)
    }
}
