use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    widgets::{block::Title, Block, Widget},
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
    pub fn new(label: &str, text: &str, constraint: Constraint, selected: bool) -> Self {
        let vec: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
        let mut text_area = TextArea::new(vec);
        text_area.move_cursor(tui_textarea::CursorMove::End);

        Self {
            label: label.to_string(),
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
        let title = Title::from(format!(" {} ", self.label).bold());
        let block = Block::bordered()
            .title(title)
            .on_dark_gray()
            .border_set(if self.selected { border::DOUBLE } else { border::PLAIN });

        let inner_area = block.inner(area);
        block.render(area, buf);
        self.text_area.render(inner_area, buf);
    }
}
