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

    pub fn get(&self) -> Option<(usize, usize)> {
        if self.selection_enabled {
            Some((self.selected_column, self.selected_card))
        } else {
            None
        }
    }

    pub fn select_next_column(&mut self, mut board: Board) -> Board {
        if self.selection_enabled {
            board = Board::deselect_card(board.clone(), self.selected_column, self.selected_card);
            self.selected_column = self.next_column_index(&board, self.selected_column);
            self.selected_card = board.columns(self.selected_column).get_card_index(self.selected_card);
        } else {
            self.selection_enabled = true;
        }

        Board::select_card(board, self.selected_column, self.selected_card)
    }

    pub fn select_prev_column(&mut self, mut board: Board) -> Board {
        if self.selection_enabled {
            board = Board::deselect_card(board.clone(), self.selected_column, self.selected_card);
            self.selected_column = self.prev_column_index(&board, self.selected_column);
            self.selected_card = board.columns(self.selected_column).get_card_index(self.selected_card);
        } else {
            self.selection_enabled = true;
        }

        Board::select_card(board, self.selected_column, self.selected_card)
    }

    pub fn select_next_card(&mut self, mut board: Board) -> Board {
        if self.selection_enabled {
            board = Board::deselect_card(board.clone(), self.selected_column, self.selected_card);
            self.selected_card = board.columns(self.selected_column).next_card_index(self.selected_card);
        } else {
            self.selection_enabled = true;
        }

        Board::select_card(board, self.selected_column, self.selected_card)
    }

    pub fn select_prev_card(&mut self, mut board: Board) -> Board {
        if self.selection_enabled {
            board = Board::deselect_card(board.clone(), self.selected_column, self.selected_card);
            self.selected_card = board.columns(self.selected_column).prev_card_index(self.selected_card);
        } else {
            self.selection_enabled = true;
        }

        Board::select_card(board, self.selected_column, self.selected_card)
    }

    pub fn disable_selection(&mut self, board: Board) -> Board {
        self.selection_enabled = false;
        Board::deselect_card(board.clone(), self.selected_column, self.selected_card)
    }

    fn next_column_index(&self, board: &Board, current_index: usize) -> usize {
        min(current_index + 1, board.columns_count() - 1)
    }

    fn prev_column_index(&self, board: &Board, current_index: usize) -> usize {
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
        let board = Board::open("res/test_board.json")?;
        let mut selector = CardSelector::new();

        assert_eq!(false, board.columns(0).get_card(0).is_selected());
        let board = selector.select_next_card(board);
        assert_eq!(true, board.columns(0).get_card(0).is_selected());

        assert_eq!(false, board.columns(0).get_card(1).is_selected());
        let board = selector.select_next_card(board);
        assert_eq!(true, board.columns(0).get_card(1).is_selected());

        assert_eq!(false, board.columns(0).get_card(2).is_selected());
        let board = selector.select_next_card(board);
        assert_eq!(true, board.columns(0).get_card(2).is_selected());

        assert_eq!(false, board.columns(1).get_card(0).is_selected());
        let board = selector.select_next_column(board);
        let board = selector.select_next_card(board);
        assert_eq!(true, board.columns(1).get_card(0).is_selected());

        assert_eq!(false, board.columns(2).get_card(0).is_selected());
        let board = selector.select_next_column(board);
        let board = selector.select_next_column(board);
        let board = selector.select_prev_card(board);
        assert_eq!(true, board.columns(2).get_card(0).is_selected());

        assert_eq!(false, board.columns(0).get_card(0).is_selected());
        let board = selector.select_prev_column(board);
        let board = selector.select_prev_column(board);
        let board = selector.select_prev_column(board);
        let board = selector.select_prev_column(board);
        assert_eq!(true, board.columns(0).get_card(0).is_selected());

        Ok(())
    }

    #[test]
    fn get_the_card_index() -> Result<()> {
        let board = Board::open("res/test_board.json")?;
        let mut selector = CardSelector::new();

        assert_eq!(None, selector.get());
        let board = selector.select_next_card(board);
        assert_eq!(Some((0, 0)), selector.get());

        let board = selector.select_next_column(board);
        let board = selector.select_next_column(board);
        let board = selector.select_next_card(board);
        assert_eq!(Some((2, 1)), selector.get());

        let board = selector.select_next_column(board);
        let board = selector.select_next_card(board);
        assert_eq!(Some((2, 1)), selector.get());

        selector.disable_selection(board);
        assert_eq!(None, selector.get());

        Ok(())
    }
}
