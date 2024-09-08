use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
};

use crate::{App, Board};

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Rendering the board...")
            .centered()
            .render(area, buf);
    }
}

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

        let inner_area = block.inner(area);

        block.render(area, buf);
        self.board.render(inner_area, buf);
    }
}

