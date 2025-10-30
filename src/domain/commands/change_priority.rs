use crate::core::{Board, Result};
use crate::domain::command::{Command, CommandResult};

/// Command for changing a card's priority (increase or decrease)
#[allow(dead_code)]
pub struct ChangePriorityCommand {
    column_index: usize,
    card_index: usize,
    increase: bool,
    original_card_index: Option<usize>,
    executed: bool,
}

impl ChangePriorityCommand {
    /// Create a new increase priority command
    #[allow(dead_code)]
    pub fn increase(column_index: usize, card_index: usize) -> Self {
        Self {
            column_index,
            card_index,
            increase: true,
            original_card_index: None,
            executed: false,
        }
    }

    /// Create a new decrease priority command
    #[allow(dead_code)]
    pub fn decrease(column_index: usize, card_index: usize) -> Self {
        Self {
            column_index,
            card_index,
            increase: false,
            original_card_index: None,
            executed: false,
        }
    }
}

impl Command for ChangePriorityCommand {
    fn execute(&mut self, board: &mut Board) -> Result<CommandResult> {
        if self.executed {
            return Ok(CommandResult::Failure("Command already executed".to_string()));
        }

        if board.card(self.column_index, self.card_index).is_none() {
            return Ok(CommandResult::Failure(format!(
                "Card not found at column {}, index {}",
                self.column_index, self.card_index
            )));
        }

        self.original_card_index = Some(self.card_index);

        let new_index = if self.increase {
            board.increase_priority(self.column_index, self.card_index).1
        } else {
            board.decrease_priority(self.column_index, self.card_index).1
        };

        self.card_index = new_index;
        self.executed = true;
        Ok(CommandResult::Success)
    }

    fn undo(&mut self, board: &mut Board) -> Result<CommandResult> {
        if !self.executed {
            return Ok(CommandResult::Failure("Command was not executed".to_string()));
        }

        let original_index = match self.original_card_index {
            Some(index) => index,
            None => {
                return Ok(CommandResult::Failure("Original card index not available for undo".to_string()));
            }
        };

        if board.card(self.column_index, self.card_index).is_none() {
            return Ok(CommandResult::Failure(format!(
                "Card not found at column {}, index {} for undo",
                self.column_index, self.card_index
            )));
        }

        if self.increase {
            board.decrease_priority(self.column_index, self.card_index);
        } else {
            board.increase_priority(self.column_index, self.card_index);
        }

        self.card_index = original_index;
        self.executed = false;
        Ok(CommandResult::Success)
    }

    fn description(&self) -> &str {
        if self.increase {
            "Increase priority"
        } else {
            "Decrease priority"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use std::borrow::Cow;
    use crate::core::Card;

    #[test]
    fn test_increase_priority_command_execute() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();

        let mut command = ChangePriorityCommand::decrease(0, 0);
        command.execute(&mut board).unwrap();

        assert_eq!(board.card(0, 0).unwrap().short_description(), card2.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card1.short_description());
    }

    #[test]
    fn test_decrease_priority_command_execute() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();

        let mut command = ChangePriorityCommand::increase(0, 1);
        command.execute(&mut board).unwrap();

        assert_eq!(board.card(0, 0).unwrap().short_description(), card2.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card1.short_description());
    }

    #[test]
    fn test_increase_priority_command_undo() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();

        let mut command = ChangePriorityCommand::decrease(0, 0);
        command.execute(&mut board).unwrap();

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(!command.executed);

        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card2.short_description());
    }

    #[test]
    fn test_decrease_priority_command_undo() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();

        let mut command = ChangePriorityCommand::increase(0, 1);
        command.execute(&mut board).unwrap();

        let result = command.undo(&mut board).unwrap();
        assert_eq!(result, CommandResult::Success);
        assert!(!command.executed);

        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
        assert_eq!(board.card(0, 1).unwrap().short_description(), card2.short_description());
    }

    #[test]
    fn test_priority_command_undo_before_execute() {
        let mut board = Board::new();
        let mut command = ChangePriorityCommand::increase(0, 0);

        let result = command.undo(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Command was not executed".to_string())
        );
    }

    #[test]
    fn test_priority_command_execute_twice() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1)).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2)).unwrap();

        let mut command = ChangePriorityCommand::increase(0, 1);
        command.execute(&mut board).unwrap();

        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Command already executed".to_string())
        );
    }

    #[test]
    fn test_priority_command_description() {
        let command1 = ChangePriorityCommand::increase(0, 0);
        assert_eq!(command1.description(), "Increase priority");

        let command2 = ChangePriorityCommand::decrease(0, 0);
        assert_eq!(command2.description(), "Decrease priority");
    }

    #[test]
    fn test_priority_command_nonexistent_card() {
        let mut board = Board::new();
        let mut command = ChangePriorityCommand::increase(0, 0);

        let result = command.execute(&mut board).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Card not found at column 0, index 0".to_string())
        );
    }

    #[test]
    fn test_priority_command_at_boundaries() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();

        let mut command_increase_top = ChangePriorityCommand::increase(0, 0);
        let result1 = command_increase_top.execute(&mut board).unwrap();
        assert_eq!(result1, CommandResult::Success);
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());

        let mut command_decrease_bottom = ChangePriorityCommand::decrease(0, 1);
        let result2 = command_decrease_bottom.execute(&mut board).unwrap();
        assert_eq!(result2, CommandResult::Success);
        assert_eq!(board.card(0, 1).unwrap().short_description(), card2.short_description());
    }

    #[test]
    fn test_priority_command_multiple_operations() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        let card3 = Card::new("Card 3", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1.clone())).unwrap();
        board.insert_card(0, 1, Cow::Owned(card2.clone())).unwrap();
        board.insert_card(0, 2, Cow::Owned(card3.clone())).unwrap();

        let mut command1 = ChangePriorityCommand::decrease(0, 0);
        command1.execute(&mut board).unwrap();
        assert_eq!(board.card(0, 1).unwrap().short_description(), card1.short_description());

        let mut command2 = ChangePriorityCommand::decrease(0, 1);
        command2.execute(&mut board).unwrap();
        assert_eq!(board.card(0, 2).unwrap().short_description(), card1.short_description());

        command2.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 1).unwrap().short_description(), card1.short_description());

        command1.undo(&mut board).unwrap();
        assert_eq!(board.card(0, 0).unwrap().short_description(), card1.short_description());
    }
}

