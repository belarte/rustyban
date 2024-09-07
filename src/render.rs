use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
};

use crate::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Welcome ".bold());

        let instructions = Title::from(Line::from(vec![
                " Quit ".into(),
                "<q> ".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
                "This is a start.".into(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

