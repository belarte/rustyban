use std::borrow::Cow;

use super::{check_already_executed, validate_card_exists};
use crate::core::{Board, Card, Result};
use crate::domain::command::{Command, CommandResult};

/// Command for updating a card in the board
#[allow(dead_code)]
pub struct UpdateCardCommand {
    column_index: usize,
    card_index: usize,
    new_card: Card,
    old_card: Option<Card>,
    executed: bool,
}

impl UpdateCardCommand {
    /// Create a new update card command
    #[allow(dead_code)]
    pub fn new(column_index: usize, card_index: usize, new_card: Card) -> Self {
        Self {
            column_index,
            card_index,
            new_card,
            old_card: None,
            executed: false,
        }
    }
}

impl Command for UpdateCardCommand {
    fn execute(&mut self, board: &mut Board) -> Result<CommandResult> {
        if let Some(result) = check_already_executed(self.executed) {
            return Ok(result);
        }

        if let Ok(CommandResult::Failure(msg)) = validate_card_exists(board, self.column_index, self.card_index) {
            return Ok(CommandResult::Failure(msg));
        }

        let old_card = board.card(self.column_index, self.card_index).unwrap().clone();

        self.old_card = Some(old_card);

        let result = board.update_card(self.column_index, self.card_index, Cow::Borrowed(&self.new_card));
        match result {
            Ok(_) => {
                self.executed = true;
                Ok(CommandResult::Success)
            }
            Err(e) => Ok(CommandResult::Failure(format!("Failed to update card: {}", e))),
        }
    }

    fn undo(&mut self, board: &mut Board) -> Result<CommandResult> {
        if !self.executed {
            return Ok(CommandResult::Failure("Command was not executed".to_string()));
        }

        let old_card = match self.old_card.as_ref() {
            Some(card) => card,
            None => {
                return Ok(CommandResult::Failure(
                    "Old card data not available for undo".to_string(),
                ));
            }
        };

        let result = board.update_card(self.column_index, self.card_index, Cow::Borrowed(old_card));
        match result {
            Ok(_) => {
                self.executed = false;
                Ok(CommandResult::Success)
            }
            Err(e) => Ok(CommandResult::Failure(format!("Failed to undo update: {}", e))),
        }
    }

    fn description(&self) -> &str {
        "Update card"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_update_card_command_execute() {
        let mut board = Board::new();
        let old_card = Card::new("Old card", Local::now());
        board.insert_card(0, 0, Cow::Owned(old_card.clone())).unwrap();

        let new_card = Card::new("New card", Local::now());
        let mut command = UpdateCardCommand::new(0, 0, new_card.clone());

        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(command.executed);
        assert!(command.old_card.is_some());

        let updated_card = board.card(0, 0).unwrap();
        assert_eq!(updated_card.short_description(), new_card.short_description());
    }

    #[test]
    fn test_update_card_command_undo() {
        let mut board = Board::new();
        let old_card = Card::new("Old card", Local::now());
        board.insert_card(0, 0, Cow::Owned(old_card.clone())).unwrap();

        let new_card = Card::new("New card", Local::now());
        let mut command = UpdateCardCommand::new(0, 0, new_card);

        command.execute(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), "New card");

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(!command.executed);

        let restored_card = board.card(0, 0).unwrap();
        assert_eq!(restored_card.short_description(), old_card.short_description());
    }

    #[test]
    fn test_update_card_command_undo_before_execute() {
        let mut board = Board::new();
        let new_card = Card::new("New card", Local::now());
        let mut command = UpdateCardCommand::new(0, 0, new_card);

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Failure("Command was not executed".to_string()));
    }

    #[test]
    fn test_update_card_command_execute_twice() {
        let mut board = Board::new();
        let old_card = Card::new("Old card", Local::now());
        board.insert_card(0, 0, Cow::Owned(old_card)).unwrap();

        let new_card = Card::new("New card", Local::now());
        let mut command = UpdateCardCommand::new(0, 0, new_card);

        command.execute(&mut board).unwrap();

        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Failure("Command already executed".to_string()));
    }

    #[test]
    fn test_update_card_command_description() {
        let new_card = Card::new("New card", Local::now());
        let command = UpdateCardCommand::new(0, 0, new_card);
        assert_eq!(command.description(), "Update card");
    }

    #[test]
    fn test_update_card_command_nonexistent_card() {
        let mut board = Board::new();
        let new_card = Card::new("New card", Local::now());
        let mut command = UpdateCardCommand::new(0, 0, new_card);

        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Card not found at column 0, index 0".to_string())
        );
    }

    #[test]
    fn test_update_card_command_multiple_updates() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();

        let card2 = Card::new("Card 2", Local::now());
        let mut command1 = UpdateCardCommand::new(0, 0, card2.clone());

        command1.execute(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), "Card 2");

        command1.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), "Card 1");
    }

    #[test]
    fn test_update_card_command_different_columns() {
        let mut board = Board::new();
        let old_card1 = Card::new("Old card 1", Local::now());
        let old_card2 = Card::new("Old card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(old_card1.clone())).unwrap();
        board.insert_card(1, 0, Cow::Owned(old_card2.clone())).unwrap();

        let new_card1 = Card::new("New card 1", Local::now());
        let new_card2 = Card::new("New card 2", Local::now());
        let mut command1 = UpdateCardCommand::new(0, 0, new_card1.clone());
        let mut command2 = UpdateCardCommand::new(1, 0, new_card2.clone());

        command1.execute(&mut board).unwrap();
        command2.execute(&mut board).unwrap();

        assert_eq!(
            board.card(0, 0).unwrap().short_description(),
            new_card1.short_description()
        );
        assert_eq!(
            board.card(1, 0).unwrap().short_description(),
            new_card2.short_description()
        );

        command2.undo(&mut board).unwrap();
        assert_eq!(
            board.card(0, 0).unwrap().short_description(),
            new_card1.short_description()
        );
        assert_eq!(
            board.card(1, 0).unwrap().short_description(),
            old_card2.short_description()
        );

        command1.undo(&mut board).unwrap();
        assert_eq!(
            board.card(0, 0).unwrap().short_description(),
            old_card1.short_description()
        );
        assert_eq!(
            board.card(1, 0).unwrap().short_description(),
            old_card2.short_description()
        );
    }
}
