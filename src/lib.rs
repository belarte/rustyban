use std::io::{self, Write};

use ratatui::{
    DefaultTerminal, Frame,
};

mod domain;
mod render;
mod event_handler;

#[derive(Debug)]
pub struct Logger {
    counter: u32,
    message: String,
}

impl Logger {
    fn new() -> Self {
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

#[derive(Debug)]
pub struct App {
    file_name: String,
    logger: Logger,
    board: domain::Board,
    show_help: bool,
    exit: bool,
}

impl App {
    pub fn new(file_name: String) -> Self {
        App {
            file_name,
            logger: Logger::new(),
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

    fn to_file(&mut self) {
        let content = self.board.to_json_string().expect("Cannot write file");

        let file = std::fs::File::create(&self.file_name);
        match file {
            Ok(mut file) => {
                match file.write_all(content.as_bytes()) {
                    Ok(_) => self.log(format!("Board written to {}", self.file_name)),
                    Err(e) => self.log(format!("Error writing to file: {}", e)),
                }
            }
            Err(e) => {
                self.log(format!("Error creating file {}: {}", self.file_name, e));
                return;
            }
        }
    }

    fn log(&mut self, msg: String) {
        self.logger.log(msg);
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

        let mut app = App::new(path.into());
        app.to_file();

        assert!(fs::metadata(path).is_ok());

        let _ = fs::remove_file(path);

        Ok(())
    }

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
