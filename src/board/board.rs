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

    pub fn insert_card(&mut self, column_index: usize, card_index: usize, card: Card) {
        self.columns[column_index].insert_card(card, card_index);
    }

    pub fn select_card(&mut self, column_index: usize, card_index: usize) {
        self.columns[column_index].select_card(card_index);
    }

    pub fn deselect_card(&mut self, column_index: usize, card_index: usize) {
        self.columns[column_index].deselect_card(card_index);
    }

    pub fn update_card(&mut self, column_index: usize, card_index: usize, card: Card) {
        self.columns[column_index].update_card(card_index, card);
    }

    pub fn increase_priority(&mut self, column_index: usize, card_index: usize) {
        self.columns[column_index].increase_priority(card_index);
    }

    pub fn decrease_priority(&mut self, column_index: usize, card_index: usize) {
        self.columns[column_index].decrease_priority(card_index);
    }

    pub fn mark_card_done(&mut self, column_index: usize, card_index: usize) -> bool {
        if column_index >= self.columns.len() - 1 {
            return false;
        }

        let card = self.columns(column_index).get_card(card_index).clone();
        self.columns[column_index].remove_card(card_index);
        self.columns[column_index + 1].insert_card(card, 0);
        true
    }

    pub fn mark_card_undone(&mut self, column_index: usize, card_index: usize) -> bool {
        if column_index == 0 {
            return false;
        }

        let card = self.columns(column_index).get_card(card_index).clone();
        self.columns[column_index].remove_card(card_index);
        self.columns[column_index - 1].insert_card(card, 0);
        true
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

    use chrono::Local;

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

    #[test]
    fn marking_card_done() -> Result<()> {
        let board = Board::open("res/test_board.json")?;

        let cases: Vec<(usize, usize, bool)> = vec![
            (0, 0, true),
            (0, 1, true),
            (0, 2, true),
            (1, 0, true),
            (2, 0, false),
            (2, 1, false),
        ];

        for (column_index, card_index, expected) in cases {
            let mut board = board.clone();
            assert_eq!(expected, board.mark_card_done(column_index, card_index));
        }

        Ok(())
    }

    #[test]
    fn marking_card_undone() -> Result<()> {
        let board = Board::open("res/test_board.json")?;

        let cases: Vec<(usize, usize, bool)> = vec![
            (0, 0, false),
            (0, 1, false),
            (0, 2, false),
            (1, 0, true),
            (2, 0, true),
            (2, 1, true),
        ];

        for (column_index, card_index, expected) in cases {
            let mut board = board.clone();
            assert_eq!(expected, board.mark_card_undone(column_index, card_index));
        }

        Ok(())
    }

    #[test]
    fn inserting_card() -> Result<()> {
        let board = Board::open("res/test_board.json")?;
        let description = "new description";
        let new_card = Card::new(description, Local::now());

        let cases: Vec<(usize, usize, &str)> = vec![
            (0, 0, "Buy milk"),
            (0, 1, "Buy eggs"),
            (0, 2, "Buy bread"),
            (1, 0, "Cook dinner"),
            (2, 0, "Eat dinner"),
            (2, 1, "Wash dishes"),
        ];

        for (column_index, card_index, old_description) in cases {
            let mut board = board.clone();

            assert_eq!(old_description, board.columns(column_index).get_card(card_index).short_description());
            board.insert_card(column_index, card_index, new_card.clone());
            assert_eq!(old_description, board.columns(column_index).get_card(card_index + 1).short_description());
            assert_eq!(description, board.columns(column_index).get_card(card_index).short_description());
        }

        Ok(())
    }

    #[test]
    fn appending_card() -> Result<()> {
        let board = Board::open("res/test_board.json")?;
        let description = "new description";
        let new_card = Card::new(description, Local::now());

        let cases: Vec<(usize, usize)> = vec![
            (0, 3),
            (1, 1),
            (2, 2),
        ];

        for (column_index, card_index) in cases {
            let mut board = board.clone();

            board.insert_card(column_index, card_index, new_card.clone());
            assert_eq!(description, board.columns(column_index).get_card(card_index).short_description());
        }

        Ok(())
    }
}
