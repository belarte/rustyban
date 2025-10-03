use std::{
    borrow::Cow,
    fs::File,
    io::{Read, Write},
};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use serde::{Deserialize, Serialize};

use crate::core::card::Card;
use crate::core::column::Column;
use crate::core::{Result, RustybanError};
use crate::domain::constants::layout;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Board {
    columns: Vec<Column>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

/// A Kanban board containing multiple columns of cards.
///
/// The board represents a complete Kanban workflow with three default columns:
/// "TODO", "Doing", and "Done!". It provides methods for managing cards across
/// columns, including insertion, removal, movement, and persistence to JSON files.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use rustyban::{Board, Card};
/// use chrono::Local;
/// use std::borrow::Cow;
///
/// # fn main() -> rustyban::Result<()> {
/// // Create a new board with default columns
/// let mut board = Board::new();
/// assert_eq!(board.columns_count(), 3);
///
/// // Add cards to the TODO column (index 0)
/// let now = Local::now();
/// let card1 = Card::new("Implement feature", now);
/// let card2 = Card::new("Write tests", now);
///
/// board.insert_card(0, 0, Cow::Owned(card1))?;
/// board.insert_card(0, 1, Cow::Owned(card2))?;
///
/// // Move a card to the next column by marking it done
/// let (new_col, new_idx) = board.mark_card_done(0, 0);
/// assert_eq!(new_col, 1); // Moved to "Doing" column
/// # Ok(())
/// # }
/// ```
///
/// ## Persistence
///
/// ```rust,no_run
/// use rustyban::Board;
///
/// # fn main() -> rustyban::Result<()> {
/// // Save a board to file
/// let board = Board::new();
/// board.to_file("my_board.json")?;
///
/// // Load a board from file
/// let loaded_board = Board::open("my_board.json")?;
/// # Ok(())
/// # }
/// ```
impl Board {
    /// Creates a new board with three default columns: "TODO", "Doing", and "Done!".
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(board.columns_count(), 3);
    /// assert_eq!(board.column(0).unwrap().header(), "TODO");
    /// assert_eq!(board.column(1).unwrap().header(), "Doing");
    /// assert_eq!(board.column(2).unwrap().header(), "Done!");
    /// ```
    pub fn new() -> Self {
        let todo = Column::new("TODO", vec![]);
        let doing = Column::new("Doing", vec![]);
        let done = Column::new("Done!", vec![]);

        Board {
            columns: vec![todo, doing, done],
        }
    }

    /// Loads a board from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be opened or read
    /// - The file content is not valid JSON
    /// - The JSON doesn't match the expected board structure
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rustyban::Board;
    ///
    /// # fn main() -> rustyban::Result<()> {
    /// let board = Board::open("my_board.json")?;
    /// println!("Loaded board with {} columns", board.columns_count());
    /// # Ok(())
    /// # }
    /// ```
    pub fn open(file_name: &str) -> Result<Self> {
        let mut content = String::new();
        let mut file = File::open(file_name)?;
        file.read_to_string(&mut content)?;

        match serde_json::from_str(&content) {
            Ok(board) => Ok(board),
            Err(e) => Err(e.into()),
        }
    }

    /// Saves the board to a JSON file.
    ///
    /// The file will be created if it doesn't exist, or overwritten if it does.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be created or written to
    /// - The board cannot be serialized to JSON
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rustyban::{Board, Card};
    /// use chrono::Local;
    /// use std::borrow::Cow;
    ///
    /// # fn main() -> rustyban::Result<()> {
    /// let mut board = Board::new();
    /// let card = Card::new("Save this board", Local::now());
    /// board.insert_card(0, 0, Cow::Owned(card))?;
    ///
    /// board.to_file("my_project.json")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_file(&self, file_name: &str) -> Result<()> {
        let content = self.to_json_string()?;

        let mut file = File::create(file_name)
            .map_err(RustybanError::Io)?;
        file.write_all(content.as_bytes())
            .map_err(RustybanError::Io)?;
        
        Ok(())
    }

    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string_pretty(&self)
            .map_err(RustybanError::Serialization)
    }

    /// Get a column by index, returning None if out of bounds
    pub fn column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    /// Get a card by column and card index, returning None if out of bounds
    pub fn card(&self, column_index: usize, card_index: usize) -> Option<&Card> {
        self.columns.get(column_index)?.card(card_index)
    }

    pub fn columns_count(&self) -> usize {
        self.columns.len()
    }

    /// Insert a card with bounds checking
    pub fn insert_card(&mut self, column_index: usize, card_index: usize, card: Cow<Card>) -> Result<()> {
        if column_index >= self.columns.len() {
            return Err(RustybanError::IndexOutOfBounds { 
                index: column_index, 
                max: self.columns.len().saturating_sub(1) 
            });
        }
        self.columns[column_index].insert_card(card.into_owned(), card_index);
        Ok(())
    }

    /// Remove a card with bounds checking
    pub fn remove_card(&mut self, column_index: usize, card_index: usize) -> Result<(usize, usize)> {
        if column_index >= self.columns.len() {
            return Err(RustybanError::IndexOutOfBounds { 
                index: column_index, 
                max: self.columns.len().saturating_sub(1) 
            });
        }
        let card_index = self.columns[column_index].remove_card(card_index);
        Ok((column_index, card_index))
    }

    /// Select a card with bounds checking
    pub fn select_card(&mut self, column_index: usize, card_index: usize) -> Result<()> {
        if column_index >= self.columns.len() {
            return Err(RustybanError::IndexOutOfBounds { 
                index: column_index, 
                max: self.columns.len().saturating_sub(1) 
            });
        }
        self.columns[column_index].select_card(card_index);
        Ok(())
    }

    /// Deselect a card with bounds checking
    pub fn deselect_card(&mut self, column_index: usize, card_index: usize) -> Result<()> {
        if column_index >= self.columns.len() {
            return Err(RustybanError::IndexOutOfBounds { 
                index: column_index, 
                max: self.columns.len().saturating_sub(1) 
            });
        }
        self.columns[column_index].deselect_card(card_index);
        Ok(())
    }

    /// Update a card with bounds checking
    pub fn update_card(&mut self, column_index: usize, card_index: usize, card: Cow<Card>) -> Result<()> {
        if column_index >= self.columns.len() {
            return Err(RustybanError::IndexOutOfBounds { 
                index: column_index, 
                max: self.columns.len().saturating_sub(1) 
            });
        }
        self.columns[column_index].update_card(card_index, card.into_owned());
        Ok(())
    }

    pub fn increase_priority(&mut self, column_index: usize, card_index: usize) -> (usize, usize) {
        let card_index = self.columns[column_index].increase_priority(card_index);
        (column_index, card_index)
    }

    pub fn decrease_priority(&mut self, column_index: usize, card_index: usize) -> (usize, usize) {
        let card_index = self.columns[column_index].decrease_priority(card_index);
        (column_index, card_index)
    }

    pub fn mark_card_done(&mut self, column_index: usize, card_index: usize) -> (usize, usize) {
        if column_index >= self.columns.len() - 1 {
            return (column_index, card_index);
        }

        if let Some(card) = self.columns[column_index].take_card(card_index) {
            self.columns[column_index + 1].insert_card(card, 0);
        }

        (column_index + 1, 0)
    }

    pub fn mark_card_undone(&mut self, column_index: usize, card_index: usize) -> (usize, usize) {
        if column_index == 0 {
            return (column_index, card_index);
        }

        if let Some(card) = self.columns[column_index].take_card(card_index) {
            self.columns[column_index - 1].insert_card(card, 0);
        }

        (column_index - 1, 0)
    }
}

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, center, right] = Layout::horizontal([
            Constraint::Percentage(layout::LEFT_COLUMN_WIDTH),
            Constraint::Percentage(layout::CENTER_COLUMN_WIDTH),
            Constraint::Percentage(layout::RIGHT_COLUMN_WIDTH),
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
        let board = Board::open(path)?;

        assert_eq!("TODO", board.columns[0].header());
        assert_eq!("Buy milk", board.columns[0].card(0).unwrap().short_description());
        assert_eq!("Buy eggs", board.columns[0].card(1).unwrap().short_description());
        assert_eq!("Buy bread", board.columns[0].card(2).unwrap().short_description());
        assert_eq!("Doing", board.columns[1].header());
        assert_eq!("Cook dinner", board.columns[1].card(0).unwrap().short_description());
        assert_eq!("Done!", board.columns[2].header());
        assert_eq!("Eat dinner", board.columns[2].card(0).unwrap().short_description());
        assert_eq!("Wash dishes", board.columns[2].card(1).unwrap().short_description());

        Ok(())
    }

    #[test]
    fn safe_access_methods() -> Result<()> {
        let path = "res/test_board.json";
        let board = Board::open(path)?;

        // Test safe column access
        assert!(board.column(0).is_some());
        assert!(board.column(1).is_some());
        assert!(board.column(2).is_some());
        assert!(board.column(3).is_none()); // Out of bounds
        assert!(board.column(999).is_none()); // Way out of bounds

        // Test safe card access
        assert!(board.card(0, 0).is_some());
        assert!(board.card(0, 1).is_some());
        assert!(board.card(0, 2).is_some());
        assert!(board.card(0, 3).is_none()); // Out of bounds
        assert!(board.card(3, 0).is_none()); // Column out of bounds
        assert!(board.card(999, 999).is_none()); // Both out of bounds

        Ok(())
    }

    #[test]
    fn safe_operations_with_bounds_checking() -> Result<()> {
        let path = "res/test_board.json";
        let mut board = Board::open(path)?;

        // Test safe operations within bounds
        let card = Card::new("Test card", Local::now());
        assert!(board.insert_card(0, 0, Cow::Borrowed(&card)).is_ok());
        assert!(board.select_card(0, 0).is_ok());
        assert!(board.update_card(0, 0, Cow::Borrowed(&card)).is_ok());
        assert!(board.deselect_card(0, 0).is_ok());
        assert!(board.remove_card(0, 0).is_ok());

        // Test safe operations out of bounds
        assert!(board.insert_card(999, 0, Cow::Borrowed(&card)).is_err());
        assert!(board.select_card(999, 0).is_err());
        assert!(board.update_card(999, 0, Cow::Borrowed(&card)).is_err());
        assert!(board.deselect_card(999, 0).is_err());
        assert!(board.remove_card(999, 0).is_err());

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
    fn increasing_priority() -> Result<()> {
        let cases: Vec<((usize, usize), (usize, usize))> = vec![((0, 0), (0, 0)), ((0, 1), (0, 0)), ((0, 2), (0, 1))];

        for ((column_index, card_index), expected) in cases {
            let mut board = Board::open("res/test_board.json")?;
            assert_eq!(expected, board.increase_priority(column_index, card_index));
        }

        Ok(())
    }

    #[test]
    fn decreasing_priority() -> Result<()> {
        let cases: Vec<((usize, usize), (usize, usize))> = vec![((0, 0), (0, 1)), ((0, 1), (0, 2)), ((0, 2), (0, 2))];

        for ((column_index, card_index), expected) in cases {
            let mut board = Board::open("res/test_board.json")?;
            assert_eq!(expected, board.decrease_priority(column_index, card_index));
        }

        Ok(())
    }

    #[test]
    fn marking_card_done() -> Result<()> {
        let cases: Vec<((usize, usize), (usize, usize))> = vec![
            ((0, 0), (1, 0)),
            ((0, 1), (1, 0)),
            ((0, 2), (1, 0)),
            ((1, 0), (2, 0)),
            ((2, 0), (2, 0)),
            ((2, 1), (2, 1)),
        ];

        for ((column_index, card_index), expected) in cases {
            let mut board = Board::open("res/test_board.json")?;
            assert_eq!(expected, board.mark_card_done(column_index, card_index));
        }

        Ok(())
    }

    #[test]
    fn marking_card_undone() -> Result<()> {
        let cases: Vec<((usize, usize), (usize, usize))> = vec![
            ((0, 0), (0, 0)),
            ((0, 1), (0, 1)),
            ((0, 2), (0, 2)),
            ((1, 0), (0, 0)),
            ((2, 0), (1, 0)),
            ((2, 1), (1, 0)),
        ];

        for ((column_index, card_index), expected) in cases {
            let mut board = Board::open("res/test_board.json")?;
            assert_eq!(expected, board.mark_card_undone(column_index, card_index));
        }

        Ok(())
    }

    #[test]
    fn inserting_card() -> Result<()> {
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
            let mut board = Board::open("res/test_board.json")?;

            assert_eq!(
                old_description,
                board.card(column_index, card_index).unwrap().short_description()
            );
            let _ = board.insert_card(column_index, card_index, Cow::Borrowed(&new_card));
            assert_eq!(
                old_description,
                board.card(column_index, card_index + 1).unwrap().short_description()
            );
            assert_eq!(description, board.card(column_index, card_index).unwrap().short_description());
        }

        Ok(())
    }

    #[test]
    fn appending_card() -> Result<()> {
        let description = "new description";
        let new_card = Card::new(description, Local::now());

        let cases: Vec<(usize, usize)> = vec![(0, 3), (1, 1), (2, 2)];

        for (column_index, card_index) in cases {
            let mut board = Board::open("res/test_board.json")?;

            let _ = board.insert_card(column_index, card_index, Cow::Borrowed(&new_card));
            assert_eq!(description, board.card(column_index, card_index).unwrap().short_description());
        }

        Ok(())
    }

    #[test]
    fn deleting_card() -> Result<()> {
        let mut board = Board::open("res/test_board.json")?;

        assert_eq!(3, board.column(0).unwrap().size());
        let position = board.remove_card(0, 1).unwrap();
        assert_eq!((0, 1), position);
        assert_eq!(2, board.column(0).unwrap().size());
        let position = board.remove_card(0, 1).unwrap();
        assert_eq!((0, 0), position);
        assert_eq!(1, board.column(0).unwrap().size());
        let position = board.remove_card(0, 0).unwrap();
        assert_eq!((0, 0), position);
        assert_eq!(0, board.column(0).unwrap().size());

        Ok(())
    }
}
