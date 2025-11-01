use super::{check_already_executed, check_not_executed, validate_card_exists, validate_card_exists_for_undo};
use crate::core::{Board, Result};
use crate::domain::command::{Command, CommandResult};

/// Command for marking a card as done or undone
#[allow(dead_code)]
pub struct MarkCardCommand {
    column_index: usize,
    card_index: usize,
    mark_done: bool,
    original_column_index: Option<usize>,
    original_card_index: Option<usize>,
    executed: bool,
}

impl MarkCardCommand {
    /// Create a new mark card as done command
    #[allow(dead_code)]
    pub fn mark_done(column_index: usize, card_index: usize) -> Self {
        Self {
            column_index,
            card_index,
            mark_done: true,
            original_column_index: None,
            original_card_index: None,
            executed: false,
        }
    }

    /// Create a new mark card as undone command
    #[allow(dead_code)]
    pub fn mark_undone(column_index: usize, card_index: usize) -> Self {
        Self {
            column_index,
            card_index,
            mark_done: false,
            original_column_index: None,
            original_card_index: None,
            executed: false,
        }
    }
}

impl Command for MarkCardCommand {
    fn execute(&mut self, board: &mut Board) -> Result<CommandResult> {
        if let Some(result) = check_already_executed(self.executed) {
            return Ok(result);
        }

        if let Ok(CommandResult::Failure(msg)) = validate_card_exists(board, self.column_index, self.card_index) {
            return Ok(CommandResult::Failure(msg));
        }

        self.original_column_index = Some(self.column_index);
        self.original_card_index = Some(self.card_index);

        let (new_column, new_card_index) = if self.mark_done {
            board.mark_card_done(self.column_index, self.card_index)
        } else {
            board.mark_card_undone(self.column_index, self.card_index)
        };

        if new_column == self.column_index && new_card_index == self.card_index {
            return Ok(CommandResult::Failure(
                "Cannot mark card done/undone at column boundary".to_string(),
            ));
        }

        self.column_index = new_column;
        self.card_index = new_card_index;
        self.executed = true;
        Ok(CommandResult::Success)
    }

    fn undo(&mut self, board: &mut Board) -> Result<CommandResult> {
        if let Some(result) = check_not_executed(self.executed) {
            return Ok(result);
        }

        let original_column = match self.original_column_index {
            Some(col) => col,
            None => {
                return Ok(CommandResult::Failure(
                    "Original column index not available for undo".to_string(),
                ));
            }
        };

        let original_card = match self.original_card_index {
            Some(card) => card,
            None => {
                return Ok(CommandResult::Failure(
                    "Original card index not available for undo".to_string(),
                ));
            }
        };

        if let Ok(CommandResult::Failure(msg)) =
            validate_card_exists_for_undo(board, self.column_index, self.card_index)
        {
            return Ok(CommandResult::Failure(msg));
        }

        let (new_column, new_card_index) = if self.mark_done {
            board.mark_card_undone(self.column_index, self.card_index)
        } else {
            board.mark_card_done(self.column_index, self.card_index)
        };

        if new_column != original_column || new_card_index != original_card {
            return Ok(CommandResult::Failure("Failed to undo mark card operation".to_string()));
        }

        self.column_index = original_column;
        self.card_index = original_card;
        self.executed = false;
        Ok(CommandResult::Success)
    }

    fn description(&self) -> &str {
        if self.mark_done {
            "Mark card done"
        } else {
            "Mark card undone"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Card;
    use chrono::Local;
    use std::borrow::Cow;

    #[test]
    fn test_mark_done_command_execute() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card.clone())).unwrap();

        let mut command = MarkCardCommand::mark_done(0, 0);
        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(command.executed);
        assert!(board.card(0, 0).is_none());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card.short_description());
        assert_eq!(command.column_index, 1);
        assert_eq!(command.card_index, 0);
    }

    #[test]
    fn test_mark_undone_command_execute() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(1, 0, Cow::Owned(card.clone())).unwrap();

        let mut command = MarkCardCommand::mark_undone(1, 0);
        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(command.executed);
        assert!(board.card(1, 0).is_none());
        assert_eq!(board.card(0, 0).unwrap().short_description(), card.short_description());
        assert_eq!(command.column_index, 0);
        assert_eq!(command.card_index, 0);
    }

    #[test]
    fn test_mark_done_command_undo() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card.clone())).unwrap();

        let mut command = MarkCardCommand::mark_done(0, 0);
        command.execute(&mut board).unwrap();
        assert!(board.card(0, 0).is_none());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card.short_description());

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(!command.executed);
        assert_eq!(board.card(0, 0).unwrap().short_description(), card.short_description());
        assert!(board.card(1, 0).is_none());
    }

    #[test]
    fn test_mark_undone_command_undo() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(1, 0, Cow::Owned(card.clone())).unwrap();

        let mut command = MarkCardCommand::mark_undone(1, 0);
        command.execute(&mut board).unwrap();
        assert!(board.card(1, 0).is_none());
        assert_eq!(board.card(0, 0).unwrap().short_description(), card.short_description());

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(!command.executed);
        assert_eq!(board.card(1, 0).unwrap().short_description(), card.short_description());
        assert!(board.card(0, 0).is_none());
    }

    #[test]
    fn test_mark_command_undo_before_execute() {
        let mut board = Board::new();
        let mut command = MarkCardCommand::mark_done(0, 0);

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Failure("Command was not executed".to_string()));
    }

    #[test]
    fn test_mark_command_execute_twice() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card)).unwrap();

        let mut command = MarkCardCommand::mark_done(0, 0);
        command.execute(&mut board).unwrap();

        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Failure("Command already executed".to_string()));
    }

    #[test]
    fn test_mark_command_description() {
        let command1 = MarkCardCommand::mark_done(0, 0);
        assert_eq!(command1.description(), "Mark card done");

        let command2 = MarkCardCommand::mark_undone(1, 0);
        assert_eq!(command2.description(), "Mark card undone");
    }

    #[test]
    fn test_mark_command_nonexistent_card() {
        let mut board = Board::new();
        let mut command = MarkCardCommand::mark_done(0, 0);

        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Card not found at column 0, index 0".to_string())
        );
    }

    #[test]
    fn test_mark_done_at_last_column() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(2, 0, Cow::Owned(card)).unwrap();

        let mut command = MarkCardCommand::mark_done(2, 0);
        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Cannot mark card done/undone at column boundary".to_string())
        );
    }

    #[test]
    fn test_mark_undone_at_first_column() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card)).unwrap();

        let mut command = MarkCardCommand::mark_undone(0, 0);
        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Cannot mark card done/undone at column boundary".to_string())
        );
    }

    #[test]
    fn test_mark_command_multiple_operations() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();

        let mut command1 = MarkCardCommand::mark_done(0, 0);
        command1.execute(&mut board).unwrap();
        assert_eq!(board.card(1, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 0).unwrap().short_description(), card2.short_description());

        let mut command2 = MarkCardCommand::mark_done(0, 0);
        command2.execute(&mut board).unwrap();
        assert_eq!(board.card(1, 0).unwrap().short_description(), card2.short_description());
        assert_eq!(board.card(1, 1).unwrap().short_description(), card1.short_description());

        command2.undo(&mut board).unwrap();
        assert_eq!(board.card(1, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 0).unwrap().short_description(), card2.short_description());

        command1.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card2.short_description());
    }
}
