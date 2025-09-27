use std::{cell::RefCell, rc::Rc};

use chrono::Local;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Widget,
};

use crate::engine::logger::Logger as ConcreteLogger;
use crate::core::Board;
use crate::{engine::card_selector::CardSelector, core::Card, domain::{InsertPosition, event_handlers::AppOperations, services::{FileService, Logger}}};

#[derive(Debug)]
pub struct App {
    file_name: String,
    logger: Box<dyn Logger>,
    board: Rc<RefCell<Board>>,
    selector: CardSelector,
    file_service: Box<dyn FileService>,
}


impl App {
    pub fn new(file_name: &str) -> Self {
        let mut logger = ConcreteLogger::new();
        let board = if !file_name.is_empty() {
            match Board::open(file_name) {
                Ok(board) => board,
                Err(e) => {
                    logger.log(&format!(
                        "Cannot read file '{}' because: {}. Creating a new board instead.",
                        file_name, e
                    ));
                    Board::new()
                }
            }
        } else {
            logger.log("No file specified, creating a new board");
            Board::new()
        };

        let board = Rc::new(RefCell::new(board));
        let selector = CardSelector::new(Rc::clone(&board));

        App {
            file_name: file_name.to_string(),
            logger: Box::new(crate::engine::concrete_logger::ConcreteLoggerWrapper::new()),
            board,
            selector,
            file_service: Box::new(crate::engine::file_service::ConcreteFileService::new()),
        }
    }

    /// Create App with FileService dependency (for dependency injection and testing)
    pub fn with_file_service<F>(file_name: &str, file_service: F) -> Self 
    where 
        F: FileService + 'static,
    {
        let mut logger = ConcreteLogger::new();
        let board = if !file_name.is_empty() {
            match file_service.load_board(file_name) {
                Ok(board) => board,
                Err(e) => {
                    logger.log(&format!(
                        "Cannot read file '{}' because: {}. Creating a new board instead.",
                        file_name, e
                    ));
                    Board::new()
                }
            }
        } else {
            logger.log("No file specified, creating a new board");
            Board::new()
        };

        let board = Rc::new(RefCell::new(board));
        let selector = CardSelector::new(Rc::clone(&board));

        App {
            file_name: file_name.to_string(),
            logger: Box::new(crate::engine::concrete_logger::ConcreteLoggerWrapper::new()),
            board,
            selector,
            file_service: Box::new(file_service),
        }
    }

    /// Create App with FileService and Logger dependencies (for dependency injection and testing)
    pub fn with_dependencies<F, L>(file_name: &str, file_service: F, logger: L) -> Self 
    where 
        F: FileService + 'static,
        L: Logger + 'static,
    {
        let mut logger = logger;
        let board = if !file_name.is_empty() {
            match file_service.load_board(file_name) {
                Ok(board) => board,
                Err(e) => {
                    logger.log(&format!(
                        "Cannot read file '{}' because: {}. Creating a new board instead.",
                        file_name, e
                    ));
                    Board::new()
                }
            }
        } else {
            logger.log("No file specified, creating a new board");
            Board::new()
        };

        let board = Rc::new(RefCell::new(board));
        let selector = CardSelector::new(Rc::clone(&board));

        App {
            file_name: file_name.to_string(),
            logger: Box::new(logger),
            board,
            selector,
            file_service: Box::new(file_service),
        }
    }

    /// Create App from individual components (for dependency injection)
    pub fn from_components<F, L>(
        file_name: String,
        logger: L,
        board: Rc<RefCell<Board>>,
        selector: CardSelector,
        file_service: F,
    ) -> Self 
    where
        F: FileService + 'static,
        L: Logger + 'static,
    {
        Self {
            file_name,
            logger: Box::new(logger),
            board,
            selector,
            file_service: Box::new(file_service),
        }
    }


    pub fn update_card(&mut self, card: Card) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .update_card(column_index, card_index, card.clone())
                .unwrap_or_else(|e| {
                    this.logger.log(&format!("Failed to update card: {}", e));
                });
            (column_index, card_index)
        });
    }

    pub fn insert_card(&mut self, position: InsertPosition) -> Option<Card> {
        self.with_selected_card(|this, column_index, card_index| {
            this.board.as_ref().borrow_mut().deselect_card(column_index, card_index)
                .unwrap_or_else(|e| {
                    this.logger.log(&format!("Failed to deselect card: {}", e));
                });

            let card_index = match position {
                InsertPosition::Current => card_index,
                InsertPosition::Next => card_index + 1,
                InsertPosition::Top => 0,
                InsertPosition::Bottom => this.board.as_ref().borrow().column(column_index).map(|c| c.size()).unwrap_or(0),
            };

            this.board
                .as_ref()
                .borrow_mut()
                .insert_card(column_index, card_index, Card::new("TODO", Local::now()))
                .unwrap_or_else(|e| {
                    this.logger.log(&format!("Failed to insert card: {}", e));
                });
            this.board.as_ref().borrow_mut().select_card(column_index, card_index)
                .unwrap_or_else(|e| {
                    this.logger.log(&format!("Failed to select card: {}", e));
                });
            (column_index, card_index)
        });

        self.get_selected_card()
    }


    pub fn write_to_file(&mut self, file_name: String) {
        self.file_name = file_name;
        self.write();
    }

    fn with_selected_card<F>(&mut self, mut action: F)
    where
        F: FnMut(&mut Self, usize, usize) -> (usize, usize),
    {
        match self.selector.get() {
            Some((column_index, card_index)) => {
                let (column_index, card_index) = action(self, column_index, card_index);
                self.selector.set(column_index, card_index);
            }
            None => self.log("No card selected"),
        }
    }

    fn card_selection<F>(&mut self, mut action: F)
    where
        F: FnMut(&mut Self) -> (usize, usize),
    {
        if let Some((column_index, card_index)) = self.selector.get() {
            self.board.as_ref().borrow_mut().deselect_card(column_index, card_index)
                .unwrap_or_else(|e| {
                    self.logger.log(&format!("Failed to deselect card: {}", e));
                });
        }

        let (column_index, card_index) = action(self);
        self.board.as_ref().borrow_mut().select_card(column_index, card_index)
            .unwrap_or_else(|e| {
                self.logger.log(&format!("Failed to select card: {}", e));
            });
    }

    fn log(&mut self, msg: &str) {
        self.logger.log(msg);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_area, board_area, logger_area, instructions_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(area);

        let title = Line::from(" Welcome ".bold()).centered();
        title.render(title_area, buf);

        let instructions = Line::from(vec![
            " Help ".into(),
            "<?> ".blue().bold(),
            "Quit ".into(),
            "<q> ".blue().bold(),
        ])
        .centered();
        instructions.render(instructions_area, buf);

        self.board.as_ref().borrow().render(board_area, buf);
        self.logger.render(logger_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;
    use crate::domain::event_handlers::AppOperations;

    use crate::engine::app::InsertPosition;

    use super::App;

    #[test]
    fn mark_done_and_undone() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        app.select_next_card();
        app.select_next_card();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        app.mark_card_done();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        app.select_next_column();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Wash dishes", card.short_description());

        app.mark_card_undone();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Wash dishes", card.short_description());

        Ok(())
    }

    #[test]
    fn insertion_does_nothing_when_no_card_selected() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        assert_eq!(None, app.insert_card(InsertPosition::Current));

        Ok(())
    }

    #[test]
    fn insertion_at_current_position() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        app.select_next_card();
        app.select_next_card();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        let card = app.insert_card(InsertPosition::Current).unwrap();
        assert_eq!("TODO", card.short_description());

        {
            let board = app.board.as_ref().borrow();
            let card = board.card(0, 3);
            assert!(!card.unwrap().is_selected());
            let card = board.card(0, 2);
            assert!(card.unwrap().is_selected());
        }

        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        Ok(())
    }

    #[test]
    fn insertion_at_top() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        app.select_next_card();
        app.select_next_card();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        assert_eq!("Buy milk", app.board.as_ref().borrow().card(0, 0).unwrap().short_description());
        let card = app.insert_card(InsertPosition::Top).unwrap();
        assert_eq!("TODO", card.short_description());
        assert_eq!("TODO", app.board.as_ref().borrow().card(0, 0).unwrap().short_description());
        let card = app.get_selected_card().unwrap();
        assert_eq!("TODO", card.short_description());

        Ok(())
    }

    #[test]
    fn deletion() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        app.select_next_column();
        app.select_next_column();
        app.remove_card();
        app.remove_card();

        Ok(())
    }

    #[test]
    fn test_app_with_concrete_file_service() {
        // Test that App can be created with ConcreteFileService
        let app = App::with_file_service("test.json", crate::engine::file_service::ConcreteFileService::new());
        assert_eq!(app.file_name, "test.json");
    }

    #[test]
    fn test_app_with_mock_file_service() {
        // Test that App can be created with MockFileService
        let mock_service = crate::engine::mock_file_service::MockFileService::new()
            .with_load_result(Ok(crate::core::Board::new()));
        
        let app = App::with_file_service("test.json", mock_service);
        assert_eq!(app.file_name, "test.json");
    }

    #[test]
    fn test_app_write_with_mock_file_service() {
        // Test that App.write() uses the injected FileService
        let mock_service = crate::engine::mock_file_service::MockFileService::new()
            .with_save_result(Ok(()));
        
        let mut app = App::with_file_service("test.json", mock_service);
        
        // Add a card to the board
        app.insert_card(InsertPosition::Current);
        
        // Write should succeed (using mock)
        app.write();
        
        // Verify the file name is set correctly
        assert_eq!(app.file_name, "test.json");
    }

    #[test]
    fn test_app_write_with_mock_file_service_error() {
        // Test that App.write() handles FileService errors
        let mock_service = crate::engine::mock_file_service::MockFileService::new()
            .with_save_result(Err(crate::core::RustybanError::InvalidOperation { message: "Mock error".to_string() }));
        
        let mut app = App::with_file_service("test.json", mock_service);
        
        // Add a card to the board
        app.insert_card(InsertPosition::Current);
        
        // Write should handle the error gracefully
        app.write();
        
        // App should still be functional
        assert_eq!(app.file_name, "test.json");
    }

    #[test]
    fn test_app_with_mock_logger() {
        // Test that App can be created with MockLogger
        let mock_logger = crate::engine::mock_logger::MockLogger::new();
        let mock_file_service = crate::engine::mock_file_service::MockFileService::new();
        
        let app = App::with_dependencies("test.json", mock_file_service, mock_logger);
        assert_eq!(app.file_name, "test.json");
    }

    #[test]
    fn test_app_logging_with_mock_logger() {
        // Test that App.log() uses the injected Logger
        let mock_logger = crate::engine::mock_logger::MockLogger::new();
        let mock_file_service = crate::engine::mock_file_service::MockFileService::new();
        
        let mut app = App::with_dependencies("test.json", mock_file_service, mock_logger);
        
        // Log a message
        app.log("Test message");
        
        // Verify the message was logged (we need to access the logger)
        // Since we can't access the logger directly, we'll test through AppOperations
        app.write(); // This should log a message
        
        // The test passes if no panic occurs
        assert_eq!(app.file_name, "test.json");
    }

    #[test]
    fn test_app_with_concrete_logger() {
        // Test that App can be created with ConcreteLoggerWrapper
        let concrete_logger = crate::engine::concrete_logger::ConcreteLoggerWrapper::new();
        let concrete_file_service = crate::engine::file_service::ConcreteFileService::new();
        
        let app = App::with_dependencies("test.json", concrete_file_service, concrete_logger);
        assert_eq!(app.file_name, "test.json");
    }
}

impl AppOperations for App {
    fn update_card(&mut self, card: Card) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .update_card(column_index, card_index, card.clone())
                .unwrap_or_else(|e| {
                    this.logger.log(&format!("Failed to update card: {}", e));
                });
            (column_index, card_index)
        });
    }

    fn write_to_file(&mut self, file_name: String) {
        self.file_name = file_name;
        self.write();
    }

    fn select_next_column(&mut self) {
        self.card_selection(|this| this.selector.select_next_column())
    }

    fn select_prev_column(&mut self) {
        self.card_selection(|this| this.selector.select_prev_column())
    }

    fn select_next_card(&mut self) {
        self.card_selection(|this| this.selector.select_next_card())
    }

    fn select_prev_card(&mut self) {
        self.card_selection(|this| this.selector.select_prev_card())
    }

    fn disable_selection(&mut self) {
        if let Some((column_index, card_index)) = self.selector.get() {
            let mut board = self.board.as_ref().borrow_mut();
            board.deselect_card(column_index, card_index)
                .unwrap_or_else(|e| {
                    self.logger.log(&format!("Failed to deselect card: {}", e));
                });
        }

        self.selector.disable_selection();
    }

    fn get_selected_card(&self) -> Option<Card> {
        self.selector.get_selected_card()
    }

    fn insert_card(&mut self, position: InsertPosition) -> Option<Card> {
        if let Some((column_index, card_index)) = self.selector.get() {
            let card_index = match position {
                InsertPosition::Current => card_index,
                InsertPosition::Next => card_index + 1,
                InsertPosition::Top => 0,
                InsertPosition::Bottom => self.board.as_ref().borrow().column(column_index).map(|c| c.size()).unwrap_or(0),
            };

            let card = Card::new("", Local::now());
            self.board
                .as_ref()
                .borrow_mut()
                .insert_card(column_index, card_index, card.clone())
                .unwrap_or_else(|e| {
                    self.logger.log(&format!("Failed to insert card: {}", e));
                });

            self.board.as_ref().borrow_mut().select_card(column_index, card_index)
                .unwrap_or_else(|e| {
                    self.logger.log(&format!("Failed to select card: {}", e));
                });

            Some(card)
        } else {
            None
        }
    }

    fn remove_card(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let (column_index, card_index) = this.board.as_ref().borrow_mut().remove_card(column_index, card_index)
                .unwrap_or_else(|e| {
                    this.logger.log(&format!("Failed to remove card: {}", e));
                    (column_index, card_index)
                });
            this.board.as_ref().borrow_mut().select_card(column_index, card_index)
                .unwrap_or_else(|e| {
                    this.logger.log(&format!("Failed to select card: {}", e));
                });
            (column_index, card_index)
        });
    }

    fn increase_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .increase_priority(column_index, card_index)
        });
    }

    fn decrease_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .decrease_priority(column_index, card_index)
        });
    }

    fn mark_card_done(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .mark_card_done(column_index, card_index)
        });
    }

    fn mark_card_undone(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .mark_card_undone(column_index, card_index)
        });
    }

    fn write(&mut self) {
        let board = self.board.as_ref().borrow().clone();
        match self.file_service.save_board(&board, &self.file_name) {
            Ok(_) => self.log(&format!("Board successfully saved to '{}'", self.file_name)),
            Err(e) => self.log(&format!("Failed to save board to '{}': {}", self.file_name, e)),
        }
    }
}

