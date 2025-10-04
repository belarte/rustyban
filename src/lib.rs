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
pub use core::{Board, Card, Column, Result, RustybanError};
pub use domain::InsertPosition;
pub use engine::App;
pub use ui::AppRunner;
