use std::io;

use ratatui::{
    DefaultTerminal, Frame,
};

mod render;
mod event_handler;

#[derive(Debug)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App { exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            event_handler::handle_events(self)?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }


    fn exit(&mut self) {
        self.exit = true;
    }
}

