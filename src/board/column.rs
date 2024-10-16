use std::cmp::min;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{block::Title, Block, Widget},
};
use serde::{Deserialize, Serialize};

use crate::board::Card;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Column {
    header: String,
    cards: Vec<Card>,
}

impl Column {
    pub fn new(header: &str) -> Self {
        Column {
            header: header.into(),
            cards: vec![],
        }
    }

    pub fn header(&self) -> &str {
        &self.header
    }

    pub fn get_card(&self, i: usize) -> &Card {
        &self.cards[i]
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn get_card_index(&self, index: usize) -> usize {
        min(index, self.cards.len() - 1)
    }

    pub fn next_card_index(&self, current_index: usize) -> usize {
        self.get_card_index(current_index + 1)
    }

    pub fn prev_card_index(&self, current_index: usize) -> usize {
        if current_index == 0 {
            return 0;
        }

        self.get_card_index(current_index - 1)
    }

    pub fn select_card(mut column: Column, card_index: usize) -> Column {
        let card = Card::select(column.cards[card_index].clone());
        column.cards[card_index] = card;
        column
    }

    pub fn deselect_card(mut column: Column, card_index: usize) -> Column {
        let card = Card::deselect(column.cards[card_index].clone());
        column.cards[card_index] = card;
        column
    }
}

impl Widget for &Column {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = format!(" {} ", self.header);
        let title = Title::from(header.bold()).alignment(Alignment::Center);

        let block = Block::bordered().title(title).border_set(border::THICK);

        let inner_area = block.inner(area);
        let areas = Layout::vertical([Constraint::Max(4); 4]).split(inner_area);
        self.cards.iter().enumerate().for_each(|(i, card)| {
            card.render(areas[i], buf);
        });

        block.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use crate::board::card::Card;

    use super::Column;

    #[test]
    fn selection() -> Result<()> {
        let mut column = Column::new("test");

        column.add_card(Card::default());
        column.add_card(Card::default());
        column.add_card(Card::default());

        assert_eq!(1, column.next_card_index(0));
        assert_eq!(2, column.next_card_index(1));
        assert_eq!(2, column.next_card_index(2));
        assert_eq!(2, column.next_card_index(999));

        assert_eq!(0, column.prev_card_index(0));
        assert_eq!(0, column.prev_card_index(1));
        assert_eq!(1, column.prev_card_index(2));
        assert_eq!(2, column.prev_card_index(999));

        Ok(())
    }
}
