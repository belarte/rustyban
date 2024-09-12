use std::io;

use ratatui::{
    DefaultTerminal, Frame,
};

mod render;
mod event_handler;

#[derive(Debug)]
pub struct App {
    board: Board,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App { board: Board::new(), exit: false }
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

#[derive(Debug)]
struct Board {
    columns: Vec<Column>,
}

impl Board {
    fn new() -> Self {
        Board { columns: vec![Column::new("TODO"), Column::new("Doing"), Column::new("Done!")] }
    }
}

#[derive(Debug)]
struct Column {
    header: String,
}

impl Column {
    fn new(header: &str) -> Self {
        Column { header: header.into() }
    }
}

