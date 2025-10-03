use std::borrow::Borrow;

use chrono::{DateTime, Local};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};

use crate::utils::time;

/// A Kanban card representing a task or work item.
///
/// Cards are the fundamental unit of work in a Kanban board. Each card has a short description
/// (title), an optional long description (details), and tracks when it was created. Cards can
/// be selected for UI interaction and are serializable to JSON for persistence.
///
/// # Examples
///
/// ```rust
/// use rustyban::Card;
/// use chrono::Local;
///
/// // Create a new card
/// let now = Local::now();
/// let mut card = Card::new("Fix login bug", now);
///
/// // Update the card with more details
/// card.update_long_description("The login form doesn't validate email addresses properly");
///
/// // Access card information
/// assert_eq!(card.short_description(), "Fix login bug");
/// assert_eq!(card.creation_date(), &now);
/// assert!(!card.is_selected());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Card {
    short_description: String,

    long_description: String,

    creation_date: DateTime<Local>,

    #[serde(skip)]
    is_selected: bool,
}

impl Card {
    /// Creates a new card with the given short description and creation date.
    ///
    /// The card is created with an empty long description and is not selected by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let now = Local::now();
    /// let card = Card::new("Implement feature X", now);
    /// 
    /// assert_eq!(card.short_description(), "Implement feature X");
    /// assert_eq!(card.long_description(), "");
    /// assert!(!card.is_selected());
    /// ```
    pub fn new(short_description: &str, creation_date: DateTime<Local>) -> Self {
        Card {
            short_description: short_description.into(),
            long_description: "".into(),
            creation_date,
            is_selected: false,
        }
    }

    /// Returns the card's short description (title).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let card = Card::new("Review pull request", Local::now());
    /// assert_eq!(card.short_description(), "Review pull request");
    /// ```
    pub fn short_description(&self) -> &String {
        &self.short_description
    }

    /// Returns the card's long description (detailed information).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let mut card = Card::new("Fix bug", Local::now());
    /// card.update_long_description("The submit button doesn't work on mobile");
    /// 
    /// assert_eq!(card.long_description(), "The submit button doesn't work on mobile");
    /// ```
    pub fn long_description(&self) -> &String {
        &self.long_description
    }

    /// Returns the card's creation date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let now = Local::now();
    /// let card = Card::new("Task", now);
    /// 
    /// assert_eq!(card.creation_date(), &now);
    /// ```
    pub fn creation_date(&self) -> &DateTime<Local> {
        &self.creation_date
    }

    /// Returns whether the card is currently selected in the UI.
    ///
    /// Selected cards are highlighted differently in the terminal interface.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let mut card = Card::new("Task", Local::now());
    /// assert!(!card.is_selected());
    /// 
    /// card.select();
    /// assert!(card.is_selected());
    /// ```
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    /// Updates the card's short description (title).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let mut card = Card::new("Old title", Local::now());
    /// card.update_short_description("New title");
    /// 
    /// assert_eq!(card.short_description(), "New title");
    /// ```
    pub fn update_short_description(&mut self, short_description: &str) {
        self.short_description = short_description.into();
    }

    /// Updates the card's long description (detailed information).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let mut card = Card::new("Task", Local::now());
    /// card.update_long_description("Detailed task description");
    /// 
    /// assert_eq!(card.long_description(), "Detailed task description");
    /// ```
    pub fn update_long_description(&mut self, long_description: &str) {
        self.long_description = long_description.into();
    }

    /// Marks the card as selected for UI highlighting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let mut card = Card::new("Task", Local::now());
    /// assert!(!card.is_selected());
    /// 
    /// card.select();
    /// assert!(card.is_selected());
    /// ```
    pub fn select(&mut self) {
        self.is_selected = true;
    }

    /// Marks the card as not selected, removing UI highlighting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::Card;
    /// use chrono::Local;
    ///
    /// let mut card = Card::new("Task", Local::now());
    /// card.select();
    /// assert!(card.is_selected());
    /// 
    /// card.deselect();
    /// assert!(!card.is_selected());
    /// ```
    pub fn deselect(&mut self) {
        self.is_selected = false;
    }
}

impl Widget for &Card {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border = if self.is_selected {
            border::DOUBLE
        } else {
            border::ROUNDED
        };

        let block = Block::bordered().border_set(border);
        let now = Local::now();

        let text = Text::from(vec![
            Line::from(self.short_description.borrow()),
            Line::from(time::pretty_diff(self.creation_date, now)).alignment(Alignment::Right),
        ]);

        Paragraph::new(text).block(block).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use chrono::Local;

    use super::Card;

    #[test]
    fn selection() -> Result<()> {
        let mut card = Card::new("test", Local::now());
        assert!(!card.is_selected());

        card.deselect();
        assert!(!card.is_selected());

        card.select();
        assert!(card.is_selected());

        card.select();
        assert!(card.is_selected());

        card.deselect();
        assert!(!card.is_selected());

        Ok(())
    }
}
