use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    widgets::{Block, Clear, Widget},
};
use tui_textarea::{Input, TextArea};

use super::widget_utils::centered_popup_area;

#[derive(Debug, Clone)]
pub struct Save<'a> {
    text_area: TextArea<'a>,
}

impl PartialEq for Save<'_> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Save<'_> {}

impl Default for Save<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Save<'_> {
    pub fn new() -> Self {
        let block = Block::bordered()
            .title(" Enter path: ")
            .on_blue()
            .border_set(border::DOUBLE);
        let mut text_area = TextArea::default();
        text_area.set_block(block);

        Self { text_area }
    }

    pub fn push(&mut self, input: Input) {
        self.text_area.input(input);
    }

    pub fn get(&self) -> String {
        self.text_area.lines()[0].clone()
    }
}

impl Widget for &Save<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_popup_area(area, Constraint::Length(64), Constraint::Length(3));
        Clear.render(area, buf);
        self.text_area.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use tui_textarea::Input;

    use super::Save;

    #[test]
    fn read_and_write() -> io::Result<()> {
        let mut save = Save::new();

        assert_eq!("", save.get());

        save.push(Input::from(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)));
        save.push(Input::from(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)));
        save.push(Input::from(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)));
        save.push(Input::from(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)));

        assert_eq!("test", save.get());

        Ok(())
    }
}
