use std::borrow::Cow;

use crate::core::{Board, Card, Result};
use crate::domain::command::{Command, CommandResult};

/// Command for moving a card between columns
#[allow(dead_code)]
pub struct MoveCardCommand {
    source_column_index: usize,
    source_card_index: usize,
    target_column_index: usize,
    target_card_index: usize,
    actual_target_index: Option<usize>,
    card: Option<Card>,
    executed: bool,
}

impl MoveCardCommand {
    /// Create a new move card command
    #[allow(dead_code)]
    pub fn new(
        source_column_index: usize,
        source_card_index: usize,
        target_column_index: usize,
        target_card_index: usize,
    ) -> Self {
        Self {
            source_column_index,
            source_card_index,
            target_column_index,
            target_card_index,
            actual_target_index: None,
            card: None,
            executed: false,
        }
    }
}

impl Command for MoveCardCommand {
    fn execute(&mut self, board: &mut Board) -> Result<CommandResult> {
        if self.executed {
            return Ok(CommandResult::Failure("Command already executed".to_string()));
        }

        let card = match board.card(self.source_column_index, self.source_card_index) {
            Some(card) => card.clone(),
            None => {
                return Ok(CommandResult::Failure(format!(
                    "Card not found at column {}, index {}",
                    self.source_column_index, self.source_card_index
                )));
            }
        };

        self.card = Some(card.clone());

        let mut adjusted_target_index = self.target_card_index;
        if self.source_column_index == self.target_column_index && self.source_card_index < self.target_card_index {
            adjusted_target_index = self.target_card_index.saturating_sub(1);
        }

        let column_size = board.column(self.target_column_index)
            .map(|c| c.size())
            .unwrap_or(0);
        
        if self.source_column_index == self.target_column_index {
            adjusted_target_index = adjusted_target_index.min(column_size - 1);
        } else {
            adjusted_target_index = adjusted_target_index.min(column_size);
        }

        self.actual_target_index = Some(adjusted_target_index);

        board.remove_card(self.source_column_index, self.source_card_index)?;
        board.insert_card(self.target_column_index, adjusted_target_index, Cow::Owned(card))?;

        self.executed = true;
        Ok(CommandResult::Success)
    }

    fn undo(&mut self, board: &mut Board) -> Result<CommandResult> {
        if !self.executed {
            return Ok(CommandResult::Failure("Command was not executed".to_string()));
        }

        let card = match self.card.as_ref() {
            Some(card) => card,
            None => {
                return Ok(CommandResult::Failure("Card data not available for undo".to_string()));
            }
        };

        let actual_target = match self.actual_target_index {
            Some(index) => index,
            None => {
                return Ok(CommandResult::Failure("Actual target index not available for undo".to_string()));
            }
        };

        board.remove_card(self.target_column_index, actual_target)?;
        board.insert_card(self.source_column_index, self.source_card_index, Cow::Borrowed(card))?;

        self.executed = false;
        Ok(CommandResult::Success)
    }

    fn description(&self) -> &str {
        "Move card"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_move_card_command_execute() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card.clone())).unwrap();

        let mut command = MoveCardCommand::new(0, 0, 1, 0);
        let result = command.execute(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(command.executed);
        assert!(command.card.is_some());
        assert!(board.card(0, 0).is_none());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card.short_description());
    }

    #[test]
    fn test_move_card_command_undo() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card.clone())).unwrap();

        let mut command = MoveCardCommand::new(0, 0, 1, 0);
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
    fn test_move_card_command_undo_before_execute() {
        let mut board = Board::new();
        let mut command = MoveCardCommand::new(0, 0, 1, 0);

        let result = command.undo(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Command was not executed".to_string())
        );
    }

    #[test]
    fn test_move_card_command_execute_twice() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card)).unwrap();

        let mut command = MoveCardCommand::new(0, 0, 1, 0);
        command.execute(&mut board).unwrap();

        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Command already executed".to_string())
        );
    }

    #[test]
    fn test_move_card_command_description() {
        let command = MoveCardCommand::new(0, 0, 1, 0);
        assert_eq!(command.description(), "Move card");
    }

    #[test]
    fn test_move_card_command_nonexistent_card() {
        let mut board = Board::new();
        let mut command = MoveCardCommand::new(0, 0, 1, 0);

        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Card not found at column 0, index 0".to_string())
        );
    }

    #[test]
    fn test_move_card_command_same_column_different_index() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();

        let mut command = MoveCardCommand::new(0, 0, 0, 2);
        command.execute(&mut board).unwrap();

        assert_eq!(board.card(0, 0).unwrap().short_description(), card2.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card1.short_description());
        assert!(board.card(0, 2).is_none());

        command.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card2.short_description());
    }

    #[test]
    fn test_move_card_command_different_columns() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(1, 0, Cow::Owned(card2.clone())).unwrap();

        let mut command = MoveCardCommand::new(0, 0, 2, 0);
        command.execute(&mut board).unwrap();

        assert!(board.card(0, 0).is_none());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card2.short_description());
        assert_eq!(board.card(2, 0).unwrap().short_description(), card1.short_description());

        command.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card2.short_description());
        assert!(board.card(2, 0).is_none());
    }

    #[test]
    fn test_move_card_command_with_existing_cards_in_target() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        let card3 = Card::new("Card 3", Local::now());

        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(1, 0, Cow::Owned(card2.clone())).unwrap();
        board.insert_card(1, 1, Cow::Owned(card3.clone())).unwrap();

        let mut command = MoveCardCommand::new(0, 0, 1, 1);
        command.execute(&mut board).unwrap();

        assert!(board.card(0, 0).is_none());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card2.short_description());
        assert_eq!(board.card(1, 1).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(1, 2).unwrap().short_description(), card3.short_description());

        command.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(1, 0).unwrap().short_description(), card2.short_description());
        assert_eq!(board.card(1, 1).unwrap().short_description(), card3.short_description());
    }
}

