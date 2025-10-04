use crate::domain::services::Logger;
use crate::engine::logger::Logger as ConcreteLogger;
use ratatui::widgets::Widget;

/// Concrete implementation of Logger trait wrapping the existing Logger struct
#[derive(Debug)]
pub struct ConcreteLoggerWrapper {
    inner: ConcreteLogger,
}

impl ConcreteLoggerWrapper {
    pub fn new() -> Self {
        Self {
            inner: ConcreteLogger::new(),
        }
    }
}

impl Logger for ConcreteLoggerWrapper {
    fn log(&mut self, message: &str) {
        self.inner.log(message);
    }

    fn render(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        self.inner.render(area, buf);
    }
}
