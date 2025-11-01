use chrono::Local;

use crate::core::Card;
use crate::domain::commands::{
    ChangePriorityCommand, InsertCardCommand, MarkCardCommand, RemoveCardCommand, UpdateCardCommand,
};
use crate::domain::{event_handlers::AppOperations, InsertPosition};

use super::App;

impl AppOperations for App {
    fn update_card(&mut self, card: Card) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(UpdateCardCommand::new(column_index, card_index, card.clone()));
            let _ = this.execute_command_with_error_handling(command, "update card");
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
            let result = self
                .board()
                .as_ref()
                .borrow_mut()
                .deselect_card(column_index, card_index);
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
        self.with_selected_card(|this, column_index, card_index| {
            let deselect_result = this
                .board()
                .as_ref()
                .borrow_mut()
                .deselect_card(column_index, card_index);
            if let Err(e) = deselect_result {
                this.log(&format!("Failed to deselect card: {}", e));
            }

            let column_size = this
                .board()
                .as_ref()
                .borrow()
                .column(column_index)
                .map(|c| c.size())
                .unwrap_or(0);

            let insert_index = match position {
                InsertPosition::Current => card_index.min(column_size),
                InsertPosition::Next => (card_index + 1).min(column_size),
                InsertPosition::Top => 0,
                InsertPosition::Bottom => column_size,
            };

            let card = Card::new("TODO", Local::now());
            let command = Box::new(InsertCardCommand::new(column_index, insert_index, card));
            let result = this.execute_command_with_error_handling(command, "insert card");

            if App::is_command_success(&result) {
                this.update_selection(column_index, insert_index);
                (column_index, insert_index)
            } else {
                (column_index, card_index)
            }
        });

        self.get_selected_card()
    }

    fn remove_card(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(RemoveCardCommand::new(column_index, card_index));
            let result = this.execute_command_with_error_handling(command, "remove card");

            if App::is_command_success(&result) {
                let board = this.board().as_ref().borrow();
                let new_column_size = board.column(column_index).map(|c| c.size()).unwrap_or(0);
                let new_card_index = if new_column_size > 0 {
                    card_index.min(new_column_size - 1)
                } else {
                    0
                };
                drop(board);

                if new_column_size > 0 {
                    this.update_selection(column_index, new_card_index);
                }
                (column_index, new_card_index)
            } else {
                (column_index, card_index)
            }
        });
    }

    fn increase_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(ChangePriorityCommand::increase(column_index, card_index));
            let result = this.execute_command_with_error_handling(command, "increase priority");

            if App::is_command_success(&result) {
                if let Some(new_card_index) = this.find_selected_card_index(column_index) {
                    if new_card_index != card_index {
                        this.update_selection(column_index, new_card_index);
                    }
                    (column_index, new_card_index)
                } else {
                    (column_index, card_index)
                }
            } else {
                (column_index, card_index)
            }
        });
    }

    fn decrease_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(ChangePriorityCommand::decrease(column_index, card_index));
            let result = this.execute_command_with_error_handling(command, "decrease priority");

            if App::is_command_success(&result) {
                if let Some(new_card_index) = this.find_selected_card_index(column_index) {
                    if new_card_index != card_index {
                        this.update_selection(column_index, new_card_index);
                    }
                    (column_index, new_card_index)
                } else {
                    (column_index, card_index)
                }
            } else {
                (column_index, card_index)
            }
        });
    }

    fn mark_card_done(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(MarkCardCommand::mark_done(column_index, card_index));
            let result = this.execute_command_with_error_handling(command, "mark card done");

            if App::is_command_success(&result) {
                let (new_column_index, new_card_index) =
                    if column_index + 1 < this.board().as_ref().borrow().columns_count() {
                        this.find_selected_card_in_column(column_index + 1)
                            .unwrap_or((column_index, card_index))
                    } else {
                        (column_index, card_index)
                    };

                if new_column_index != column_index || new_card_index != card_index {
                    this.update_selection(new_column_index, new_card_index);
                }
                (new_column_index, new_card_index)
            } else {
                (column_index, card_index)
            }
        });
    }

    fn mark_card_undone(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(MarkCardCommand::mark_undone(column_index, card_index));
            let result = this.execute_command_with_error_handling(command, "mark card undone");

            if App::is_command_success(&result) {
                let (new_column_index, new_card_index) = if column_index > 0 {
                    this.find_selected_card_in_column(column_index - 1)
                        .unwrap_or((column_index, card_index))
                } else {
                    (column_index, card_index)
                };

                if new_column_index != column_index || new_card_index != card_index {
                    this.update_selection(new_column_index, new_card_index);
                }
                (new_column_index, new_card_index)
            } else {
                (column_index, card_index)
            }
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
