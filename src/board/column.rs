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

    pub fn is_empty(&self) -> bool {
        self.cards.len() == 0
    }

    pub fn get_card(&self, i: usize) -> &Card {
        &self.cards[i]
    }

    pub fn get_card_index(&self, index: usize) -> usize {
        if self.is_empty() {
            return 0;
        }

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

    pub fn insert_card(mut column: Column, card: Card, index: usize) -> Column {
        column.cards.insert(index, card);
        column
    }

    pub fn remove_card(mut column: Column, index: usize) -> Column {
        column.cards.remove(index);
        column
    }

    pub fn select_card(mut column: Column, card_index: usize) -> Column {
        if !column.is_empty() {
            column.cards[card_index].select();
        }
        column
    }

    pub fn deselect_card(mut column: Column, card_index: usize) -> Column {
        if !column.is_empty() {
            column.cards[card_index].deselect();
        }
        column
    }

    pub fn update_card(mut column: Column, card_index: usize, card: Card) -> Column {
        if !column.is_empty() {
            column.cards[card_index] = card;
        }
        column
    }

    pub fn increase_priority(mut column: Column, card_index: usize) -> Column {
        if card_index > 0 && card_index < column.cards.len() {
            column.cards.swap(card_index, card_index - 1);
        }
        column
    }

    pub fn decrease_priority(mut column: Column, card_index: usize) -> Column {
        if card_index < column.cards.len() - 1 {
            column.cards.swap(card_index, card_index + 1);
        }
        column
    }
}

impl Widget for &Column {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = format!(" {} ", self.header);
        let title = Title::from(header.bold()).alignment(Alignment::Center);

        let block = Block::bordered().title(title).border_set(border::THICK);

        let inner_area = block.inner(area);
        let areas = Layout::vertical([Constraint::Max(4); 12]).split(inner_area);
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
    fn selection() -> Result<()> {
        let column = Column::new("test", vec![Card::default(), Card::default(), Card::default()]);

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

    #[test]
    fn insert_and_remove_cards() -> Result<()> {
        let now = Local::now();
        let column = Column::new("test", vec![]);

        let column = Column::insert_card(column, Card::new("card 3", now), 0);
        let column = Column::insert_card(column, Card::new("card 1", now), 0);
        let column = Column::insert_card(column, Card::new("card 2", now), 1);
        let column = Column::insert_card(column, Card::new("card 4", now), 3);

        assert_eq!(4, column.cards.len());
        assert_eq!("card 1", column.get_card(0).short_description());
        assert_eq!("card 2", column.get_card(1).short_description());
        assert_eq!("card 3", column.get_card(2).short_description());
        assert_eq!("card 4", column.get_card(3).short_description());

        let column = Column::remove_card(column, 0);
        assert_eq!(3, column.cards.len());
        assert_eq!("card 2", column.get_card(0).short_description());
        assert_eq!("card 3", column.get_card(1).short_description());
        assert_eq!("card 4", column.get_card(2).short_description());

        let column = Column::remove_card(column, 2);
        assert_eq!(2, column.cards.len());
        assert_eq!("card 2", column.get_card(0).short_description());
        assert_eq!("card 3", column.get_card(1).short_description());

        let column = Column::remove_card(column, 1);
        assert_eq!(1, column.cards.len());
        assert_eq!("card 2", column.get_card(0).short_description());

        assert!(!column.is_empty());
        let column = Column::remove_card(column, 0);
        assert!(column.is_empty());

        Ok(())
    }

    #[test]
    fn change_priority() -> Result<()> {
        let now = Local::now();
        let column = Column::new(
            "test",
            vec![
                Card::new("card 1", now),
                Card::new("card 2", now),
                Card::new("card 3", now),
            ],
        );

        let column = Column::increase_priority(column, 0);
        let column = Column::increase_priority(column, 2);
        let column = Column::increase_priority(column, 1);
        let column = Column::increase_priority(column, 2);

        assert_eq!("card 3", column.get_card(0).short_description());
        assert_eq!("card 2", column.get_card(1).short_description());
        assert_eq!("card 1", column.get_card(2).short_description());

        let column = Column::decrease_priority(column, 2);
        let column = Column::decrease_priority(column, 1);
        let column = Column::decrease_priority(column, 0);
        let column = Column::decrease_priority(column, 1);

        assert_eq!("card 1", column.get_card(0).short_description());
        assert_eq!("card 2", column.get_card(1).short_description());
        assert_eq!("card 3", column.get_card(2).short_description());

        Ok(())
    }
}
