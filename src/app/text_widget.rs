use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    widgets::{Block, Widget}
};
use tui_textarea::{Input, TextArea};



#[derive(Debug, Clone)]
pub struct TextWidget {
    label: String,
    text_area: TextArea<'static>,
    constraint: Constraint,
    selected: bool,
}

impl TextWidget {
    pub fn new(label: String, text: String, constraint: Constraint, selected: bool) -> Self {
        let vec: Vec<String> = text
            .split('\n')
            .map(String::from)
            .collect();
        let mut text_area = TextArea::new(vec);
        text_area.move_cursor(tui_textarea::CursorMove::End);

        Self {
            label,
            text_area,
            constraint,
            selected,
        }
    }

    pub fn constaint(&self) -> Constraint {
        self.constraint
    }

    pub fn select(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn input(&mut self, input: Input) {
        self.text_area.input(input);
    }

    pub fn lines(&self) -> Vec<String> {
        self.text_area.lines().to_vec()
    }
}

impl Widget for &TextWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = format!(" {} ", self.label);
        let block = Block::bordered()
            .title(title)
            .on_dark_gray()
            .border_set(if self.selected {
                border::DOUBLE
            } else {
                border::PLAIN
            });

        let mut text_area = self.text_area.clone();
        text_area.set_block(block);
        text_area.render(area, buf);
    }
}
