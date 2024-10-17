use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{Block, Clear, Widget},
};
use tui_textarea::{Input, TextArea};

use crate::board::Card;

#[derive(Debug, Clone)]
pub struct CardEditor<'a> {
    text_areas: Vec<TextArea<'a>>,
    selected: usize,
    card: Card,
}

impl PartialEq for CardEditor<'_> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for CardEditor<'_> {}

impl CardEditor<'_> {
    pub fn new(card: Card) -> Self {
        let mut short_description = TextArea::new(vec![card.short_description().to_string()]);
        let mut long_description = TextArea::new(vec![card.long_description().to_string()]);
        short_description.move_cursor(tui_textarea::CursorMove::End);
        long_description.move_cursor(tui_textarea::CursorMove::End);

        let text_areas = vec![
            short_description,
            long_description,
        ];

        Self {
            text_areas,
            selected: 0,
            card,
        }
    }

    pub fn input(&mut self, input: Input) {
        self.text_areas[self.selected].input(input);
    }

    pub fn next_field(&mut self) {
        self.selected = (self.selected + 1) % self.text_areas.len();
    }

    pub fn get_card(&self) -> Card {
        let card = self.card.clone();
        let short_description = self.text_areas[0].lines().join("\n");
        let long_description = self.text_areas[1].lines().join("\n");
        let card = Card::update_short_description(card, &short_description);
        
        Card::update_long_description(card, &long_description)
    }
}

fn get_block(title: String, is_selected: bool) -> Block<'static> {
    Block::bordered()
        .title(title)
        .on_dark_gray()
        .border_set(if is_selected {
            border::DOUBLE
        } else {
            border::PLAIN
        })
}

impl Widget for &CardEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = editor_area(area);
        Clear.render(area, buf);

        let block = Block::bordered()
            .title(" Edit card ")
            .on_blue()
            .border_set(border::PLAIN);

        let short_description_block = get_block(" Short description: ".into(), self.selected == 0);
        let mut short_description = self.text_areas[0].clone();
        short_description.set_block(short_description_block);

        let long_description_block = get_block(" Long description: ".into(), self.selected == 1);
        let mut long_description = self.text_areas[1].clone();
        long_description.set_block(long_description_block);

        let inner_area = block.inner(area);
        let [short_area, long_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(10)]).areas(inner_area);

        block.render(area, buf);
        short_description.render(short_area, buf);
        long_description.render(long_area, buf);
    }
}

fn editor_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(50)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(50)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
