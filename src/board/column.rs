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
    pub fn new(header: &str, cards: Vec<Card>) -> Self {
        Column {
            header: header.into(),
            cards,
        }
    }

    pub fn header(&self) -> &str {
        &self.header
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.len() == 0
    }

    /// Get a card by index, returning None if out of bounds
    pub fn card(&self, i: usize) -> Option<&Card> {
        self.cards.get(i)
    }

    pub fn insert_card(&mut self, card: Card, index: usize) {
        self.cards.insert(index, card);
    }

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

    pub fn select_card(&mut self, card_index: usize) {
        if !self.is_empty() {
            self.cards[card_index].select();
        }
    }

    pub fn deselect_card(&mut self, card_index: usize) {
        if !self.is_empty() {
            self.cards[card_index].deselect();
        }
    }

    pub fn update_card(&mut self, card_index: usize, card: Card) {
        if !self.is_empty() {
            self.cards[card_index] = card;
        }
    }

    pub fn increase_priority(&mut self, card_index: usize) -> usize {
        if card_index > 0 && card_index < self.cards.len() {
            let new_index = card_index - 1;
            self.cards.swap(card_index, new_index);
            return new_index;
        }

        card_index
    }

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
        let areas = Layout::vertical([Constraint::Max(4); 8]).split(inner_area);
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

    use crate::board::card::Card;

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
