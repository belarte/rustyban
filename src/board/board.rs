use std::{
    fs::File,
    io::{Read, Result, Write},
};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use serde::{Deserialize, Serialize};

use crate::board::{Card, Column};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Board {
    columns: Vec<Column>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let todo = Column::new("TODO", vec![]);
        let doing = Column::new("Doing", vec![]);
        let done = Column::new("Done!", vec![]);

        Board {
            columns: vec![todo, doing, done],
        }
    }

    pub fn open(file_name: &str) -> Result<Self> {
        let mut content = String::new();
        let mut file = File::open(file_name)?;
        file.read_to_string(&mut content)?;

        match serde_json::from_str(&content) {
            Ok(board) => Ok(board),
            Err(e) => Err(e.into()),
        }
    }

    pub fn to_file(&self, file_name: &str) -> Result<()> {
        let content = self.to_json_string().expect("Cannot write file");

        let file = File::create(file_name);
        match file {
            Ok(mut file) => file.write_all(content.as_bytes()),
            Err(e) => Err(e),
        }
    }

    fn to_json_string(&self) -> Result<String> {
        match serde_json::to_string_pretty(&self) {
            Ok(res) => Ok(res),
            Err(e) => Err(e.into()),
        }
    }

    pub fn columns(&self, index: usize) -> &Column {
        &self.columns[index]
    }

    pub fn columns_count(&self) -> usize {
        self.columns.len()
    }

    pub fn select_card(mut board: Board, column_index: usize, card_index: usize) -> Board {
        let column = Column::select_card(board.columns[column_index].clone(), card_index);
        board.columns[column_index] = column;
        board
    }

    pub fn deselect_card(mut board: Board, column_index: usize, card_index: usize) -> Board {
        let column = Column::deselect_card(board.columns[column_index].clone(), card_index);
        board.columns[column_index] = column;
        board
    }

    pub fn update_card(mut board: Board, column_index: usize, card_index: usize, card: Card) -> Board {
        let column = Column::update_card(board.columns[column_index].clone(), card_index, card);
        board.columns[column_index] = column;
        board
    }

    pub fn increase_priority(mut board: Board, column_index: usize, card_index: usize) -> Board {
        let column = Column::increase_priority(board.columns[column_index].clone(), card_index);
        board.columns[column_index] = column;
        board
    }

    pub fn decrease_priority(mut board: Board, column_index: usize, card_index: usize) -> Board {
        let column = Column::decrease_priority(board.columns[column_index].clone(), card_index);
        board.columns[column_index] = column;
        board
    }
}

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, center, right] = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .areas(area);

        for (column, area) in self.columns.iter().zip([left, center, right].iter()) {
            column.render(*area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn open_board() -> Result<()> {
        let path = "res/test_board.json";
        let board = Board::open(path).expect("Cannot open board");

        assert_eq!("TODO", board.columns[0].header());
        assert_eq!("Buy milk", board.columns[0].get_card(0).short_description());
        assert_eq!("Buy eggs", board.columns[0].get_card(1).short_description());
        assert_eq!("Buy bread", board.columns[0].get_card(2).short_description());
        assert_eq!("Doing", board.columns[1].header());
        assert_eq!("Cook dinner", board.columns[1].get_card(0).short_description());
        assert_eq!("Done!", board.columns[2].header());
        assert_eq!("Eat dinner", board.columns[2].get_card(0).short_description());
        assert_eq!("Wash dishes", board.columns[2].get_card(1).short_description());

        Ok(())
    }

    #[test]
    fn write_board_to_file() -> Result<()> {
        let path = "board.txt";
        let _ = fs::remove_file(path);

        let board = Board::new();
        let res = board.to_file(path);

        assert!(res.is_ok());
        assert!(fs::metadata(path).is_ok());

        let _ = fs::remove_file(path);

        Ok(())
    }

    #[test]
    fn board_to_json_string() -> Result<()> {
        let board = Board::open("res/test_board.json")?;
        let result = board.to_json_string()?;

        assert!(result.contains("TODO"));
        assert!(result.contains("Buy milk"));
        assert!(result.contains("Buy eggs"));
        assert!(result.contains("Buy bread"));

        assert!(result.contains("Doing"));
        assert!(result.contains("Cook dinner"));

        assert!(result.contains("Done!"));
        assert!(result.contains("Eat dinner"));
        assert!(result.contains("Wash dishes"));

        Ok(())
    }
}
