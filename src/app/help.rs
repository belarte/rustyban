use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Clear, Paragraph, Widget,
    },
};

use crate::app::widget_utils::centered_popup_area;

pub struct Help;

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_popup_area(area, Constraint::Percentage(60), Constraint::Percentage(20));
        Clear.render(area, buf);

        let title = Title::from(" Help ".bold());
        let status = Title::from(" Press any key to dismiss ");
        let text = Text::from(vec![
            Line::from(vec![" <h/j/k/l> ".bold(), "Select card".into()]),
            Line::from(vec![" <←/↓/↑/→> ".bold(), "Select card".into()]),
            Line::from(vec![" <e>  ".bold(), "Edit selected card".into()]),
            Line::from(vec![" <CR> ".bold(), "Edit selected card".into()]),
            Line::from(vec![" <w> ".bold(), "Write the board to file".into()]),
            Line::from(vec![
                " <W> ".bold(),
                "Write the board to a new file (opens pop up)".into(),
            ]),
            Line::from(vec![" <q> ".bold(), "Quit the application".into()]),
            Line::from(vec![" <?> ".bold(), "Toggle this help message".into()]),
        ]);

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(status.alignment(Alignment::Center).position(Position::Bottom))
            .on_dark_gray()
            .border_set(border::ROUNDED);
        Paragraph::new(text).block(block).render(area, buf);
    }
}
