use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};

#[derive(Debug)]
pub struct CardEditor {}

impl PartialEq for CardEditor {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for CardEditor {}

impl CardEditor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for &CardEditor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = editor_area(area);
        Clear.render(area, buf);

        let block = Block::bordered()
            .title(" Edit card ")
            .on_blue()
            .border_set(border::DOUBLE);

        let text = Line::from("Coming soon!");

        Paragraph::new(text).block(block).render(area, buf);
    }
}

fn editor_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(50)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(50)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
