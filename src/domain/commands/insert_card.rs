use std::borrow::Cow;

use crate::core::{Board, Card, Result};
use crate::domain::command::{Command, CommandResult};

/// Command for inserting a card into the board
#[allow(dead_code)]
pub struct InsertCardCommand {
    column_index: usize,
    card_index: usize,
    card: Card,
    executed: bool,
}

impl InsertCardCommand {
    /// Create a new insert card command
    #[allow(dead_code)]
    pub fn new(column_index: usize, card_index: usize, card: Card) -> Self {
        Self {
            column_index,
            card_index,
            card,
            executed: false,
        }
    }
}

impl Command for InsertCardCommand {
    fn execute(&mut self, board: &mut Board) -> Result<CommandResult> {
        board.insert_card(self.column_index, self.card_index, Cow::Owned(self.card.clone()))?;
        self.executed = true;
        Ok(CommandResult::Success)
    }

    fn undo(&mut self, board: &mut Board) -> Result<CommandResult> {
        if !self.executed {
            return Ok(CommandResult::Failure("Command was not executed".to_string()));
        }

        let result = board.remove_card(self.column_index, self.card_index);
        match result {
            Ok(_) => {
                self.executed = false;
                Ok(CommandResult::Success)
            }
            Err(e) => Ok(CommandResult::Failure(format!("Failed to undo insert: {}", e))),
        }
    }

    fn description(&self) -> &str {
        "Insert card"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_insert_card_command_execute() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        let mut command = InsertCardCommand::new(0, 0, card.clone());

        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(command.executed);

        let inserted_card = board.card(0, 0).unwrap();
        assert_eq!(inserted_card.short_description(), card.short_description());
    }

    #[test]
    fn test_insert_card_command_undo() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        let mut command = InsertCardCommand::new(0, 0, card.clone());

        command.execute(&mut board).unwrap();
        assert!(board.card(0, 0).is_some());

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(!command.executed);
        assert!(board.card(0, 0).is_none());
    }

    #[test]
    fn test_insert_card_command_undo_before_execute() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        let mut command = InsertCardCommand::new(0, 0, card);

        let result = command.undo(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Command was not executed".to_string())
        );
    }

    #[test]
    fn test_insert_card_command_description() {
        let card = Card::new("Test card", Local::now());
        let command = InsertCardCommand::new(0, 0, card);
        assert_eq!(command.description(), "Insert card");
    }

    #[test]
    fn test_insert_card_command_multiple_cards() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());

        let mut command1 = InsertCardCommand::new(0, 0, card1.clone());
        let mut command2 = InsertCardCommand::new(0, 1, card2.clone());

        command1.execute(&mut board).unwrap();
        command2.execute(&mut board).unwrap();

        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card2.short_description());

        command2.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert!(board.card(0, 1).is_none());

        command1.undo(&mut board).unwrap();
        assert!(board.card(0, 0).is_none());
    }

    #[test]
    fn test_insert_card_command_different_columns() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());

        let mut command1 = InsertCardCommand::new(0, 0, card1.clone());
        let mut command2 = InsertCardCommand::new(1, 0, card2.clone());

        command1.execute(&mut board).unwrap();
        command2.execute(&mut board).unwrap();

        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card2.short_description());

        command2.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert!(board.card(1, 0).is_none());

        command1.undo(&mut board).unwrap();
        assert!(board.card(0, 0).is_none());
    }
}

