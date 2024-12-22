use std::cmp::min;

use crate::board::{Board, Card};

#[derive(Debug, Default)]
pub struct CardSelector {
    selected_column: usize,
    selected_card: usize,
    selection_enabled: bool,
}

impl CardSelector {
    pub fn new() -> Self {
        Self {
            selected_column: 0,
            selected_card: 0,
            selection_enabled: false,
        }
    }

    pub fn get(&self) -> Option<(usize, usize)> {
        if self.selection_enabled {
            Some((self.selected_column, self.selected_card))
        } else {
            None
        }
    }

    pub fn set(&mut self, column_index: usize, card_index: usize, board: &Board) {
        self.selected_column = min(column_index, board.columns_count() - 1);
        self.selected_card = if board.column(self.selected_column).is_empty() {
            0
        } else {
            min(card_index, board.column(self.selected_column).size() - 1)
        }
    }

    pub fn get_selected_card(&self, board: &Board) -> Option<Card> {
        if self.selection_enabled && !board.column(self.selected_column).is_empty() {
            Some(board.card(self.selected_column, self.selected_card).clone())
        } else {
            None
        }
    }

    pub fn select_next_column(&mut self, board: &mut Board) {
        self.select(board, |this, board| {
            this.selected_column = this.next_column_index(board, this.selected_column);
            this.selected_card = this.get_card_index(board, this.selected_card);
        });
    }

    pub fn select_prev_column(&mut self, board: &mut Board) {
        self.select(board, |this, board| {
            this.selected_column = this.prev_column_index(board, this.selected_column);
            this.selected_card = this.get_card_index(board, this.selected_card);
        });
    }

    pub fn select_next_card(&mut self, board: &mut Board) {
        self.select(board, |this, board| {
            this.selected_card = this.next_card_index(board);
        });
    }

    pub fn select_prev_card(&mut self, board: &mut Board) {
        self.select(board, |this, board| {
            this.selected_card = this.prev_card_index(board);
        });
    }

    pub fn disable_selection(&mut self, board: &mut Board) {
        self.selection_enabled = false;
        board.deselect_card(self.selected_column, self.selected_card);
    }

    fn select<F>(&mut self, board: &mut Board, update_selection: F)
    where
        F: FnOnce(&mut Self, &mut Board),
    {
        if self.selection_enabled {
            board.deselect_card(self.selected_column, self.selected_card);
            update_selection(self, board);
        } else {
            self.selection_enabled = true;
        }

        board.select_card(self.selected_column, self.selected_card);
    }

    fn get_card_index(&self, board: &Board, index: usize) -> usize {
        let column = board.column(self.selected_column);

        if column.is_empty() {
            return 0;
        }

        min(index, column.size() - 1)
    }

    fn next_card_index(&self, board: &Board) -> usize {
        self.get_card_index(board, self.selected_card + 1)
    }

    fn prev_card_index(&self, board: &Board) -> usize {
        if self.selected_card == 0 {
            return 0;
        }

        self.get_card_index(board, self.selected_card - 1)
    }

    fn next_column_index(&self, board: &Board, current_index: usize) -> usize {
        min(current_index + 1, board.columns_count() - 1)
    }

    fn prev_column_index(&self, board: &Board, current_index: usize) -> usize {
        if current_index == 0 {
            return 0;
        }

        min(current_index - 1, board.columns_count() - 1)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use crate::board::Board;

    use super::CardSelector;

    #[test]
    fn card_selection() -> Result<()> {
        let mut board = Board::open("res/test_board.json")?;
        let mut selector = CardSelector::new();

        assert!(!board.card(0, 0).is_selected());
        selector.select_next_card(&mut board);
        assert!(board.card(0, 0).is_selected());

        assert!(!board.card(0, 1).is_selected());
        selector.select_next_card(&mut board);
        assert!(board.card(0, 1).is_selected());

        assert!(!board.card(0, 2).is_selected());
        selector.select_next_card(&mut board);
        assert!(board.card(0, 2).is_selected());

        assert!(!board.card(1, 0).is_selected());
        selector.select_next_column(&mut board);
        selector.select_next_card(&mut board);
        assert!(board.card(1, 0).is_selected());

        assert!(!board.card(2, 0).is_selected());
        selector.select_next_column(&mut board);
        selector.select_next_column(&mut board);
        selector.select_prev_card(&mut board);
        assert!(board.card(2, 0).is_selected());

        assert!(!board.card(0, 0).is_selected());
        selector.select_prev_column(&mut board);
        selector.select_prev_column(&mut board);
        selector.select_prev_column(&mut board);
        selector.select_prev_column(&mut board);
        assert!(board.card(0, 0).is_selected());

        Ok(())
    }

    #[test]
    fn get_the_card_index() -> Result<()> {
        let mut board = Board::open("res/test_board.json")?;
        let mut selector = CardSelector::new();

        assert_eq!(None, selector.get());
        selector.select_next_card(&mut board);
        assert_eq!(Some((0, 0)), selector.get());

        selector.select_next_column(&mut board);
        selector.select_next_column(&mut board);
        selector.select_next_card(&mut board);
        assert_eq!(Some((2, 1)), selector.get());

        selector.select_next_column(&mut board);
        selector.select_next_card(&mut board);
        assert_eq!(Some((2, 1)), selector.get());

        selector.disable_selection(&mut board);
        assert_eq!(None, selector.get());

        Ok(())
    }

    #[test]
    fn set_the_card_index() -> Result<()> {
        let mut board = Board::open("res/test_board_with_empty_column.json")?;
        let mut selector = CardSelector::new();
        selector.select_next_card(&mut board);

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
            selector.set(column_index, card_index, &board);

            let output = selector.get().unwrap();
            assert_eq!(expected, output);
        }

        Ok(())
    }

    #[test]
    fn returns_none_on_empty_board() -> Result<()> {
        let mut board = Board::open("res/test_board_with_empty_column.json")?;
        let mut selector = CardSelector::new();

        selector.select_next_column(&mut board);
        selector.select_next_column(&mut board);
        assert_eq!(None, selector.get_selected_card(&board));

        Ok(())
    }
}
