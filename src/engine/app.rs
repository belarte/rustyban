use std::{borrow::Cow, cell::RefCell, rc::Rc};

use chrono::Local;

use crate::core::Board;
use crate::{core::Card, domain::{InsertPosition, event_handlers::AppOperations, services::{FileService, Logger, CardSelector}}};

/// The main application state and business logic coordinator.
///
/// `App` serves as the central hub for the rustyban application, managing the Kanban board
/// state, file operations, logging, and card selection. It coordinates between different
/// components using dependency injection for testability and flexibility.
///
/// The App is designed to be used with the [`AppRunner`] for the complete terminal UI experience,
/// but can also be used programmatically for testing or automation.
///
/// # Architecture
///
/// The App uses several injected dependencies:
/// - **FileService**: Handles board persistence to/from JSON files
/// - **Logger**: Manages application logging and user messages
/// - **CardSelector**: Tracks which card is currently selected in the UI
/// - **Board**: The core Kanban board data structure
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use rustyban::App;
///
/// // Create a new app with default dependencies
/// let app = App::new("my_board.json");
/// assert_eq!(app.file_name(), "my_board.json");
/// ```
///
/// ## Programmatic Board Manipulation
///
/// ```rust,no_run
/// use rustyban::App;
///
/// # fn main() {
/// let mut app = App::new("test.json");
/// 
/// // The App provides methods for board manipulation
/// // These are typically used by the UI layer
/// let file_name = app.file_name();
/// println!("Working with board: {}", file_name);
/// # }
/// ```
///
/// ## With Custom Dependencies (for testing)
///
/// ```rust,no_run
/// use rustyban::App;
/// 
/// // The App provides methods for dependency injection
/// // This is typically used for testing with mock implementations
/// let app = App::new("test.json");
/// 
/// // The app uses concrete implementations by default
/// // but can be configured with custom dependencies via the builder pattern
/// ```
///
/// [`AppRunner`]: crate::ui::AppRunner
#[derive(Debug)]
pub struct App {
    file_name: String,
    logger: Box<dyn Logger>,
    board: Rc<RefCell<Board>>,
    selector: Box<dyn CardSelector>,
    file_service: Box<dyn FileService>,
}


impl App {
    /// Private constructor for use by constructor modules
    pub(crate) fn from_parts(
        file_name: String,
        logger: Box<dyn Logger>,
        board: Rc<RefCell<Board>>,
        selector: Box<dyn CardSelector>,
        file_service: Box<dyn FileService>,
    ) -> Self {
        Self {
            file_name,
            logger,
            board,
            selector,
            file_service,
        }
    }

    /// Get the file name (for testing)
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Get the card selector (for testing)
    pub fn selector(&self) -> &dyn CardSelector {
        self.selector.as_ref()
    }

    /// Get the board (for widget rendering)
    pub(crate) fn board(&self) -> &Rc<RefCell<Board>> {
        &self.board
    }

    /// Get the logger (for widget rendering)
    pub(crate) fn logger(&self) -> &dyn Logger {
        self.logger.as_ref()
    }

    /// Get the selector (for operations)
    pub(crate) fn selector_mut(&mut self) -> &mut dyn CardSelector {
        self.selector.as_mut()
    }

    /// Get the file service (for operations)
    pub(crate) fn file_service(&self) -> &dyn FileService {
        self.file_service.as_ref()
    }

    /// Set the file name (for operations)
    #[allow(dead_code)] // Used in app_operations module
    pub(crate) fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }



    pub fn update_card(&mut self, card: Card) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .update_card(column_index, card_index, Cow::Borrowed(&card))
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
                .insert_card(column_index, card_index, Cow::Owned(Card::new("TODO", Local::now())))
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

    pub(crate) fn with_selected_card<F>(&mut self, mut action: F)
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

    pub(crate) fn card_selection<F>(&mut self, mut action: F)
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

    pub(crate) fn log(&mut self, msg: &str) {
        self.logger.log(msg);
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
        let app = App::with_file_service("res/dummy.json", crate::engine::file_service::ConcreteFileService::new());
        assert_eq!(app.file_name, "res/dummy.json");
    }

    #[test]
    fn test_app_with_mock_file_service() {
        // Test that App can be created with MockFileService
        let mock_service = crate::engine::mock_file_service::MockFileService::new()
            .with_load_result(Ok(crate::core::Board::new()));
        
        let app = App::with_file_service("res/dummy.json", mock_service);
        assert_eq!(app.file_name, "res/dummy.json");
    }

    #[test]
    fn test_app_write_with_mock_file_service() {
        // Test that App.write() uses the injected FileService
        let mock_service = crate::engine::mock_file_service::MockFileService::new()
            .with_save_result(Ok(()));
        
        let mut app = App::with_file_service("res/dummy.json", mock_service);
        
        // Add a card to the board
        app.insert_card(InsertPosition::Current);
        
        // Write should succeed (using mock)
        app.write();
        
        // Verify the file name is set correctly
        assert_eq!(app.file_name, "res/dummy.json");
    }

    #[test]
    fn test_app_write_with_mock_file_service_error() {
        // Test that App.write() handles FileService errors
        let mock_service = crate::engine::mock_file_service::MockFileService::new()
            .with_save_result(Err(crate::core::RustybanError::InvalidOperation { message: "Mock error".to_string() }));
        
        let mut app = App::with_file_service("res/dummy.json", mock_service);
        
        // Add a card to the board
        app.insert_card(InsertPosition::Current);
        
        // Write should handle the error gracefully
        app.write();
        
        // App should still be functional
        assert_eq!(app.file_name, "res/dummy.json");
    }

    #[test]
    fn test_app_with_mock_logger() {
        // Test that App can be created with MockLogger
        let mock_logger = crate::engine::mock_logger::MockLogger::new();
        let mock_file_service = crate::engine::mock_file_service::MockFileService::new();
        
        let app = App::with_dependencies("res/dummy.json", mock_file_service, mock_logger);
        assert_eq!(app.file_name, "res/dummy.json");
    }

    #[test]
    fn test_app_logging_with_mock_logger() {
        // Test that App.log() uses the injected Logger
        let mock_logger = crate::engine::mock_logger::MockLogger::new();
        let mock_file_service = crate::engine::mock_file_service::MockFileService::new();
        
        let mut app = App::with_dependencies("res/dummy.json", mock_file_service, mock_logger);
        
        // Log a message
        app.log("Test message");
        
        // Verify the message was logged (we need to access the logger)
        // Note: This test verifies that logging doesn't panic
        // In a more sophisticated architecture, we'd expose a way to verify logged messages
    }

    #[test]
    fn test_app_with_concrete_logger() {
        // Test that App can be created with ConcreteLoggerWrapper
        let concrete_logger = crate::engine::concrete_logger::ConcreteLoggerWrapper::new();
        let concrete_file_service = crate::engine::file_service::ConcreteFileService::new();
        
        let app = App::with_dependencies("res/dummy.json", concrete_file_service, concrete_logger);
        assert_eq!(app.file_name, "res/dummy.json");
    }

    #[test]
    fn test_app_with_mock_card_selector() {
        use crate::engine::mock_card_selector::MockCardSelector;
        use crate::engine::mock_file_service::MockFileService;
        use crate::engine::mock_logger::MockLogger;
        use crate::core::Card;
        
        let mock_card_selector = MockCardSelector::new()
            .with_selection(1, 2)
            .with_selected_card(Card::new("Test Card", chrono::Local::now()));
        let mock_file_service = MockFileService::new();
        let mock_logger = MockLogger::new();
        
        let app = App::with_all_dependencies("res/dummy.json", mock_file_service, mock_logger, mock_card_selector);
        
        // Verify the app was created successfully
        assert_eq!(app.file_name, "res/dummy.json");
        
        // Verify card selector functionality
        assert_eq!(app.selector.get(), Some((1, 2)));
        assert!(app.selector.get_selected_card().is_some());
    }

    #[test]
    fn test_app_card_selector_navigation() {
        use crate::engine::mock_card_selector::MockCardSelector;
        use crate::engine::mock_file_service::MockFileService;
        use crate::engine::mock_logger::MockLogger;
        
        let mock_card_selector = MockCardSelector::new().with_selection(0, 0);
        let mock_file_service = MockFileService::new();
        let mock_logger = MockLogger::new();
        
        let mut app = App::with_all_dependencies("res/dummy.json", mock_file_service, mock_logger, mock_card_selector);
        
        // Test navigation methods
        app.selector.select_next_column();
        app.selector.select_prev_column();
        app.selector.select_next_card();
        app.selector.select_prev_card();
        
        // Verify navigation calls were made
        if let Some(mock_selector) = app.selector.as_any().downcast_ref::<MockCardSelector>() {
            assert!(mock_selector.has_navigation_call("select_next_column"));
            assert!(mock_selector.has_navigation_call("select_prev_column"));
            assert!(mock_selector.has_navigation_call("select_next_card"));
            assert!(mock_selector.has_navigation_call("select_prev_card"));
        }
    }

    #[test]
    fn test_app_card_selector_selection_control() {
        use crate::engine::mock_card_selector::MockCardSelector;
        use crate::engine::mock_file_service::MockFileService;
        use crate::engine::mock_logger::MockLogger;
        
        let mock_card_selector = MockCardSelector::new().with_selection(1, 1);
        let mock_file_service = MockFileService::new();
        let mock_logger = MockLogger::new();
        
        let mut app = App::with_all_dependencies("res/dummy.json", mock_file_service, mock_logger, mock_card_selector);
        
        // Test selection control
        app.selector.set(2, 3);
        assert_eq!(app.selector.get(), Some((2, 3)));
        
        app.selector.disable_selection();
        assert_eq!(app.selector.get(), None);
    }
}


