use chrono::{DateTime, Local};

#[derive(Debug)]
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

