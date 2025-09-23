mod app;
pub mod board; // Public because of documentation tests
mod error;
mod utils;

pub use app::AppRunner;
pub use error::{RustybanError, Result};
