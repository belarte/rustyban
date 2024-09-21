use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        Block, Clear, Paragraph, Widget
    },
};

pub struct Help;

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
