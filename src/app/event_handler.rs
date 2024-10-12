use crossterm::event::{KeyCode, KeyEvent};
use tui_textarea::{Input, Key};

use crate::app::{app_state::State, card_editor::CardEditor, App};

use super::save_to_file::Save;

pub fn normal_mode<'a>(app: &mut App, key_event: KeyEvent) -> State<'a> {
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
        KeyCode::Char('e') | KeyCode::Enter => {
            match app.get_selection() {
                Some((column, card)) => {
                    app.edit_card(column, card);
                    return State::Edit{ editor: CardEditor::new() }
                }
                None => return State::Normal
            }
        }
        KeyCode::Esc => {
            app.disable_selection();
            State::Normal
        }
        KeyCode::Char('w') => {
            app.write();
            State::Normal
        }
        KeyCode::Char('W') => State::Save{ save: Save::new() },
        KeyCode::Char('q') => {
            app.exit();
            State::Normal
        }
        KeyCode::Char('?') => State::Help,
        _ => State::Normal
    }
}

pub fn save_mode<'a>(save: &Save<'a>, app: &mut App, key_event: KeyEvent) -> State<'a> {
    match key_event.into() {
        Input { key: Key::Esc, .. } => State::Normal,
        Input { key: Key::Enter, .. } => {
            app.write_to_file(save.get());
            State::Normal
        }
        input => {
            let mut save = save.clone();
            save.push(input);
            State::Save{ save }
        }
    }
}
