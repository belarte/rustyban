use std::io::{self, Write};

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

    fn to_file(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();

        let content = self.board.to_json_string().expect("Cannot write file");
        file.write_all(content.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn write_board_to_file() -> io::Result<()> {
        let path = "board.txt";
        let _ = fs::remove_file(path);

        let app = App::new();
        app.to_file(path);

        assert!(fs::metadata(path).is_ok());

        let _ = fs::remove_file(path);

        Ok(())
    }
}
