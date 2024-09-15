use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title}, Block, Clear, Paragraph, Widget
    },
};

use crate::{domain::{Board, Card, Column}, App };

impl Widget for &Card {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_set(border::ROUNDED);

        let text = Text::from(vec![
            Line::from(self.short_description.clone()),
            Line::from(self.creation_date.format("%Y-%m-%d %H:%M").to_string())
        ]);

        Paragraph::new(text)
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &Column {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = format!(" {} ", self.header.clone());
        let title = Title::from(header.bold())
            .alignment(Alignment::Center);
        
        let block = Block::bordered()
            .title(title)
            .border_set(border::THICK);

        let inner_area = block.inner(area);
        let areas = Layout::vertical([Constraint::Max(4); 4]).split(inner_area);
        self.cards.iter().enumerate().for_each(|(i, card)| {
            card.render(areas[i], buf);
        });

        block.render(area, buf);
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
                " Help ".into(),
                "<?> ".blue().bold(),
                "Quit ".into(),
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

        if self.show_help {
            Help.render(area, buf);
        }
    }
}

struct Help;

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = help_area(area, 60, 20);
        Clear.render(area, buf);

        let title = Line::from(" Help ".bold());
        let text = Text::from(vec![
            Line::from(vec![
                " Quit: ".bold(),
                "Press <q> to quit the application".into()
            ]),
            Line::from(vec![
                " Help: ".bold(),
                "Press <?> to toggle this help message".into()
            ]),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .on_dark_gray()
            .border_set(border::ROUNDED);
        Paragraph::new(text)
            .block(block)
            .render(area, buf);
    }
}

fn help_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
