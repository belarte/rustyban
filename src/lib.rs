use std::io;

use ratatui::{
    DefaultTerminal, Frame,
};

mod domain;
mod render;
mod event_handler;

#[derive(Debug)]
pub struct App {
    board: domain::Board,
    show_help: bool,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            board: domain::Board::new(),
            show_help: false,
            exit: false,
        }
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

    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

