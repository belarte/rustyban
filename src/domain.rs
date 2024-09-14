use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Board {
    pub columns: Vec<Column>,
}

impl Board {
    pub fn new() -> Self {
        let mut todo = Column::new("TODO");
        todo.add_card(Card::new("Buy milk"));
        todo.add_card(Card::new("Buy eggs"));
        todo.add_card(Card::new("Buy bread"));

        let mut doing = Column::new("Doing");
        doing.add_card(Card::new("Cook dinner"));

        let mut done = Column::new("Done!");
        done.add_card(Card::new("Eat dinner"));
        done.add_card(Card::new("Wash dishes"));

        Board { columns: vec![todo, doing, done] }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Card {
    pub short_description: String,
    pub creation_date: DateTime<Utc>,
}

impl Card {
    fn new(short_description: &str) -> Self {
        Card {
            short_description: short_description.into(),
            creation_date: Utc::now(),
        }
    }
}

