use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Widget,
};

use crate::app::Logger;
use crate::board::Board;
use crate::{app::CardSelector, board::Card};

#[derive(Debug)]
pub struct App {
    file_name: String,
    logger: Logger,
    board: Board,
    selector: CardSelector,
    pub exit: bool,
}

impl App {
    pub fn new(file_name: String) -> Self {
        let mut logger = Logger::new();
        let board = if !file_name.is_empty() {
            match Board::open(&file_name) {
                Ok(board) => board,
                Err(e) => {
                    logger.log(format!(
                        "Cannot read file {} because {}, creating a new board",
                        file_name, e
                    ));
                    Board::new()
                }
            }
        } else {
            logger.log("No file to open, creating a new board".to_string());
            Board::new()
        };

        App {
            file_name,
            logger,
            board,
            selector: CardSelector::new(),
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn select_next_column(&mut self) {
        self.board = self.selector.select_next_column(self.board.clone());
    }

    pub fn select_prev_column(&mut self) {
        self.board = self.selector.select_prev_column(self.board.clone());
    }

    pub fn select_next_card(&mut self) {
        self.board = self.selector.select_next_card(self.board.clone());
    }

    pub fn select_prev_card(&mut self) {
        self.board = self.selector.select_prev_card(self.board.clone());
    }

    pub fn disable_selection(&mut self) {
        self.board = self.selector.disable_selection(self.board.clone());
    }

    pub fn get_selected_card(&self) -> Option<Card> {
        self.selector.get_selected_card(self.board.clone())
    }

    pub fn update_card(&mut self, card: Card) {
        match self.selector.get() {
            Some((column, card_index)) => {
                self.board = Board::update_card(self.board.clone(), column, card_index, card);
            }
            None => self.log("No card selected".to_string()),
        }
    }

    pub fn increase_priority(&mut self) {
        match self.selector.get() {
            Some((column_index, card_index)) => {
                self.board = Board::increase_priority(self.board.clone(), column_index, card_index);
                self.board = self.selector.select_prev_card(self.board.clone());
            }
            None => self.log("No card selected".to_string()),
        }
    }

    pub fn decrease_priority(&mut self) {
        match self.selector.get() {
            Some((column_index, card_index)) => {
                self.board = Board::decrease_priority(self.board.clone(), column_index, card_index);
                self.board = self.selector.select_next_card(self.board.clone());
            }
            None => self.log("No card selected".to_string()),
        }
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_area, board_area, logger_area, instructions_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(area);

        let title = Line::from(" Welcome ".bold()).centered();
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
    }
}
