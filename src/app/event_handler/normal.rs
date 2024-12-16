use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{app::App, app_state::State, card_editor::CardEditor, save_to_file::Save};

pub fn handler<'a>(app: &mut App, key_event: KeyEvent) -> State<'a> {
    match key_event.code {
        KeyCode::Char('h') | KeyCode::Left => {
            app.select_prev_column();
            State::Normal
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.select_next_card();
            State::Normal
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.select_prev_card();
            State::Normal
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app.select_next_column();
            State::Normal
        }
        KeyCode::Char('H') => {
            app.mark_card_undone();
            State::Normal
        }
        KeyCode::Char('J') => {
            app.decrease_priority();
            State::Normal
        }
        KeyCode::Char('K') => {
            app.increase_priority();
            State::Normal
        }
        KeyCode::Char('L') => {
            app.mark_card_done();
            State::Normal
        }
        KeyCode::Char('i') => match app.insert_card() {
            Some(card) => State::Edit {
                editor: CardEditor::new(card),
            },
            None => State::Normal,
        },
        KeyCode::Char('e') | KeyCode::Enter => match app.get_selected_card() {
            Some(card) => State::Edit {
                editor: CardEditor::new(card),
            },
            None => State::Normal,
        },
        KeyCode::Esc => {
            app.disable_selection();
            State::Normal
        }
        KeyCode::Char('w') => {
            app.write();
            State::Normal
        }
        KeyCode::Char('W') => State::Save { save: Save::new() },
        KeyCode::Char('q') => {
            app.exit();
            State::Normal
        }
        KeyCode::Char('?') => State::Help,
        _ => State::Normal,
    }
}
