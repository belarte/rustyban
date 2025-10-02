use std::borrow::Cow;
use chrono::Local;

use crate::core::Card;
use crate::domain::{InsertPosition, event_handlers::AppOperations};

use super::App;

impl AppOperations for App {
    fn update_card(&mut self, card: Card) {
        self.with_selected_card(|this, column_index, card_index| {
            let result = this.board()
                .as_ref()
                .borrow_mut()
                .update_card(column_index, card_index, Cow::Borrowed(&card));
            if let Err(e) = result {
                this.log(&format!("Failed to update card: {}", e));
            }
            (column_index, card_index)
        });
    }

    fn write_to_file(&mut self, file_name: String) {
        self.set_file_name(file_name);
        self.write();
    }

    fn select_next_column(&mut self) {
        self.card_selection(|this| this.selector_mut().select_next_column())
    }

    fn select_prev_column(&mut self) {
        self.card_selection(|this| this.selector_mut().select_prev_column())
    }

    fn select_next_card(&mut self) {
        self.card_selection(|this| this.selector_mut().select_next_card())
    }

    fn select_prev_card(&mut self) {
        self.card_selection(|this| this.selector_mut().select_prev_card())
    }

    fn disable_selection(&mut self) {
        if let Some((column_index, card_index)) = self.selector().get() {
            let result = self.board().as_ref().borrow_mut().deselect_card(column_index, card_index);
            if let Err(e) = result {
                self.log(&format!("Failed to deselect card: {}", e));
            }
        }

        self.selector_mut().disable_selection();
    }

    fn get_selected_card(&self) -> Option<Card> {
        self.selector().get_selected_card()
    }

    fn insert_card(&mut self, position: InsertPosition) -> Option<Card> {
        if let Some((column_index, card_index)) = self.selector().get() {
            let card_index = match position {
                InsertPosition::Current => card_index,
                InsertPosition::Next => card_index + 1,
                InsertPosition::Top => 0,
                InsertPosition::Bottom => self.board().as_ref().borrow().column(column_index).map(|c| c.size()).unwrap_or(0),
            };

            let card = Card::new("", Local::now());
            let insert_result = self.board()
                .as_ref()
                .borrow_mut()
                .insert_card(column_index, card_index, Cow::Borrowed(&card));
            if let Err(e) = insert_result {
                self.log(&format!("Failed to insert card: {}", e));
            }

            let select_result = self.board().as_ref().borrow_mut().select_card(column_index, card_index);
            if let Err(e) = select_result {
                self.log(&format!("Failed to select card: {}", e));
            }

            Some(card)
        } else {
            None
        }
    }

    fn remove_card(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let remove_result = this.board().as_ref().borrow_mut().remove_card(column_index, card_index);
            let (column_index, card_index) = match remove_result {
                Ok(indices) => indices,
                Err(e) => {
                    this.log(&format!("Failed to remove card: {}", e));
                    (column_index, card_index)
                }
            };
            let select_result = this.board().as_ref().borrow_mut().select_card(column_index, card_index);
            if let Err(e) = select_result {
                this.log(&format!("Failed to select card: {}", e));
            }
            (column_index, card_index)
        });
    }

    fn increase_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board()
                .as_ref()
                .borrow_mut()
                .increase_priority(column_index, card_index)
        });
    }

    fn decrease_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board()
                .as_ref()
                .borrow_mut()
                .decrease_priority(column_index, card_index)
        });
    }

    fn mark_card_done(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board()
                .as_ref()
                .borrow_mut()
                .mark_card_done(column_index, card_index)
        });
    }

    fn mark_card_undone(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            this.board()
                .as_ref()
                .borrow_mut()
                .mark_card_undone(column_index, card_index)
        });
    }

    fn write(&mut self) {
        let result = {
            let board = self.board().as_ref().borrow();
            self.file_service().save_board(&board, self.file_name())
        };
        
        match result {
            Ok(_) => self.log(&format!("Board successfully saved to '{}'", self.file_name())),
            Err(e) => self.log(&format!("Failed to save board to '{}': {}", self.file_name(), e)),
        }
    }
}
