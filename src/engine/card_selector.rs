use std::{cell::RefCell, cmp::min, rc::Rc};

use crate::core::{Board, Card};
use crate::domain::services::CardSelector as CardSelectorTrait;

#[derive(Debug, Default)]
pub struct CardSelector {
    selected_column: usize,
    selected_card: usize,
    selection_enabled: bool,
    board: Rc<RefCell<Board>>,
}

impl CardSelector {
    pub fn new(board: Rc<RefCell<Board>>) -> Self {
        Self {
            selected_column: 0,
            selected_card: 0,
            selection_enabled: false,
            board,
        }
    }

    fn select<F>(&mut self, update_selection: F) -> (usize, usize)
    where
        F: FnOnce(&mut Self),
    {
        if self.selection_enabled {
            update_selection(self);
        } else {
            self.selection_enabled = true;
        }

        (self.selected_column, self.selected_card)
    }

    fn get_card_index(&self, index: usize) -> usize {
        let board = self.board.as_ref().borrow();
        if let Some(column) = board.column(self.selected_column) {
            if column.is_empty() {
                return 0;
            }
            min(index, column.size().saturating_sub(1))
        } else {
            0
        }
    }

    fn next_card_index(&self) -> usize {
        self.get_card_index(self.selected_card + 1)
    }

    fn prev_card_index(&self) -> usize {
        if self.selected_card == 0 {
            return 0;
        }

        self.get_card_index(self.selected_card - 1)
    }

    fn next_column_index(&self, current_index: usize) -> usize {
        let board = self.board.as_ref().borrow();
        min(current_index + 1, board.columns_count() - 1)
    }

    fn prev_column_index(&self, current_index: usize) -> usize {
        let board = self.board.as_ref().borrow();

        if current_index == 0 {
            return 0;
        }

        min(current_index - 1, board.columns_count() - 1)
    }
}

impl CardSelectorTrait for CardSelector {
    fn get(&self) -> Option<(usize, usize)> {
        if self.selection_enabled {
            Some((self.selected_column, self.selected_card))
        } else {
            None
        }
    }

    fn set(&mut self, column_index: usize, card_index: usize) {
        let board = self.board.as_ref().borrow();
        self.selected_column = min(column_index, board.columns_count().saturating_sub(1));
        self.selected_card = if let Some(column) = board.column(self.selected_column) {
            if column.is_empty() {
                0
            } else {
                min(card_index, column.size().saturating_sub(1))
            }
        } else {
            0
        };
    }

    fn get_selected_card(&self) -> Option<Card> {
        let board = self.board.as_ref().borrow();
        if self.selection_enabled {
            if let Some(column) = board.column(self.selected_column) {
                if !column.is_empty() {
                    return board.card(self.selected_column, self.selected_card).cloned();
                }
            }
        }
        None
    }

    fn select_next_column(&mut self) -> (usize, usize) {
        self.select(|this| {
            this.selected_column = this.next_column_index(this.selected_column);
            this.selected_card = this.get_card_index(this.selected_card);
        })
    }

    fn select_prev_column(&mut self) -> (usize, usize) {
        self.select(|this| {
            this.selected_column = this.prev_column_index(this.selected_column);
            this.selected_card = this.get_card_index(this.selected_card);
        })
    }

    fn select_next_card(&mut self) -> (usize, usize) {
        self.select(|this| {
            this.selected_card = this.next_card_index();
        })
    }

    fn select_prev_card(&mut self) -> (usize, usize) {
        self.select(|this| {
            this.selected_card = this.prev_card_index();
        })
    }

    fn disable_selection(&mut self) {
        self.selection_enabled = false;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::core::Board;
    use crate::domain::services::CardSelector as CardSelectorTrait;
    use crate::{Result, RustybanError};

    use super::CardSelector;

    fn create_board(file_name: &str) -> Result<Rc<RefCell<Board>>> {
        let board = Board::open(file_name).map_err(|e| RustybanError::InvalidOperation {
            message: format!("Cannot open test file '{}': {}", file_name, e),
        })?;
        Ok(Rc::new(RefCell::new(board)))
    }

    #[test]
    fn card_selection() -> Result<()> {
        let board = create_board("res/test_board.json")?;
        let mut selector = CardSelector::new(board);

        assert_eq!((0, 0), selector.select_next_column());
        assert_eq!((1, 0), selector.select_next_column());
        assert_eq!((2, 0), selector.select_next_column());
        assert_eq!((2, 0), selector.select_next_column());

        assert_eq!((1, 0), selector.select_prev_column());
        assert_eq!((0, 0), selector.select_prev_column());
        assert_eq!((0, 0), selector.select_prev_column());

        assert_eq!((0, 1), selector.select_next_card());
        assert_eq!((0, 2), selector.select_next_card());
        assert_eq!((0, 2), selector.select_next_card());

        assert_eq!((0, 1), selector.select_prev_card());
        assert_eq!((0, 0), selector.select_prev_card());
        assert_eq!((0, 0), selector.select_prev_card());

        Ok(())
    }

    #[test]
    fn get_the_card_index() -> Result<()> {
        let board = create_board("res/test_board.json")?;
        let mut selector = CardSelector::new(board);

        assert_eq!(None, selector.get());
        selector.select_next_card();
        assert_eq!(Some((0, 0)), selector.get());

        selector.select_next_column();
        selector.select_next_column();
        selector.select_next_card();
        assert_eq!(Some((2, 1)), selector.get());

        selector.select_next_column();
        selector.select_next_card();
        assert_eq!(Some((2, 1)), selector.get());

        selector.disable_selection();
        assert_eq!(None, selector.get());

        Ok(())
    }

    #[test]
    fn set_the_card_index() -> Result<()> {
        let board = create_board("res/test_board_with_empty_column.json")?;
        let mut selector = CardSelector::new(board);
        selector.select_next_card();

        let cases: Vec<((usize, usize), (usize, usize))> = vec![
            ((0, 0), (0, 0)),
            ((0, 1), (0, 1)),
            ((0, 2), (0, 2)),
            ((0, 3), (0, 2)),
            ((1, 0), (1, 0)),
            ((2, 0), (2, 0)),
            ((2, 1), (2, 1)),
            ((2, 2), (2, 1)),
            ((3, 0), (2, 0)),
        ];

        for (input, expected) in cases {
            let (column_index, card_index) = input;
            selector.set(column_index, card_index);

            let output = selector.get().ok_or_else(|| RustybanError::InvalidOperation {
                message: "Expected selector to have a valid selection".to_string(),
            })?;
            assert_eq!(expected, output);
        }

        Ok(())
    }

    #[test]
    fn returns_none_on_empty_board() -> Result<()> {
        let board = create_board("res/test_board_with_empty_column.json")?;
        let mut selector = CardSelector::new(board);

        selector.select_next_column();
        selector.select_next_column();
        assert_eq!(None, selector.get_selected_card());

        Ok(())
    }
}
