pub mod command;
pub mod command_history;
pub mod constants;
pub mod event_handlers;
pub mod services;
pub mod types;
pub mod utils;

// Re-export commonly used types for convenience
#[allow(unused_imports)]
pub use command::{Command, CommandResult};
#[allow(unused_imports)]
pub use command_history::CommandHistory;
pub use types::InsertPosition;
pub use utils::centered_popup_area;
