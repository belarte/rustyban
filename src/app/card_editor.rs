use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{Block, Clear, Widget},
};
use tui_textarea::TextArea;

use crate::board::Card;

#[derive(Debug, Clone)]
pub struct CardEditor<'a> {
    text_areas: Vec<TextArea<'a>>,
}

impl PartialEq for CardEditor<'_> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for CardEditor<'_> {}

impl CardEditor<'_> {
    pub fn new(card: Card) -> Self {
        let short_description_block = Block::bordered()
            .title(" Short description: ")
            .on_dark_gray()
            .border_set(border::PLAIN);
        let long_description_block = Block::bordered()
            .title(" Long description: ")
            .on_dark_gray()
            .border_set(border::PLAIN);

        let mut text_areas = vec![
            TextArea::new(vec![card.short_description().to_string()]),
            TextArea::new(vec![card.long_description().to_string()]),
        ];

        text_areas[0].set_block(short_description_block);
        text_areas[1].set_block(long_description_block);

        Self { text_areas }
    }
}

impl Widget for &CardEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = editor_area(area);
        Clear.render(area, buf);

        let block = Block::bordered()
            .title(" Edit card ")
            .on_blue()
            .border_set(border::DOUBLE);

        let inner_area = block.inner(area);
        let [short_area, long_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(10)]).areas(inner_area);

        block.render(area, buf);
        self.text_areas[0].render(short_area, buf);
        self.text_areas[1].render(long_area, buf);
    }
}

fn editor_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(50)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(50)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
