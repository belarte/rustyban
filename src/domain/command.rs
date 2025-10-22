use crate::core::{Board, Result};

/// Result of executing a command
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum CommandResult {
    /// Command executed successfully
    Success,
    /// Command executed but with a message
    SuccessWithMessage(String),
    /// Command failed to execute
    Failure(String),
}

/// Trait for reversible operations that can be executed and undone
#[allow(dead_code)]
pub trait Command {
    /// Execute the command
    fn execute(&mut self, board: &mut Board) -> Result<CommandResult>;
    
    /// Undo the command
    fn undo(&mut self, board: &mut Board) -> Result<CommandResult>;
    
    /// Get a description of what this command does
    fn description(&self) -> &str;
}

/// A simple test command for validation
#[allow(dead_code)]
pub struct TestCommand {
    executed: bool,
    description: String,
}

impl TestCommand {
    #[allow(dead_code)]
    pub fn new(description: &str) -> Self {
        Self {
            executed: false,
            description: description.to_string(),
        }
    }
}

impl Command for TestCommand {
    fn execute(&mut self, _board: &mut Board) -> Result<CommandResult> {
        self.executed = true;
        Ok(CommandResult::Success)
    }
    
    fn undo(&mut self, _board: &mut Board) -> Result<CommandResult> {
        self.executed = false;
        Ok(CommandResult::Success)
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Board;

    #[test]
    fn test_command_should_execute_successfully() {
        let mut board = Board::new();
        let mut command = TestCommand::new("Test operation");
        
        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(command.executed);
    }

    #[test]
    fn test_command_should_undo_successfully() {
        let mut board = Board::new();
        let mut command = TestCommand::new("Test operation");
        
        // Execute first
        command.execute(&mut board).unwrap();
        assert!(command.executed);
        
        // Then undo
        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(!command.executed);
    }

    #[test]
    fn test_command_should_provide_description() {
        let command = TestCommand::new("Test operation");
        assert_eq!(command.description(), "Test operation");
    }

    #[test]
    fn test_command_result_variants() {
        assert_eq!(CommandResult::Success, CommandResult::Success);
        assert_eq!(
            CommandResult::SuccessWithMessage("test".to_string()),
            CommandResult::SuccessWithMessage("test".to_string())
        );
        assert_eq!(
            CommandResult::Failure("error".to_string()),
            CommandResult::Failure("error".to_string())
        );
    }
}
