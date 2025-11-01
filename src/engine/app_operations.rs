use chrono::Local;

use crate::core::Card;
use crate::domain::command::CommandResult;
use crate::domain::commands::{
    ChangePriorityCommand, InsertCardCommand, MarkCardCommand, RemoveCardCommand, UpdateCardCommand,
};
use crate::domain::{event_handlers::AppOperations, InsertPosition};

use super::App;

impl AppOperations for App {
    fn update_card(&mut self, card: Card) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(UpdateCardCommand::new(column_index, card_index, card.clone()));
            let result = this.execute_command(command);

            match result {
                Ok(CommandResult::Success | CommandResult::SuccessWithMessage(_)) => {}
                Ok(CommandResult::Failure(msg)) => {
                    this.log(&format!("Failed to update card: {}", msg));
                }
                Err(e) => {
                    this.log(&format!("Failed to update card: {}", e));
                }
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
            let result = this.execute_command(command);

            match result {
                Ok(CommandResult::Success | CommandResult::SuccessWithMessage(_)) => {
                    let select_result = this
                        .board()
                        .as_ref()
                        .borrow_mut()
                        .select_card(column_index, insert_index);
                    if let Err(e) = select_result {
                        this.log(&format!("Failed to select card: {}", e));
                    }
                    (column_index, insert_index)
                }
                Ok(CommandResult::Failure(msg)) => {
                    this.log(&format!("Failed to insert card: {}", msg));
                    (column_index, card_index)
                }
                Err(e) => {
                    this.log(&format!("Failed to insert card: {}", e));
                    (column_index, card_index)
                }
            }
        });

        self.get_selected_card()
    }

    fn remove_card(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(RemoveCardCommand::new(column_index, card_index));
            let result = this.execute_command(command);

            match result {
                Ok(CommandResult::Success | CommandResult::SuccessWithMessage(_)) => {
                    let board = this.board().as_ref().borrow();
                    let new_column_size = board.column(column_index).map(|c| c.size()).unwrap_or(0);
                    let new_card_index = if new_column_size > 0 {
                        card_index.min(new_column_size - 1)
                    } else {
                        0
                    };

                    drop(board);
                    let select_result = if new_column_size > 0 {
                        this.board()
                            .as_ref()
                            .borrow_mut()
                            .select_card(column_index, new_card_index)
                    } else {
                        Ok(())
                    };
                    if let Err(e) = select_result {
                        this.log(&format!("Failed to select card: {}", e));
                    }
                    (column_index, new_card_index)
                }
                Ok(CommandResult::Failure(msg)) => {
                    this.log(&format!("Failed to remove card: {}", msg));
                    (column_index, card_index)
                }
                Err(e) => {
                    this.log(&format!("Failed to remove card: {}", e));
                    (column_index, card_index)
                }
            }
        });
    }

    fn increase_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(ChangePriorityCommand::increase(column_index, card_index));
            let result = this.execute_command(command);

            match result {
                Ok(CommandResult::Success | CommandResult::SuccessWithMessage(_)) => {
                    let board = this.board().as_ref().borrow();
                    let new_card_index = if let Some(col) = board.column(column_index) {
                        (0..col.size())
                            .find(|&i| col.card(i).map(|c| c.is_selected()).unwrap_or(false))
                            .unwrap_or(card_index)
                    } else {
                        card_index
                    };
                    drop(board);
                    if new_card_index != card_index {
                        let select_result = this
                            .board()
                            .as_ref()
                            .borrow_mut()
                            .select_card(column_index, new_card_index);
                        if let Err(e) = select_result {
                            this.log(&format!("Failed to select card: {}", e));
                        }
                    }
                    (column_index, new_card_index)
                }
                Ok(CommandResult::Failure(msg)) => {
                    this.log(&format!("Failed to increase priority: {}", msg));
                    (column_index, card_index)
                }
                Err(e) => {
                    this.log(&format!("Failed to increase priority: {}", e));
                    (column_index, card_index)
                }
            }
        });
    }

    fn decrease_priority(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(ChangePriorityCommand::decrease(column_index, card_index));
            let result = this.execute_command(command);

            match result {
                Ok(CommandResult::Success | CommandResult::SuccessWithMessage(_)) => {
                    let board = this.board().as_ref().borrow();
                    let new_card_index = if let Some(col) = board.column(column_index) {
                        (0..col.size())
                            .find(|&i| col.card(i).map(|c| c.is_selected()).unwrap_or(false))
                            .unwrap_or(card_index)
                    } else {
                        card_index
                    };
                    drop(board);
                    if new_card_index != card_index {
                        let select_result = this
                            .board()
                            .as_ref()
                            .borrow_mut()
                            .select_card(column_index, new_card_index);
                        if let Err(e) = select_result {
                            this.log(&format!("Failed to select card: {}", e));
                        }
                    }
                    (column_index, new_card_index)
                }
                Ok(CommandResult::Failure(msg)) => {
                    this.log(&format!("Failed to decrease priority: {}", msg));
                    (column_index, card_index)
                }
                Err(e) => {
                    this.log(&format!("Failed to decrease priority: {}", e));
                    (column_index, card_index)
                }
            }
        });
    }

    fn mark_card_done(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(MarkCardCommand::mark_done(column_index, card_index));
            let result = this.execute_command(command);

            match result {
                Ok(CommandResult::Success | CommandResult::SuccessWithMessage(_)) => {
                    let board = this.board().as_ref().borrow();
                    let (new_column_index, new_card_index) = if column_index + 1 < board.columns_count() {
                        if let Some(col) = board.column(column_index + 1) {
                            if let Some(idx) =
                                (0..col.size()).find(|&i| col.card(i).map(|c| c.is_selected()).unwrap_or(false))
                            {
                                (column_index + 1, idx)
                            } else {
                                (column_index, card_index)
                            }
                        } else {
                            (column_index, card_index)
                        }
                    } else {
                        (column_index, card_index)
                    };
                    drop(board);
                    if new_column_index != column_index || new_card_index != card_index {
                        let select_result = this
                            .board()
                            .as_ref()
                            .borrow_mut()
                            .select_card(new_column_index, new_card_index);
                        if let Err(e) = select_result {
                            this.log(&format!("Failed to select card: {}", e));
                        }
                    }
                    (new_column_index, new_card_index)
                }
                Ok(CommandResult::Failure(msg)) => {
                    this.log(&format!("Failed to mark card done: {}", msg));
                    (column_index, card_index)
                }
                Err(e) => {
                    this.log(&format!("Failed to mark card done: {}", e));
                    (column_index, card_index)
                }
            }
        });
    }

    fn mark_card_undone(&mut self) {
        self.with_selected_card(|this, column_index, card_index| {
            let command = Box::new(MarkCardCommand::mark_undone(column_index, card_index));
            let result = this.execute_command(command);

            match result {
                Ok(CommandResult::Success | CommandResult::SuccessWithMessage(_)) => {
                    let board = this.board().as_ref().borrow();
                    let (new_column_index, new_card_index) = if column_index > 0 {
                        if let Some(col) = board.column(column_index - 1) {
                            if let Some(idx) =
                                (0..col.size()).find(|&i| col.card(i).map(|c| c.is_selected()).unwrap_or(false))
                            {
                                (column_index - 1, idx)
                            } else {
                                (column_index, card_index)
                            }
                        } else {
                            (column_index, card_index)
                        }
                    } else {
                        (column_index, card_index)
                    };
                    drop(board);
                    if new_column_index != column_index || new_card_index != card_index {
                        let select_result = this
                            .board()
                            .as_ref()
                            .borrow_mut()
                            .select_card(new_column_index, new_card_index);
                        if let Err(e) = select_result {
                            this.log(&format!("Failed to select card: {}", e));
                        }
                    }
                    (new_column_index, new_card_index)
                }
                Ok(CommandResult::Failure(msg)) => {
                    this.log(&format!("Failed to mark card undone: {}", msg));
                    (column_index, card_index)
                }
                Err(e) => {
                    this.log(&format!("Failed to mark card undone: {}", e));
                    (column_index, card_index)
                }
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
