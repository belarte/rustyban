use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{Block, Clear, Widget},
};
use tui_textarea::{Input, TextArea};

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

impl Save<'_> {
    pub fn new() -> Self {
        let block = Block::bordered()
            .title(" Enter path: ")
            .on_blue()
            .border_set(border::DOUBLE);
        let mut text_area = TextArea::default();
        text_area.set_block(block);

        return Self { text_area };
    }

    pub fn push(&mut self, input: Input) {
        self.text_area.input(input);
    }

    pub fn get(&self) -> String {
        self.text_area.lines()[0].clone()
    }

    pub fn clear(&mut self) {
        self.text_area.delete_line_by_head();
        self.text_area.delete_line_by_end();
    }
}

impl Widget for &Save<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = save_area(area);
        Clear.render(area, buf);
        self.text_area.render(area, buf);
    }
}

fn save_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(3)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
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

        save.clear();

        assert_eq!("", save.get());

        Ok(())
    }
}
