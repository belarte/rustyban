//! # Rustyban - Terminal Kanban Board
//!
//! Rustyban is a terminal-based Kanban board application built with Rust and Ratatui.
//! It provides an intuitive interface for managing tasks and projects using the Kanban methodology.
//!
//! ## Features
//!
//! - **Terminal UI**: Clean, responsive terminal interface using Ratatui
//! - **Kanban Workflow**: Traditional three-column layout (To Do, In Progress, Done)
//! - **Card Management**: Create, edit, move, and delete cards
//! - **Persistence**: Save and load boards from JSON files
//! - **Keyboard Navigation**: Efficient keyboard-driven interface
//!
//! ## Quick Start
//!
//! ### Basic Usage
//!
//! ```rust,no_run
//! use rustyban::AppRunner;
//! use ratatui::init;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize terminal
//! let mut terminal = init();
//!
//! // Create and run the application
//! let mut app_runner = AppRunner::new("my_board.json");
//! app_runner.run(&mut terminal)?;
//!
//! // Restore terminal
//! ratatui::restore();
//! # Ok(())
//! # }
//! ```
//!
//! ### Working with Boards Programmatically
//!
//! ```rust
//! use rustyban::{Board, Card};
//! use chrono::Local;
//! use std::borrow::Cow;
//!
//! # fn main() -> rustyban::Result<()> {
//! // Create a new board
//! let mut board = Board::new();
//!
//! // Create cards
//! let now = Local::now();
//! let card1 = Card::new("Implement user authentication", now);
//! let card2 = Card::new("Write documentation", now);
//! let card3 = Card::new("Add unit tests", now);
//!
//! // Add cards to the "To Do" column (index 0)
//! board.insert_card(0, 0, Cow::Owned(card1))?;
//! board.insert_card(0, 0, Cow::Owned(card2))?;
//! board.insert_card(0, 0, Cow::Owned(card3))?;
//!
//! // Get a card to move to "In Progress" 
//! if let Some(card) = board.card(0, 0) {
//!     let card_clone = card.clone();
//!     board.remove_card(0, 0)?;
//!     board.insert_card(1, 0, Cow::Owned(card_clone))?;
//! }
//!
//! // Mark a card as done (returns new position)
//! let (new_col, new_idx) = board.mark_card_done(1, 0);
//!
//! // Save the board
//! board.to_file("my_project.json")?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - **Core Types**: [`Board`], [`Card`], [`Column`] - The fundamental data structures
//! - **Application**: [`App`], [`AppRunner`] - The main application logic and UI runner
//! - **Domain Types**: [`InsertPosition`] - Domain-specific enums and utilities
//! - **Error Handling**: [`RustybanError`], [`Result`] - Comprehensive error types
//!
//! ## Error Handling
//!
//! All fallible operations return a [`Result<T>`] where the error type is [`RustybanError`].
//! This provides detailed information about what went wrong and includes context for debugging.
//!
//! ```rust
//! use rustyban::{Board, RustybanError};
//!
//! # fn example() -> rustyban::Result<()> {
//! let mut board = Board::new();
//!
//! // This will return an error if the column index is invalid
//! match board.remove_card(99, 0) {
//!     Ok((col_idx, card_idx)) => println!("Removed card at column {}, position {}", col_idx, card_idx),
//!     Err(RustybanError::IndexOutOfBounds { index, max }) => {
//!         println!("Column index {} is out of bounds (max: {})", index, max);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```

// Domain types, constants, and traits
mod domain;

// Core data structures and business logic
mod core;

// UI components and interface
mod ui;

// Business logic and operations
mod engine;

// Utilities
pub(crate) mod utils;

// Public API - what users need
pub use core::{Board, Card, Column, RustybanError, Result};
pub use domain::InsertPosition;
pub use engine::App;
pub use ui::AppRunner;
