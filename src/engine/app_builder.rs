use std::{cell::RefCell, rc::Rc};
use crate::core::Board;
use crate::domain::services::{FileService, Logger, CardSelector, AppBuilderError};
use crate::engine::app::App;

/// Builder for constructing App instances with dependency injection
#[derive(Debug)]
pub struct AppBuilder {
    file_name: Option<String>,
    file_service: Option<Box<dyn FileService>>,
    logger: Option<Box<dyn Logger>>,
    card_selector: Option<Box<dyn CardSelector>>,
    board: Option<Rc<RefCell<Board>>>,
    fail_on_file_load_error: bool,
}

impl AppBuilder {
    /// Create a new AppBuilder instance
    pub fn new() -> Self {
        Self {
            file_name: None,
            file_service: None,
            logger: None,
            card_selector: None,
            board: None,
            fail_on_file_load_error: false, // Default: graceful fallback
        }
    }

    /// Set the file name (required)
    pub fn with_file_name(mut self, file_name: &str) -> Self {
        self.file_name = Some(file_name.to_string());
        self
    }

    /// Set the file service dependency
    pub fn with_file_service<F>(mut self, file_service: F) -> Self 
    where 
        F: FileService + 'static 
    {
        self.file_service = Some(Box::new(file_service));
        self
    }

    /// Set the logger dependency
    pub fn with_logger<L>(mut self, logger: L) -> Self 
    where 
        L: Logger + 'static 
    {
        self.logger = Some(Box::new(logger));
        self
    }

    /// Set the card selector dependency
    pub fn with_card_selector<C>(mut self, card_selector: C) -> Self 
    where 
        C: CardSelector + 'static 
    {
        self.card_selector = Some(Box::new(card_selector));
        self
    }

    /// Set the board directly (for testing)
    #[allow(dead_code)]
    pub fn with_board(mut self, board: Rc<RefCell<Board>>) -> Self {
        self.board = Some(board);
        self
    }
    
    /// Configure whether to fail on file load errors (default: false - graceful fallback)
    #[allow(dead_code)]
    pub fn fail_on_file_load_error(mut self, fail: bool) -> Self {
        self.fail_on_file_load_error = fail;
        self
    }

    /// Build the App instance
    pub fn build(mut self) -> Result<App, AppBuilderError> {
        // Validate required fields
        let file_name = self.file_name
            .ok_or(AppBuilderError::MissingFileName)?;

        // Create or use provided board
        let board = if let Some(board) = self.board {
            board
        } else {
            // Create board from file
            let board = if !file_name.is_empty() {
                // Use provided file service or default
                let file_service = self.file_service.take().unwrap_or_else(|| {
                    Box::new(crate::engine::file_service::ConcreteFileService::new())
                });
                
                match file_service.load_board(&file_name) {
                    Ok(board) => board,
                    Err(e) => {
                        if self.fail_on_file_load_error {
                            return Err(AppBuilderError::BoardLoadError {
                                file_name: file_name.clone(),
                                error: e.to_string(),
                            });
                        }
                        
                        // Log error and create new board (graceful fallback)
                        if let Some(logger) = &mut self.logger {
                            logger.log(&format!(
                                "Cannot read file '{}' because: {}. Creating a new board instead.",
                                file_name, e
                            ));
                        }
                        Board::new()
                    }
                }
            } else {
                // Log and create new board
                if let Some(logger) = &mut self.logger {
                    logger.log("No file specified, creating a new board");
                }
                Board::new()
            };
            
            Rc::new(RefCell::new(board))
        };

        // Create default dependencies if not provided
        let file_service = self.file_service.unwrap_or_else(|| {
            Box::new(crate::engine::file_service::ConcreteFileService::new())
        });

        let logger = self.logger.unwrap_or_else(|| {
            Box::new(crate::engine::concrete_logger::ConcreteLoggerWrapper::new())
        });

        let card_selector = self.card_selector.unwrap_or_else(|| {
            Box::new(crate::engine::card_selector::CardSelector::new(Rc::clone(&board)))
        });

        // Create App instance
        Ok(App::from_builder(
            file_name,
            logger,
            board,
            card_selector,
            file_service,
        ))
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::mock_file_service::MockFileService;
    use crate::engine::mock_logger::MockLogger;
    use crate::engine::mock_card_selector::MockCardSelector;
    use crate::core::Card;

    #[test]
    fn test_app_builder_with_defaults() {
        let app = AppBuilder::new()
            .with_file_name("res/dummy.json")
            .build()
            .expect("Failed to build App with defaults");
        
        assert_eq!(app.file_name(), "res/dummy.json");
    }

    #[test]
    fn test_app_builder_with_all_dependencies() {
        let mock_file_service = MockFileService::new();
        let mock_logger = MockLogger::new();
        let mock_card_selector = MockCardSelector::new()
            .with_selection(1, 2)
            .with_selected_card(Card::new("Test Card", chrono::Local::now()));

        let app = AppBuilder::new()
            .with_file_name("res/dummy.json")
            .with_file_service(mock_file_service)
            .with_logger(mock_logger)
            .with_card_selector(mock_card_selector)
            .build()
            .expect("Failed to build App with all dependencies");
        
        assert_eq!(app.file_name(), "res/dummy.json");
        assert_eq!(app.selector().get(), Some((1, 2)));
    }

    #[test]
    fn test_app_builder_with_partial_dependencies() {
        let mock_file_service = MockFileService::new();
        let mock_logger = MockLogger::new();

        let app = AppBuilder::new()
            .with_file_name("res/dummy.json")
            .with_file_service(mock_file_service)
            .with_logger(mock_logger)
            .build()
            .expect("Failed to build App with partial dependencies");
        
        assert_eq!(app.file_name(), "res/dummy.json");
    }

    #[test]
    fn test_app_builder_missing_file_name() {
        let result = AppBuilder::new()
            .build();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            AppBuilderError::MissingFileName => {},
            _ => panic!("Expected MissingFileName error"),
        }
    }

    #[test]
    fn test_app_builder_with_board() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mock_card_selector = MockCardSelector::new().with_selection(0, 0);

        let app = AppBuilder::new()
            .with_file_name("res/dummy.json")
            .with_board(Rc::clone(&board))
            .with_card_selector(mock_card_selector)
            .build()
            .expect("Failed to build App with board");
        
        assert_eq!(app.file_name(), "res/dummy.json");
        assert_eq!(app.selector().get(), Some((0, 0)));
    }

    #[test]
    fn test_app_builder_fluent_api() {
        // Test that the builder methods can be chained
        let app = AppBuilder::new()
            .with_file_name("res/dummy.json")
            .with_file_service(MockFileService::new())
            .with_logger(MockLogger::new())
            .with_card_selector(MockCardSelector::new())
            .build()
            .expect("Failed to build App with fluent API");
        
        assert_eq!(app.file_name(), "res/dummy.json");
    }

    #[test]
    fn test_app_builder_fail_on_file_load_error() {
        use crate::core::RustybanError;
        
        // Test graceful fallback (default behavior)
        let mock_file_service_graceful = MockFileService::new()
            .with_load_result(Err(RustybanError::InvalidOperation { 
                message: "File not found".to_string() 
            }));

        let app = AppBuilder::new()
            .with_file_name("nonexistent.json")
            .with_file_service(mock_file_service_graceful)
            .build()
            .expect("Should succeed with graceful fallback");
        
        assert_eq!(app.file_name(), "nonexistent.json");

        // Test fail on error
        let mock_file_service_fail = MockFileService::new()
            .with_load_result(Err(RustybanError::InvalidOperation { 
                message: "File not found".to_string() 
            }));

        let result = AppBuilder::new()
            .with_file_name("nonexistent.json")
            .with_file_service(mock_file_service_fail)
            .fail_on_file_load_error(true)
            .build();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            AppBuilderError::BoardLoadError { file_name, error } => {
                assert_eq!(file_name, "nonexistent.json");
                assert!(error.contains("File not found"));
            },
            _ => panic!("Expected BoardLoadError"),
        }
    }
}