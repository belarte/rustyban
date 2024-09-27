use std::io::Result;

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
use crate::app::Save;
use crate::board::Board;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Normal,
    Save,
    Help,
}

#[derive(Debug)]
pub struct App<'a> {
    pub state: State,
    file_name: String,
    pub save_to_file: Save<'a>,
    logger: Logger,
    board: Board,
    pub exit: bool,
}

impl App<'_> {
    pub fn new(file_name: String) -> Self {
        let mut logger = Logger::new();
        let board = if !file_name.is_empty() {
            match Board::open(&file_name) {
                Ok(board) => board,
                Err(e) => {
                    logger.log(format!("Cannot read file {} because {}, creating a new board", file_name, e));
                    Board::new()
                }
            }
        } else {
            logger.log(format!("No file to open, creating a new board"));
            Board::new()
        };

        App {
            file_name,
            logger,
            board,
            save_to_file: Save::new(),
            state: State::Normal,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            event_handler::handle_events(self)?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn write(&mut self) {
        match self.board.to_file(&self.file_name) {
            Ok(_) => self.log(format!("Board written to {}", self.file_name)),
            Err(e) => self.log(format!("Error writing to file: {}", e)),
        }
    }

    pub fn write_to_file(&mut self, file_name: String) {
        self.file_name = file_name;
        self.write();
    }

    fn log(&mut self, msg: String) {
        self.logger.log(msg);
    }
}

impl Widget for &App<'_> {
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

        if self.state == State::Help {
            Help.render(area, buf);
        }

        if self.state == State::Save {
            self.save_to_file.render(area, buf);
        }
    }
}
