use std::io::{self, Write};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Widget,
    DefaultTerminal,
    Frame,
};

use crate::app::Logger;
use crate::app::event_handler;
use crate::app::Help;
use crate::board::Board;

#[derive(Debug)]
pub struct App {
    file_name: String,
    logger: Logger,
    board: Board,
    show_help: bool,
    pub exit: bool,
}

impl App {
    pub fn new(file_name: String) -> Self {
        App {
            file_name,
            logger: Logger::new(),
            board: Board::new(),
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

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn to_file(&mut self) {
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_area, board_area, logger_area, instructions_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ]).areas(area);

        let title = Line::from(" Welcome ".bold())
            .centered();
        title.render(title_area, buf);

        let instructions = Line::from(vec![
                " Help ".into(),
                "<?> ".blue().bold(),
                "Quit ".into(),
                "<q> ".blue().bold(),
        ])
            .centered();
        instructions.render(instructions_area, buf);

        self.board.render(board_area, buf);
        self.logger.render(logger_area, buf);

        if self.show_help {
            Help.render(area, buf);
        }
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
}

