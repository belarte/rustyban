use std::borrow::Borrow;

use chrono::{DateTime, Local};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Card {
    short_description: String,

    creation_date: DateTime<Local>,

    #[serde(skip)]
    is_selected: bool,
}

impl Card {
    pub fn new(short_description: &str, creation_date: DateTime<Local>) -> Self {
        Card {
            short_description: short_description.into(),
            creation_date,
            is_selected: false,
        }
    }

    pub fn short_description(&self) -> &String {
        &self.short_description
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn select(mut card: Card) -> Self {
        card.is_selected = true;
        card
    }

    pub fn deselect(mut card: Card) -> Self {
        card.is_selected = false;
        card
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use chrono::Local;

    use super::Card;

    #[test]
    fn selection() -> Result<()> {
        let card = Card::new("test", Local::now());
        assert_eq!(false, card.is_selected());

        let card = Card::deselect(card);
        assert_eq!(false, card.is_selected());

        let card = Card::select(card);
        assert_eq!(true, card.is_selected());

        let card = Card::select(card);
        assert_eq!(true, card.is_selected());

        let card = Card::deselect(card);
        assert_eq!(false, card.is_selected());

        Ok(())
    }
}

impl Widget for &Card {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border = if self.is_selected {
            border::DOUBLE
        } else {
            border::ROUNDED
        };

        let block = Block::bordered()
            .border_set(border);

        let text = Text::from(vec![
            Line::from(self.short_description.borrow()),
            Line::from(self.creation_date.format("%Y-%m-%d %H:%M").to_string())
        ]);

        Paragraph::new(text)
            .block(block)
            .render(area, buf);
    }
}

