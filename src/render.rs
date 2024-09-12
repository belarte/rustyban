use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Widget,
    },
};

use crate::{App, Board, Column};

impl Widget for &Column {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = format!(" {} ", self.header.clone());
        let title = Title::from(header.bold())
            .alignment(Alignment::Center);
        
        Block::bordered()
            .title(title)
            .border_set(border::THICK)
            .render(area, buf);
    }
}

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, center, right] = Layout::horizontal(
            [Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)]
        ).areas(area);

        for (column, area) in self.columns.iter().zip([left, center, right].iter()) {
            column.render(*area, buf);
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Welcome ".bold());

        let instructions = Title::from(Line::from(vec![
                " Quit ".into(),
                "<q> ".blue().bold(),
        ]));

        let block = Block::new()
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

