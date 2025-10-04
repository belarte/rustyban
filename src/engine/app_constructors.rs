use std::{cell::RefCell, rc::Rc};

use crate::core::Board;
use crate::domain::services::{CardSelector, FileService, Logger};

use super::App;

impl App {
    pub fn new(file_name: &str) -> Self {
        // Use the builder pattern internally for consistency
        crate::engine::app_builder::AppBuilder::new()
            .with_file_name(file_name)
            .build()
            .expect("Failed to create App with default dependencies")
    }

    /// Private constructor for use by AppBuilder
    pub(crate) fn from_builder(
        file_name: String,
        logger: Box<dyn Logger>,
        board: Rc<RefCell<Board>>,
        selector: Box<dyn CardSelector>,
        file_service: Box<dyn FileService>,
    ) -> Self {
        Self::from_parts(file_name, logger, board, selector, file_service)
    }

    /// Create App with FileService dependency (for dependency injection and testing)
    pub fn with_file_service<F>(file_name: &str, file_service: F) -> Self
    where
        F: FileService + 'static,
    {
        crate::engine::app_builder::AppBuilder::new()
            .with_file_name(file_name)
            .with_file_service(file_service)
            .build()
            .expect("Failed to create App with FileService dependency")
    }

    /// Create App with FileService and Logger dependencies (for dependency injection and testing)
    pub fn with_dependencies<F, L>(file_name: &str, file_service: F, logger: L) -> Self
    where
        F: FileService + 'static,
        L: Logger + 'static,
    {
        crate::engine::app_builder::AppBuilder::new()
            .with_file_name(file_name)
            .with_file_service(file_service)
            .with_logger(logger)
            .build()
            .expect("Failed to create App with FileService and Logger dependencies")
    }

    /// Create App with FileService, Logger, and CardSelector dependencies
    pub fn with_all_dependencies<F, L, C>(file_name: &str, file_service: F, logger: L, card_selector: C) -> Self
    where
        F: FileService + 'static,
        L: Logger + 'static,
        C: CardSelector + 'static,
    {
        crate::engine::app_builder::AppBuilder::new()
            .with_file_name(file_name)
            .with_file_service(file_service)
            .with_logger(logger)
            .with_card_selector(card_selector)
            .build()
            .expect("Failed to create App with all dependencies")
    }

    /// Create App from individual components (for dependency injection)
    pub fn from_components<F, L, C>(
        file_name: String,
        logger: L,
        board: Rc<RefCell<Board>>,
        selector: C,
        file_service: F,
    ) -> Self
    where
        F: FileService + 'static,
        L: Logger + 'static,
        C: CardSelector + 'static,
    {
        Self::from_parts(
            file_name,
            Box::new(logger),
            board,
            Box::new(selector),
            Box::new(file_service),
        )
    }
}
