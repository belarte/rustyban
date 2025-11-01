use crate::core::{Board, Result};
use crate::domain::command::{Command, CommandResult};
use std::collections::VecDeque;

/// Maximum number of commands to keep in undo history
#[allow(dead_code)]
const MAX_UNDO_HISTORY: usize = 50;

/// Manages command history for undo/redo functionality
#[allow(dead_code)]
pub struct CommandHistory {
    /// Stack of executed commands for undo
    undo_stack: VecDeque<Box<dyn Command>>,
    /// Stack of undone commands for redo
    redo_stack: VecDeque<Box<dyn Command>>,
    /// Maximum number of commands to keep in undo history
    max_history: usize,
}

impl std::fmt::Debug for CommandHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandHistory")
            .field("undo_count", &self.undo_stack.len())
            .field("redo_count", &self.redo_stack.len())
            .field("max_history", &self.max_history)
            .finish()
    }
}

impl CommandHistory {
    /// Create a new command history with default settings
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::with_max_history(MAX_UNDO_HISTORY)
    }

    /// Create a new command history with custom max history size
    #[allow(dead_code)]
    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history,
        }
    }

    /// Execute a command and add it to the undo stack
    #[allow(dead_code)]
    pub fn execute_command(&mut self, mut command: Box<dyn Command>, board: &mut Board) -> Result<CommandResult> {
        self.redo_stack.clear();

        let result = command.execute(board)?;

        if matches!(result, CommandResult::Success | CommandResult::SuccessWithMessage(_)) {
            if self.undo_stack.len() >= self.max_history {
                self.undo_stack.pop_front();
            }
            self.undo_stack.push_back(command);
        }

        Ok(result)
    }

    /// Undo the last executed command
    #[allow(dead_code)]
    pub fn undo(&mut self, board: &mut Board) -> Result<CommandResult> {
        if let Some(mut command) = self.undo_stack.pop_back() {
            let result = command.undo(board)?;
            if matches!(result, CommandResult::Success | CommandResult::SuccessWithMessage(_)) {
                self.redo_stack.push_back(command);
            } else {
                self.undo_stack.push_back(command);
            }
            Ok(result)
        } else {
            Ok(CommandResult::Failure("Nothing to undo".to_string()))
        }
    }

    /// Redo the last undone command
    #[allow(dead_code)]
    pub fn redo(&mut self, board: &mut Board) -> Result<CommandResult> {
        if let Some(mut command) = self.redo_stack.pop_back() {
            let result = command.execute(board)?;
            if matches!(result, CommandResult::Success | CommandResult::SuccessWithMessage(_)) {
                if self.undo_stack.len() >= self.max_history {
                    self.undo_stack.pop_front();
                }
                self.undo_stack.push_back(command);
            } else {
                self.redo_stack.push_back(command);
            }
            Ok(result)
        } else {
            Ok(CommandResult::Failure("Nothing to redo".to_string()))
        }
    }

    /// Check if undo is available
    #[allow(dead_code)]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    #[allow(dead_code)]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of commands in undo stack
    #[allow(dead_code)]
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of commands in redo stack
    #[allow(dead_code)]
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all command history
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the description of the last command that can be undone
    #[allow(dead_code)]
    pub fn last_undo_description(&self) -> Option<&str> {
        self.undo_stack.back().map(|cmd| cmd.description())
    }

    /// Get the description of the last command that can be redone
    #[allow(dead_code)]
    pub fn last_redo_description(&self) -> Option<&str> {
        self.redo_stack.back().map(|cmd| cmd.description())
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Board;
    use crate::domain::command::TestCommand;

    #[test]
    fn test_command_history_new() {
        let history = CommandHistory::new();
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_command_history_with_custom_max_history() {
        let history = CommandHistory::with_max_history(10);
        assert_eq!(history.max_history, 10);
    }

    #[test]
    fn test_execute_command_success() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();
        let command = Box::new(TestCommand::new("Test command"));

        let result = history.execute_command(command, &mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert_eq!(history.undo_count(), 1);
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_undo_success() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();
        let command = Box::new(TestCommand::new("Test command"));

        history.execute_command(command, &mut board).unwrap();
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);

        let result = history.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 1);
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_redo_success() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();
        let command = Box::new(TestCommand::new("Test command"));

        history.execute_command(command, &mut board).unwrap();
        history.undo(&mut board).unwrap();

        let result = history.redo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert_eq!(history.undo_count(), 1);
        assert_eq!(history.redo_count(), 0);
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_undo_when_empty() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();

        let result = history.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Failure("Nothing to undo".to_string()));
    }

    #[test]
    fn test_redo_when_empty() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();

        let result = history.redo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Failure("Nothing to redo".to_string()));
    }

    #[test]
    fn test_clear_redo_stack_on_new_command() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();

        let command1 = Box::new(TestCommand::new("Command 1"));
        history.execute_command(command1, &mut board).unwrap();
        history.undo(&mut board).unwrap();
        assert_eq!(history.redo_count(), 1);

        let command2 = Box::new(TestCommand::new("Command 2"));
        history.execute_command(command2, &mut board).unwrap();
        assert_eq!(history.redo_count(), 0);
        assert_eq!(history.undo_count(), 1);
    }

    #[test]
    fn test_max_history_limit() {
        let mut history = CommandHistory::with_max_history(2);
        let mut board = Board::new();

        for i in 0..3 {
            let command = Box::new(TestCommand::new(&format!("Command {}", i)));
            history.execute_command(command, &mut board).unwrap();
        }

        assert_eq!(history.undo_count(), 2);
        assert!(history.can_undo());
    }

    #[test]
    fn test_clear_all_history() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();

        let command = Box::new(TestCommand::new("Test command"));
        history.execute_command(command, &mut board).unwrap();
        history.undo(&mut board).unwrap();

        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 1);

        history.clear();
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_last_undo_description() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();

        assert_eq!(history.last_undo_description(), None);

        let command = Box::new(TestCommand::new("Test command"));
        history.execute_command(command, &mut board).unwrap();
        assert_eq!(history.last_undo_description(), Some("Test command"));

        history.undo(&mut board).unwrap();
        assert_eq!(history.last_undo_description(), None);
    }

    #[test]
    fn test_last_redo_description() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();

        assert_eq!(history.last_redo_description(), None);

        let command = Box::new(TestCommand::new("Test command"));
        history.execute_command(command, &mut board).unwrap();
        history.undo(&mut board).unwrap();
        assert_eq!(history.last_redo_description(), Some("Test command"));

        history.redo(&mut board).unwrap();
        assert_eq!(history.last_redo_description(), None);
    }

    #[test]
    fn test_multiple_undo_redo_chain() {
        let mut history = CommandHistory::new();
        let mut board = Board::new();

        for i in 0..3 {
            let command = Box::new(TestCommand::new(&format!("Command {}", i)));
            history.execute_command(command, &mut board).unwrap();
        }

        assert_eq!(history.undo_count(), 3);
        assert_eq!(history.redo_count(), 0);

        for i in (0..3).rev() {
            let result = history.undo(&mut board).unwrap();
            assert_eq!(result, CommandResult::Success);
            assert_eq!(history.undo_count(), i);
            assert_eq!(history.redo_count(), 3 - i);
        }

        for i in 0..3 {
            let result = history.redo(&mut board).unwrap();
            assert_eq!(result, CommandResult::Success);
            assert_eq!(history.undo_count(), i + 1);
            assert_eq!(history.redo_count(), 2 - i);
        }
    }
}
