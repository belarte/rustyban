use std::cmp::min;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{block::Title, Block, Widget},
};
use serde::{Deserialize, Serialize};

use crate::core::card::Card;
use crate::domain::constants::layout;

/// A Kanban column containing a collection of cards.
///
/// Columns represent the different stages of work in a Kanban board (e.g., "To Do", "In Progress", "Done").
/// Each column has a header (title) and contains an ordered list of cards. Columns provide methods
/// for managing cards including insertion, removal, and reordering.
///
/// # Examples
///
/// ```rust
/// use rustyban::{Column, Card};
/// use chrono::Local;
///
/// // Create a new column
/// let mut column = Column::new("To Do", vec![]);
/// assert_eq!(column.header(), "To Do");
/// assert!(column.is_empty());
///
/// // Add some cards
/// let now = Local::now();
/// let card1 = Card::new("Task 1", now);
/// let card2 = Card::new("Task 2", now);
///
/// column.insert_card(card1, 0);
/// column.insert_card(card2, 1);
///
/// assert_eq!(column.size(), 2);
/// assert!(!column.is_empty());
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Column {
    header: String,
    cards: Vec<Card>,
}

impl Column {
    /// Creates a new column with the given header and cards.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// // Create an empty column
    /// let column = Column::new("In Progress", vec![]);
    /// assert_eq!(column.header(), "In Progress");
    /// assert!(column.is_empty());
    ///
    /// // Create a column with initial cards
    /// let now = Local::now();
    /// let cards = vec![
    ///     Card::new("Task 1", now),
    ///     Card::new("Task 2", now),
    /// ];
    /// let column = Column::new("Done", cards);
    /// assert_eq!(column.size(), 2);
    /// ```
    pub fn new(header: &str, cards: Vec<Card>) -> Self {
        Column {
            header: header.into(),
            cards,
        }
    }

    /// Returns the column's header (title).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Column;
    ///
    /// let column = Column::new("Backlog", vec![]);
    /// assert_eq!(column.header(), "Backlog");
    /// ```
    pub fn header(&self) -> &str {
        &self.header
    }

    /// Returns the number of cards in the column.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// assert_eq!(column.size(), 0);
    ///
    /// let card = Card::new("New task", Local::now());
    /// column.insert_card(card, 0);
    /// assert_eq!(column.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.cards.len()
    }

    /// Returns whether the column contains no cards.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// assert!(column.is_empty());
    ///
    /// let card = Card::new("New task", Local::now());
    /// column.insert_card(card, 0);
    /// assert!(!column.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cards.len() == 0
    }

    /// Gets a reference to the card at the specified index.
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let card = Card::new("Task", Local::now());
    /// column.insert_card(card, 0);
    ///
    /// assert!(column.card(0).is_some());
    /// assert!(column.card(1).is_none());
    /// ```
    pub fn card(&self, i: usize) -> Option<&Card> {
        self.cards.get(i)
    }

    /// Inserts a card at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let card1 = Card::new("First task", Local::now());
    /// let card2 = Card::new("Second task", Local::now());
    ///
    /// column.insert_card(card1, 0);
    /// column.insert_card(card2, 0); // Insert at beginning
    ///
    /// assert_eq!(column.size(), 2);
    /// assert_eq!(column.card(0).unwrap().short_description(), "Second task");
    /// ```
    pub fn insert_card(&mut self, card: Card, index: usize) {
        self.cards.insert(index, card);
    }

    /// Removes the card at the specified index and returns the new suggested index.
    ///
    /// Returns the index where the cursor should be positioned after removal.
    /// If the column becomes empty, returns 0. Otherwise, returns the minimum of
    /// the original index and the new last valid index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds and the column is not empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let now = Local::now();
    /// column.insert_card(Card::new("Task 1", now), 0);
    /// column.insert_card(Card::new("Task 2", now), 1);
    ///
    /// let new_index = column.remove_card(0);
    /// assert_eq!(column.size(), 1);
    /// assert_eq!(new_index, 0);
    /// ```
    pub fn remove_card(&mut self, index: usize) -> usize {
        if self.cards.is_empty() {
            return 0;
        }

        self.cards.remove(index);

        if self.is_empty() {
            0
        } else {
            min(index, self.cards.len() - 1)
        }
    }

    /// Removes and returns the card at the specified index.
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let now = Local::now();
    /// column.insert_card(Card::new("Task", now), 0);
    ///
    /// let taken_card = column.take_card(0);
    /// assert!(taken_card.is_some());
    /// assert_eq!(taken_card.unwrap().short_description(), "Task");
    /// assert!(column.is_empty());
    ///
    /// // Out of bounds returns None
    /// assert!(column.take_card(0).is_none());
    /// ```
    pub fn take_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    /// Marks the card at the specified index as selected.
    ///
    /// Does nothing if the column is empty.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds and the column is not empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let now = Local::now();
    /// column.insert_card(Card::new("Task", now), 0);
    ///
    /// column.select_card(0);
    /// assert!(column.card(0).unwrap().is_selected());
    /// ```
    pub fn select_card(&mut self, card_index: usize) {
        if !self.is_empty() {
            self.cards[card_index].select();
        }
    }

    /// Marks the card at the specified index as not selected.
    ///
    /// Does nothing if the column is empty.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds and the column is not empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let now = Local::now();
    /// column.insert_card(Card::new("Task", now), 0);
    ///
    /// column.select_card(0);
    /// assert!(column.card(0).unwrap().is_selected());
    ///
    /// column.deselect_card(0);
    /// assert!(!column.card(0).unwrap().is_selected());
    /// ```
    pub fn deselect_card(&mut self, card_index: usize) {
        if !self.is_empty() {
            self.cards[card_index].deselect();
        }
    }

    /// Replaces the card at the specified index with a new card.
    ///
    /// Does nothing if the column is empty.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds and the column is not empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let now = Local::now();
    /// column.insert_card(Card::new("Old task", now), 0);
    ///
    /// let new_card = Card::new("New task", now);
    /// column.update_card(0, new_card);
    ///
    /// assert_eq!(column.card(0).unwrap().short_description(), "New task");
    /// ```
    pub fn update_card(&mut self, card_index: usize, card: Card) {
        if !self.is_empty() {
            self.cards[card_index] = card;
        }
    }

    /// Increases the priority of the card at the specified index by moving it up one position.
    ///
    /// Returns the new index of the card after the move. If the card is already at the top
    /// or the index is invalid, returns the original index unchanged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let now = Local::now();
    /// column.insert_card(Card::new("Task 1", now), 0);
    /// column.insert_card(Card::new("Task 2", now), 1);
    /// column.insert_card(Card::new("Task 3", now), 2);
    ///
    /// // Move "Task 3" up one position
    /// let new_index = column.increase_priority(2);
    /// assert_eq!(new_index, 1);
    /// assert_eq!(column.card(1).unwrap().short_description(), "Task 3");
    /// ```
    pub fn increase_priority(&mut self, card_index: usize) -> usize {
        if card_index > 0 && card_index < self.cards.len() {
            let new_index = card_index - 1;
            self.cards.swap(card_index, new_index);
            return new_index;
        }

        card_index
    }

    /// Decreases the priority of the card at the specified index by moving it down one position.
    ///
    /// Returns the new index of the card after the move. If the card is already at the bottom
    /// or the index is invalid, returns the original index unchanged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::{Column, Card};
    /// use chrono::Local;
    ///
    /// let mut column = Column::new("To Do", vec![]);
    /// let now = Local::now();
    /// column.insert_card(Card::new("Task 1", now), 0);
    /// column.insert_card(Card::new("Task 2", now), 1);
    /// column.insert_card(Card::new("Task 3", now), 2);
    ///
    /// // Move "Task 1" down one position
    /// let new_index = column.decrease_priority(0);
    /// assert_eq!(new_index, 1);
    /// assert_eq!(column.card(1).unwrap().short_description(), "Task 1");
    /// ```
    pub fn decrease_priority(&mut self, card_index: usize) -> usize {
        if card_index < self.cards.len() - 1 {
            let new_index = card_index + 1;
            self.cards.swap(card_index, card_index + 1);
            return new_index;
        }

        card_index
    }
}

impl Widget for &Column {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = format!(" {} ", self.header);
        let title = Title::from(header.bold()).alignment(Alignment::Center);

        let block = Block::bordered().title(title).border_set(border::THICK);

        let inner_area = block.inner(area);
        let areas = Layout::vertical([Constraint::Max(layout::MAX_CARD_HEIGHT); layout::MAX_CARDS_PER_COLUMN]).split(inner_area);
        self.cards.iter().enumerate().for_each(|(i, card)| {
            card.render(areas[i], buf);
        });

        block.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use chrono::Local;

    use crate::core::card::Card;

    use super::Column;

    #[test]
    fn insert_and_remove_cards() -> Result<()> {
        let now = Local::now();
        let mut column = Column::new("test", vec![]);

        column.insert_card(Card::new("card 3", now), 0);
        column.insert_card(Card::new("card 1", now), 0);
        column.insert_card(Card::new("card 2", now), 1);
        column.insert_card(Card::new("card 4", now), 3);

        assert_eq!(4, column.cards.len());
        assert_eq!("card 1", column.card(0).unwrap().short_description());
        assert_eq!("card 2", column.card(1).unwrap().short_description());
        assert_eq!("card 3", column.card(2).unwrap().short_description());
        assert_eq!("card 4", column.card(3).unwrap().short_description());

        let index = column.remove_card(0);
        assert_eq!(0, index);
        assert_eq!(3, column.cards.len());
        assert_eq!("card 2", column.card(0).unwrap().short_description());
        assert_eq!("card 3", column.card(1).unwrap().short_description());
        assert_eq!("card 4", column.card(2).unwrap().short_description());

        let index = column.remove_card(2);
        assert_eq!(1, index);
        assert_eq!(2, column.cards.len());
        assert_eq!("card 2", column.card(0).unwrap().short_description());
        assert_eq!("card 3", column.card(1).unwrap().short_description());

        let index = column.remove_card(1);
        assert_eq!(0, index);
        assert_eq!(1, column.cards.len());
        assert_eq!("card 2", column.card(0).unwrap().short_description());

        assert!(!column.is_empty());
        let index = column.remove_card(0);
        assert_eq!(0, index);
        assert!(column.is_empty());

        let index = column.remove_card(0);
        assert_eq!(0, index);

        Ok(())
    }

    #[test]
    fn change_priority() -> Result<()> {
        let now = Local::now();
        let mut column = Column::new(
            "test",
            vec![
                Card::new("card 1", now),
                Card::new("card 2", now),
                Card::new("card 3", now),
            ],
        );

        column.increase_priority(0);
        column.increase_priority(2);
        column.increase_priority(1);
        column.increase_priority(2);

        assert_eq!("card 3", column.card(0).unwrap().short_description());
        assert_eq!("card 2", column.card(1).unwrap().short_description());
        assert_eq!("card 1", column.card(2).unwrap().short_description());

        column.decrease_priority(2);
        column.decrease_priority(1);
        column.decrease_priority(0);
        column.decrease_priority(1);

        assert_eq!("card 1", column.card(0).unwrap().short_description());
        assert_eq!("card 2", column.card(1).unwrap().short_description());
        assert_eq!("card 3", column.card(2).unwrap().short_description());

        Ok(())
    }

    #[test]
    fn safe_card_access() -> Result<()> {
        let now = Local::now();
        let column = Column::new("test", vec![
            Card::new("card 1", now),
            Card::new("card 2", now),
            Card::new("card 3", now),
        ]);

        // Test safe access within bounds
        assert!(column.card(0).is_some());
        assert!(column.card(1).is_some());
        assert!(column.card(2).is_some());
        
        // Test safe access out of bounds
        assert!(column.card(3).is_none());
        assert!(column.card(999).is_none());

        Ok(())
    }
}
