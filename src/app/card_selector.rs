use std::cmp::min;

use crate::board::Board;

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

    pub fn select_next_column(&mut self, board: &mut Board) -> (usize, usize) {
        if self.selection_enabled {
            self.deselect_card(board, self.selected_column, self.selected_card);
            self.selected_column = self.next_column_index(board, self.selected_column);
            self.selected_card = board.columns(self.selected_column).get_card_index(self.selected_card);
        } else {
            self.selection_enabled = true;
        }

        self.select_card(board, self.selected_column, self.selected_card);
        (self.selected_column, self.selected_card)
    }

    pub fn select_prev_column(&mut self, board: &mut Board) -> (usize, usize) {
        if self.selection_enabled {
            self.deselect_card(board, self.selected_column, self.selected_card);
            self.selected_column = self.prev_column_index(board, self.selected_column);
            self.selected_card = board.columns(self.selected_column).get_card_index(self.selected_card);
        } else {
            self.selection_enabled = true;
        }

        self.select_card(board, self.selected_column, self.selected_card);
        (self.selected_column, self.selected_card)
    }

    pub fn select_next_card(&mut self, board: &mut Board) -> (usize, usize) {
        if self.selection_enabled {
            self.deselect_card(board, self.selected_column, self.selected_card);
            self.selected_card = board.columns(self.selected_column).next_card_index(self.selected_card);
        } else {
            self.selection_enabled = true;
        }

        self.select_card(board, self.selected_column, self.selected_card);
        (self.selected_column, self.selected_card)
    }

    pub fn select_prev_card(&mut self, board: &mut Board) -> (usize, usize) {
        if self.selection_enabled {
            self.deselect_card(board, self.selected_column, self.selected_card);
            self.selected_card = board.columns(self.selected_column).prev_card_index(self.selected_card);
        } else {
            self.selection_enabled = true;
        }

        self.select_card(board, self.selected_column, self.selected_card);
        (self.selected_column, self.selected_card)
    }

    pub fn disable_selection(&mut self, board: &mut Board) {
        self.selection_enabled = false;
        self.deselect_card(board, self.selected_column, self.selected_card);
    }

    fn select_card(&mut self, board: &mut Board, column_index: usize, card_index: usize) {
        board.columns(column_index).select_card(card_index);
    }

    fn deselect_card(&mut self, board: &mut Board, column_index: usize, card_index: usize) {
        board.columns(column_index).deselect_card(card_index);
    }

    fn next_column_index(&self, board: &mut Board, current_index: usize) -> usize {
        min(current_index + 1, board.columns_count() - 1)
    }

    fn prev_column_index(&self, board: &mut Board, current_index: usize) -> usize {
        if current_index == 0 {
            return 0
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

        assert_eq!(false, board.columns(0).get_card(0).is_selected());
        selector.select_next_card(&mut board);
        assert_eq!(true, board.columns(0).get_card(0).is_selected());

        assert_eq!(false, board.columns(0).get_card(1).is_selected());
        selector.select_next_card(&mut board);
        assert_eq!(true, board.columns(0).get_card(1).is_selected());

        assert_eq!(false, board.columns(0).get_card(2).is_selected());
        selector.select_next_card(&mut board);
        assert_eq!(true, board.columns(0).get_card(2).is_selected());

        assert_eq!(false, board.columns(1).get_card(0).is_selected());
        selector.select_next_column(&mut board);
        selector.select_next_card(&mut board);
        assert_eq!(true, board.columns(1).get_card(0).is_selected());

        assert_eq!(false, board.columns(2).get_card(0).is_selected());
        selector.select_next_column(&mut board);
        selector.select_next_column(&mut board);
        selector.select_prev_card(&mut board);
        assert_eq!(true, board.columns(2).get_card(0).is_selected());

        assert_eq!(false, board.columns(0).get_card(0).is_selected());
        selector.select_prev_column(&mut board);
        selector.select_prev_column(&mut board);
        selector.select_prev_column(&mut board);
        selector.select_prev_column(&mut board);
        assert_eq!(true, board.columns(0).get_card(0).is_selected());

        Ok(())
    }
}
