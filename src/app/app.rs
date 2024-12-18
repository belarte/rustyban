use std::cmp::min;

use chrono::Local;
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
        self.selector.select_next_column(&mut self.board);
    }

    pub fn select_prev_column(&mut self) {
        self.selector.select_prev_column(&mut self.board);
    }

    pub fn select_next_card(&mut self) {
        self.selector.select_next_card(&mut self.board);
    }

    pub fn select_prev_card(&mut self) {
        self.selector.select_prev_card(&mut self.board);
    }

    pub fn disable_selection(&mut self) {
        self.selector.disable_selection(&mut self.board);
    }

    pub fn get_selected_card(&self) -> Option<Card> {
        self.selector.get_selected_card(&self.board)
    }

    pub fn update_card(&mut self, card: Card) {
        match self.selector.get() {
            Some((column, card_index)) => {
                self.board.update_card(column, card_index, card);
            }
            None => self.log("No card selected".to_string()),
        }
    }

    pub fn increase_priority(&mut self) {
        match self.selector.get() {
            Some((column_index, card_index)) => {
                self.board.increase_priority(column_index, card_index);
                self.selector.select_prev_card(&mut self.board);
            }
            None => self.log("No card selected".to_string()),
        }
    }

    pub fn decrease_priority(&mut self) {
        match self.selector.get() {
            Some((column_index, card_index)) => {
                self.board.decrease_priority(column_index, card_index);
                self.selector.select_next_card(&mut self.board);
            }
            None => self.log("No card selected".to_string()),
        }
    }

    pub fn mark_card_done(&mut self) {
        match self.selector.get() {
            Some((column_index, card_index)) => {
                if self.board.mark_card_done(column_index, card_index) {
                    let new_index = min(column_index + 1, 2);
                    self.selector.set(new_index, 0);
                }
            }
            None => self.log("No card selected".to_string()),
        }
    }

    pub fn insert_card(&mut self) -> Option<Card> {
        match self.selector.get() {
            Some((column_index, card_index)) => {
                self.board.deselect_card(column_index, card_index);
                self.board
                    .insert_card(column_index, card_index, Card::new("TODO", Local::now()));
                self.board.select_card(column_index, card_index);
            }
            None => self.log("No card selected".to_string()),
        };

        self.get_selected_card()
    }

    pub fn mark_card_undone(&mut self) {
        match self.selector.get() {
            Some((column_index, card_index)) => {
                if self.board.mark_card_undone(column_index, card_index) {
                    let new_index = if column_index > 0 { column_index - 1 } else { 0 };
                    self.selector.set(new_index, 0);
                }
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

#[cfg(test)]
mod tests {
    use std::io::Result;

    use super::App;

    #[test]
    fn mark_done_and_undone() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());

        app.select_next_card();
        app.select_next_card();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        app.mark_card_done();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        app.select_next_column();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Wash dishes", card.short_description());

        app.mark_card_undone();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Wash dishes", card.short_description());

        Ok(())
    }

    #[test]
    fn card_insertion() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());

        assert_eq!(None, app.insert_card());

        app.select_next_card();
        app.select_next_card();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        let card = app.insert_card().unwrap();
        assert_eq!("TODO", card.short_description());

        let card = app.board.card(0, 3);
        assert!(!card.is_selected());
        let card = app.board.card(0, 2);
        assert!(card.is_selected());

        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        Ok(())
    }
}
