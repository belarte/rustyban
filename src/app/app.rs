use std::{cell::RefCell, cmp::min, rc::Rc};

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
    board: Rc<RefCell<Board>>,
    selector: CardSelector,
    pub exit: bool,
}

pub enum InsertPosition {
    Current,
    Next,
    Top,
    Bottom,
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

        let board = Rc::new(RefCell::new(board));
        let selector = CardSelector::new(Rc::clone(&board));

        App {
            file_name,
            logger,
            board,
            selector,
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn select_next_column(&mut self) {
        self.card_selection(|this| this.selector.select_next_column())
    }

    pub fn select_prev_column(&mut self) {
        self.card_selection(|this| this.selector.select_prev_column())
    }

    pub fn select_next_card(&mut self) {
        self.card_selection(|this| this.selector.select_next_card())
    }

    pub fn select_prev_card(&mut self) {
        self.card_selection(|this| this.selector.select_prev_card())
    }

    pub fn disable_selection(&mut self) {
        if let Some((column_index, card_index)) = self.selector.get() {
            let mut board = self.board.as_ref().borrow_mut();
            board.deselect_card(column_index, card_index);
        }

        self.selector.disable_selection();
    }

    pub fn get_selected_card(&self) -> Option<Card> {
        self.selector.get_selected_card()
    }

    pub fn update_card(&mut self, card: Card) {
        self.with_selected_card(|this, column, card_index| {
            let mut board = this.board.as_ref().borrow_mut();
            board.update_card(column, card_index, card.clone());
        });
    }

    pub fn insert_card(&mut self, position: InsertPosition) -> Option<Card> {
        self.with_selected_card(|this, column_index, card_index| {
            this.board.as_ref().borrow_mut().deselect_card(column_index, card_index);

            let card_index = match position {
                InsertPosition::Current => card_index,
                InsertPosition::Next => card_index + 1,
                InsertPosition::Top => 0,
                InsertPosition::Bottom => this.board.as_ref().borrow().column(column_index).size(),
            };

            this.board
                .as_ref()
                .borrow_mut()
                .insert_card(column_index, card_index, Card::new("TODO", Local::now()));
            this.board.as_ref().borrow_mut().select_card(column_index, card_index);
            this.selector.set(column_index, card_index);
        });

        self.get_selected_card()
    }

    pub fn remove_card(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            if this.board.as_ref().borrow().column(column_index).is_empty() {
                return;
            }

            this.select_next_card();
            this.board.as_ref().borrow_mut().remove_card(column_index, card_index);
            this.selector.set(column_index, card_index);
        });
    }

    pub fn increase_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .increase_priority(column_index, card_index);
            this.select_prev_card();
        });
    }

    pub fn decrease_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board
                .as_ref()
                .borrow_mut()
                .decrease_priority(column_index, card_index);
            this.select_next_card();
        });
    }

    pub fn mark_card_done(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            if this
                .board
                .as_ref()
                .borrow_mut()
                .mark_card_done(column_index, card_index)
            {
                let new_index = min(column_index + 1, 2);
                this.selector.set(new_index, 0);
            }
        });
    }

    pub fn mark_card_undone(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            if this
                .board
                .as_ref()
                .borrow_mut()
                .mark_card_undone(column_index, card_index)
            {
                let new_index = if column_index > 0 { column_index - 1 } else { 0 };
                this.selector.set(new_index, 0);
            }
        });
    }

    pub fn write(&mut self) {
        let board = self.board.as_ref().borrow().clone();
        match board.to_file(&self.file_name) {
            Ok(_) => self.log(format!("Board written to {}", self.file_name)),
            Err(e) => self.log(format!("Error writing to file: {}", e)),
        }
    }

    pub fn write_to_file(&mut self, file_name: String) {
        self.file_name = file_name;
        self.write();
    }

    fn with_selected_card<F>(&mut self, mut action: F)
    where
        F: FnMut(&mut Self, usize, usize),
    {
        match self.selector.get() {
            Some((column_index, card_index)) => action(self, column_index, card_index),
            None => self.log("No card selected".to_string()),
        }
    }

    fn card_selection<F>(&mut self, mut action: F)
    where
        F: FnMut(&mut Self) -> (usize, usize),
    {
        if let Some((column_index, card_index)) = self.selector.get() {
            self.board.as_ref().borrow_mut().deselect_card(column_index, card_index);
        }

        let (column_index, card_index) = action(self);
        self.board.as_ref().borrow_mut().select_card(column_index, card_index);
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

        self.board.as_ref().borrow().render(board_area, buf);
        self.logger.render(logger_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use crate::app::app::InsertPosition;

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
    fn insertion_does_nothing_when_no_card_selected() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());

        assert_eq!(None, app.insert_card(InsertPosition::Current));

        Ok(())
    }

    #[test]
    fn insertion_at_current_position() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());

        app.select_next_card();
        app.select_next_card();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        let card = app.insert_card(InsertPosition::Current).unwrap();
        assert_eq!("TODO", card.short_description());

        {
            let board = app.board.as_ref().borrow();
            let card = board.card(0, 3);
            assert!(!card.is_selected());
            let card = board.card(0, 2);
            assert!(card.is_selected());
        }

        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        Ok(())
    }

    #[test]
    fn insertion_at_top() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());

        app.select_next_card();
        app.select_next_card();
        app.select_next_card();
        let card = app.get_selected_card().unwrap();
        assert_eq!("Buy bread", card.short_description());

        assert_eq!("Buy milk", app.board.as_ref().borrow().card(0, 0).short_description());
        let card = app.insert_card(InsertPosition::Top).unwrap();
        assert_eq!("TODO", card.short_description());
        assert_eq!("TODO", app.board.as_ref().borrow().card(0, 0).short_description());
        let card = app.get_selected_card().unwrap();
        assert_eq!("TODO", card.short_description());

        Ok(())
    }

    #[test]
    fn deletion() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());

        app.select_next_column();
        app.select_next_column();
        app.remove_card();
        app.remove_card();

        Ok(())
    }
}
