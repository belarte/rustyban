use crate::core::{Board, Result};
use crate::domain::services::FileService;

/// Mock implementation of FileService for testing
#[derive(Debug)]
pub struct MockFileService {
    pub load_result: Option<Result<Board>>,
    pub save_result: Option<Result<()>>,
}

impl MockFileService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            load_result: None,
            save_result: None,
        }
    }

    /// Configure the load result for testing
    #[allow(dead_code)]
    pub fn with_load_result(mut self, result: Result<Board>) -> Self {
        self.load_result = Some(result);
        self
    }

    /// Configure the save result for testing
    #[allow(dead_code)]
    pub fn with_save_result(mut self, result: Result<()>) -> Self {
        self.save_result = Some(result);
        self
    }
}

impl FileService for MockFileService {
    fn load_board(&self, _file_name: &str) -> Result<Board> {
        match &self.load_result {
            Some(result) => match result {
                Ok(board) => Ok(board.clone()),
                Err(e) => Err(crate::core::RustybanError::InvalidOperation {
                    message: format!("Mock load error: {}", e),
                }),
            },
            None => Ok(Board::new()),
        }
    }

    fn save_board(&self, _board: &Board, _file_name: &str) -> Result<()> {
        match &self.save_result {
            Some(result) => match result {
                Ok(()) => Ok(()),
                Err(e) => Err(crate::core::RustybanError::InvalidOperation {
                    message: format!("Mock save error: {}", e),
                }),
            },
            None => Ok(()),
        }
    }
}
