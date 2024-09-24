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

pub struct Help;

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = help_area(area, 60, 20);
        Clear.render(area, buf);

        let title = Title::from(" Help ".bold());
        let status = Title::from(" Press any key to dismiss ");
        let text = Text::from(vec![
            Line::from(vec![
                " <w> ".bold(),
                "Write the board to file".into()
            ]),
            Line::from(vec![
                " <q> ".bold(),
                "Quit the application".into()
            ]),
            Line::from(vec![
                " <?> ".bold(),
                "Toggle this help message".into()
            ]),
        ]);

        let block = Block::bordered()
            .title(title
                .alignment(Alignment::Center))
            .title(status
                .alignment(Alignment::Center)
                .position(Position::Bottom))
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
