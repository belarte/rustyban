use crate::core::{Board, Result};
use crate::domain::command::CommandResult;

pub fn check_already_executed(executed: bool) -> Option<CommandResult> {
    if executed {
        Some(CommandResult::Failure("Command already executed".to_string()))
    } else {
        None
    }
}

pub fn check_not_executed(executed: bool) -> Option<CommandResult> {
    if !executed {
        Some(CommandResult::Failure("Command was not executed".to_string()))
    } else {
        None
    }
}

pub fn validate_card_exists(board: &Board, column_index: usize, card_index: usize) -> Result<CommandResult> {
    if board.card(column_index, card_index).is_none() {
        Ok(CommandResult::Failure(format!(
            "Card not found at column {}, index {}",
            column_index, card_index
        )))
    } else {
        Ok(CommandResult::Success)
    }
}

pub fn validate_card_exists_for_undo(board: &Board, column_index: usize, card_index: usize) -> Result<CommandResult> {
    if board.card(column_index, card_index).is_none() {
        Ok(CommandResult::Failure(format!(
            "Card not found at column {}, index {} for undo",
            column_index, card_index
        )))
    } else {
        Ok(CommandResult::Success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Card;
    use chrono::Local;
    use std::borrow::Cow;

    #[test]
    fn test_check_already_executed_when_not_executed() {
        let result = check_already_executed(false);
        assert_eq!(result, None);
    }

    #[test]
    fn test_check_already_executed_when_executed() {
        let result = check_already_executed(true);
        assert_eq!(
            result,
            Some(CommandResult::Failure("Command already executed".to_string()))
        );
    }

    #[test]
    fn test_check_not_executed_when_executed() {
        let result = check_not_executed(true);
        assert_eq!(result, None);
    }

    #[test]
    fn test_check_not_executed_when_not_executed() {
        let result = check_not_executed(false);
        assert_eq!(
            result,
            Some(CommandResult::Failure("Command was not executed".to_string()))
        );
    }

    #[test]
    fn test_validate_card_exists_when_card_exists() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card)).unwrap();

        let result = validate_card_exists(&board, 0, 0).unwrap();
        assert_eq!(result, CommandResult::Success);
    }

    #[test]
    fn test_validate_card_exists_when_card_not_exists() {
        let board = Board::new();

        let result = validate_card_exists(&board, 0, 0).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Card not found at column 0, index 0".to_string())
        );
    }

    #[test]
    fn test_validate_card_exists_different_positions() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1)).unwrap();
        board.insert_card(1, 0, Cow::Owned(card2)).unwrap();

        assert_eq!(validate_card_exists(&board, 0, 0).unwrap(), CommandResult::Success);
        assert_eq!(validate_card_exists(&board, 1, 0).unwrap(), CommandResult::Success);
        assert_eq!(
            validate_card_exists(&board, 0, 1).unwrap(),
            CommandResult::Failure("Card not found at column 0, index 1".to_string())
        );
        assert_eq!(
            validate_card_exists(&board, 2, 0).unwrap(),
            CommandResult::Failure("Card not found at column 2, index 0".to_string())
        );
    }

    #[test]
    fn test_validate_card_exists_for_undo_when_card_exists() {
        let mut board = Board::new();
        let card = Card::new("Test card", Local::now());
        board.insert_card(0, 0, Cow::Owned(card)).unwrap();

        let result = validate_card_exists_for_undo(&board, 0, 0).unwrap();
        assert_eq!(result, CommandResult::Success);
    }

    #[test]
    fn test_validate_card_exists_for_undo_when_card_not_exists() {
        let board = Board::new();

        let result = validate_card_exists_for_undo(&board, 0, 0).unwrap();
        assert_eq!(
            result,
            CommandResult::Failure("Card not found at column 0, index 0 for undo".to_string())
        );
    }

    #[test]
    fn test_validate_card_exists_for_undo_different_positions() {
        let mut board = Board::new();
        let card1 = Card::new("Card 1", Local::now());
        let card2 = Card::new("Card 2", Local::now());
        board.insert_card(0, 0, Cow::Owned(card1)).unwrap();
        board.insert_card(1, 0, Cow::Owned(card2)).unwrap();

        assert_eq!(
            validate_card_exists_for_undo(&board, 0, 0).unwrap(),
            CommandResult::Success
        );
        assert_eq!(
            validate_card_exists_for_undo(&board, 1, 0).unwrap(),
            CommandResult::Success
        );
        assert_eq!(
            validate_card_exists_for_undo(&board, 0, 1).unwrap(),
            CommandResult::Failure("Card not found at column 0, index 1 for undo".to_string())
        );
    }
}

