use std::{fs::File, io::{Read, Result, Write}};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget
};
use serde::{Deserialize, Serialize};

use crate::board::Column;

#[derive(Debug, Deserialize, Serialize)]
pub struct Board {
    pub columns: Vec<Column>,
}

impl Board {
    pub fn new() -> Self {
        let todo = Column::new("TODO");
        let doing = Column::new("Doing");
        let done = Column::new("Done!");

        Board { columns: vec![todo, doing, done] }
    }

    pub fn open(file_name: &str) -> Result<Self> {
        let mut content = String::new();
        let mut file = File::open(file_name)?;
        file.read_to_string(&mut content)?;

        return match serde_json::from_str(&content) {
            Ok(board) => Ok(board),
            Err(e) => Err(e.into()),
        }
    }

    pub fn to_file(&mut self, file_name: &str) -> Result<()>  {
        let content = self.to_json_string().expect("Cannot write file");

        let file = File::create(file_name);
        match file {
            Ok(mut file) => file.write_all(content.as_bytes()),
            Err(e) => Err(e),
        }
    }

    fn to_json_string(&self) -> Result<String> {
        return match serde_json::to_string_pretty(&self) {
            Ok(res) => Ok(res),
            Err(e) => Err(e.into()),
        }
    }
}

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, center, right] = Layout::horizontal(
            [Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)]
        ).areas(area);

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

        assert_eq!("TODO", board.columns[0].header);
        assert_eq!("Buy milk", board.columns[0].cards[0].short_description);
        assert_eq!("Buy eggs", board.columns[0].cards[1].short_description);
        assert_eq!("Buy bread", board.columns[0].cards[2].short_description);
        assert_eq!("Doing", board.columns[1].header);
        assert_eq!("Cook dinner", board.columns[1].cards[0].short_description);
        assert_eq!("Done!", board.columns[2].header);
        assert_eq!("Eat dinner", board.columns[2].cards[0].short_description);
        assert_eq!("Wash dishes", board.columns[2].cards[1].short_description);

        Ok(())
    }

    #[test]
    fn write_board_to_file() -> Result<()> {
        let path = "board.txt";
        let _ = fs::remove_file(path);

        let mut board = Board::new();
        let res = board.to_file(path.into());

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