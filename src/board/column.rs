use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{block::Title, Block, Widget},
};
use serde::{Deserialize, Serialize};

use crate::board::Card;

#[derive(Debug, Deserialize, Serialize)]
pub struct Column {
    pub header: String,
    pub cards: Vec<Card>,
}

impl Column {
    pub fn new(header: &str) -> Self {
        Column { header: header.into(), cards: vec![] }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl Widget for &Column {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = format!(" {} ", self.header.clone());
        let title = Title::from(header.bold())
            .alignment(Alignment::Center);
        
        let block = Block::bordered()
            .title(title)
            .border_set(border::THICK);

        let inner_area = block.inner(area);
        let areas = Layout::vertical([Constraint::Max(4); 4]).split(inner_area);
        self.cards.iter().enumerate().for_each(|(i, card)| {
            card.render(areas[i], buf);
        });

        block.render(area, buf);
    }
}
