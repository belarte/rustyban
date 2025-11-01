use std::borrow::Cow;

use super::{check_already_executed, check_not_executed, validate_card_exists};
use crate::core::{Board, Card, Result};
use crate::domain::command::{Command, CommandResult};

/// Command for removing a card from the board
#[allow(dead_code)]
pub struct RemoveCardCommand {
    column_index: usize,
    card_index: usize,
    card: Option<Card>,
    executed: bool,
}

impl RemoveCardCommand {
    /// Create a new remove card command
    #[allow(dead_code)]
    pub fn new(column_index: usize, card_index: usize) -> Self {
        Self {
            column_index,
            card_index,
            card: None,
            executed: false,
        }
    }
}

impl Command for RemoveCardCommand {
    fn execute(&mut self, board: &mut Board) -> Result<CommandResult> {
        if let Some(result) = check_already_executed(self.executed) {
            return Ok(result);
        }

        if let Ok(CommandResult::Failure(msg)) = validate_card_exists(board, self.column_index, self.card_index) {
            return Ok(CommandResult::Failure(msg));
        }

        let card = board.card(self.column_index, self.card_index).unwrap().clone();

        self.card = Some(card);

        let result = board.remove_card(self.column_index, self.card_index);
        match result {
            Ok(_) => {
                self.executed = true;
                Ok(CommandResult::Success)
            }
            Err(e) => Ok(CommandResult::Failure(format!("Failed to remove card: {}", e))),
        }
    }

    fn undo(&mut self, board: &mut Board) -> Result<CommandResult> {
        if let Some(result) = check_not_executed(self.executed) {
            return Ok(result);
        }

        let card = match self.card.as_ref() {
            Some(card) => card,
            None => {
                return Ok(CommandResult::Failure("Card data not available for undo".to_string()));
            }
        };

        let result = board.insert_card(self.column_index, self.card_index, Cow::Borrowed(card));
        match result {
            Ok(_) => {
                self.executed = false;
                Ok(CommandResult::Success)
            }
            Err(e) => Ok(CommandResult::Failure(format!("Failed to undo remove: {}", e))),
        }
    }

    fn description(&self) -> &str {
        "Remove card"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_remove_card_command_execute() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card.clone())).unwrap();

        let mut command = RemoveCardCommand::new(0, 0);
        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(command.executed);
        assert!(command.card.is_some());
        assert_eq!(
            command.card.as_ref().unwrap().short_description(),
            card.short_description()
        );
        assert!(board.card(0, 0).is_none());
    }

    #[test]
    fn test_remove_card_command_undo() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card.clone())).unwrap();

        let mut command = RemoveCardCommand::new(0, 0);
        command.execute(&mut board).unwrap();
        assert!(board.card(0, 0).is_none());

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(!command.executed);

        let restored_card = board.card(0, 0).unwrap();
        assert_eq!(restored_card.short_description(), card.short_description());
    }

    #[test]
    fn test_remove_card_command_undo_before_execute() {
        let mut board = Board::new();
        let mut command = RemoveCardCommand::new(0, 0);

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Failure("Command was not executed".to_string()));
    }

    #[test]
    fn test_remove_card_command_execute_twice() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card)).unwrap();

        let mut command = RemoveCardCommand::new(0, 0);
        command.execute(&mut board).unwrap();

        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Failure("Command already executed".to_string()));
    }

    #[test]
    fn test_remove_card_command_description() {
        let command = RemoveCardCommand::new(0, 0);
        assert_eq!(command.description(), "Remove card");
    }

    #[test]
    fn test_remove_card_command_multiple_cards() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        let card3 = Card::new("Card 3", Local::now());

        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();
        board.insert_card(0, 2, Cow::Owned(card3.clone())).unwrap();

        let mut command1 = RemoveCardCommand::new(0, 1);
        command1.execute(&mut board).unwrap();

        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card3.short_description());
        assert!(board.card(0, 2).is_none());

        command1.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card2.short_description());
        assert_eq!(board.card(0, 2).unwrap().short_description(), card3.short_description());
    }

    #[test]
    fn test_remove_card_command_different_columns() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());

        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(1, 0, Cow::Owned(card2.clone())).unwrap();

        let mut command1 = RemoveCardCommand::new(0, 0);
        let mut command2 = RemoveCardCommand::new(1, 0);

        command1.execute(&mut board).unwrap();
        command2.execute(&mut board).unwrap();

        assert!(board.card(0, 0).is_none());
        assert!(board.card(1, 0).is_none());

        command2.undo(&mut board).unwrap();
        assert!(board.card(0, 0).is_none());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card2.short_description());

        command1.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card2.short_description());
    }

    #[test]
    fn test_remove_card_command_nonexistent_card() {
        let mut board = Board::new();
        let mut command = RemoveCardCommand::new(0, 0);

        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Card not found at column 0, index 0".to_string())
        );
    }
}
