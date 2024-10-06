use std::borrow::Borrow;

use chrono::{DateTime, Local};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    short_description: String,
    creation_date: DateTime<Local>,
}

impl Card {
    pub fn new(short_description: &str, creation_date: DateTime<Local>) -> Self {
        Card {
            short_description: short_description.into(),
            creation_date,
        }
    }

    pub fn short_description(&self) -> &String {
        &self.short_description
    }
}

impl Widget for &Card {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_set(border::ROUNDED);

        let text = Text::from(vec![
            Line::from(self.short_description.borrow()),
            Line::from(self.creation_date.format("%Y-%m-%d %H:%M").to_string())
        ]);

        Paragraph::new(text)
            .block(block)
            .render(area, buf);
    }
}

