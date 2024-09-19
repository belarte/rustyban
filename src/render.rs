use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::Title, Block, Clear, Paragraph, Widget
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
        let [title_area, board_area, instructions_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]).areas(area);

        let title = Line::from(" Welcome ".bold())
            .centered();
        title.render(title_area, buf);

        let instructions = Line::from(vec![
                " Help ".into(),
                "<?> ".blue().bold(),
                "Quit ".into(),
                "<q> ".blue().bold(),
        ])
            .centered();
        instructions.render(instructions_area, buf);

        self.board.render(board_area, buf);

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
