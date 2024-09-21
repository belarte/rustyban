use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{block::Title, Block, Paragraph, Widget}
};

#[derive(Debug)]
pub struct Logger {
    counter: u32,
    message: String,
}

impl Logger {
    pub fn new() -> Self {
        Self { counter: 0, message: String::new() }
    }

    pub fn log(&mut self, msg: String) {
        self.counter += 1;
        self.message = format!("[{}] {}", self.counter, msg)
    }

    pub fn show(&self) -> &str {
        &self.message
    }
}

impl Widget for &Logger {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Logs ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Left))
            .border_set(border::THICK);

        let message = Line::from(vec![" ".into(), self.show().into()]);

        Paragraph::new(message)
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger() -> Result<(), Box<dyn std::error::Error>> {
        let mut logger = Logger::new();

        logger.log("Hello".into());
        assert_eq!("[1] Hello", logger.show());

        logger.log("Hello again".into());
        assert_eq!("[2] Hello again", logger.show());

        logger.log("One more time for the road".into());
        assert_eq!("[3] One more time for the road", logger.show());

        Ok(())
    }
}
