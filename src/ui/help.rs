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

use crate::ui::widget_utils::centered_popup_area;

pub struct Help;

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_popup_area(area, Constraint::Length(60), Constraint::Length(20));
        Clear.render(area, buf);

        let title = Title::from(" Help ".bold());
        let status = Title::from(" Press any key to dismiss ");
        let text = Text::from(vec![
            Line::from(vec![" <h/j/k/l> ".bold(), "Select card".into()]),
            Line::from(vec![" <←/↓/↑/→> ".bold(), "Select card".into()]),
            Line::from(vec![" <e>  ".bold(), "Edit selected card".into()]),
            Line::from(vec![" <CR> ".bold(), "Edit selected card".into()]),
            Line::from(vec![" <i> ".bold(), "Insert card a current position".into()]),
            Line::from(vec![" <I> ".bold(), "Insert card at the top of current column".into()]),
            Line::from(vec![" <a> ".bold(), "Insert card a next position".into()]),
            Line::from(vec![
                " <A> ".bold(),
                "Insert card at the bottom of current clumn".into(),
            ]),
            Line::from(vec![" <x>   ".bold(), "Delete current card".into()]),
            Line::from(vec![" <DEL> ".bold(), "Delete current card".into()]),
            Line::from(vec![" <K> ".bold(), "Increase priotity of selected card".into()]),
            Line::from(vec![" <J> ".bold(), "Decrease priotity of selected card".into()]),
            Line::from(vec![" <L> ".bold(), "Mark selected card done".into()]),
            Line::from(vec![" <H> ".bold(), "Mark selected card undone".into()]),
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
