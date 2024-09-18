use std::error::Error;

use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Board {
    pub columns: Vec<Column>,
}

impl Board {
    pub fn new() -> Self {
        let now = Local::now();

        let mut todo = Column::new("TODO");
        todo.add_card(Card::new("Buy milk", now));
        todo.add_card(Card::new("Buy eggs", now));
        todo.add_card(Card::new("Buy bread", now));

        let mut doing = Column::new("Doing");
        doing.add_card(Card::new("Cook dinner", now));

        let mut done = Column::new("Done!");
        done.add_card(Card::new("Eat dinner", now));
        done.add_card(Card::new("Wash dishes", now));

        Board { columns: vec![todo, doing, done] }
    }

    pub fn to_json_string(&self) -> Result<String, Box<dyn Error>> {
        return match serde_json::to_string_pretty(&self) {
            Ok(res) => Ok(res),
            Err(_) => Err("Failed to serialize board".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_to_json_string() -> Result<(), Box<dyn Error>> {
        let board = Board::new();
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

#[derive(Debug, Serialize)]
pub struct Column {
    pub header: String,
    pub cards: Vec<Card>,
}

impl Column {
    fn new(header: &str) -> Self {
        Column { header: header.into(), cards: vec![] }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

#[derive(Debug, Serialize)]
pub struct Card {
    pub short_description: String,
    pub creation_date: DateTime<Local>,
}

impl Card {
    fn new(short_description: &str, creation_date: DateTime<Local>) -> Self {
        Card {
            short_description: short_description.into(),
            creation_date,
        }
    }
}

