use crossterm::event::{KeyCode, KeyEvent};
use tui_textarea::{Input, Key};

use crate::app::{App, app_state::State};

use super::save_to_file::Save;

pub fn normal_mode<'a>(app: &mut App, key_event: KeyEvent) -> State<'a> {
    match key_event.code {
        KeyCode::Char('h') => {
            app.select_prev_column();
            State::Normal
        }
        KeyCode::Char('j') => {
            app.select_next_card();
            State::Normal
        }
        KeyCode::Char('k') => {
            app.select_prev_card();
            State::Normal
        }
        KeyCode::Char('l') => {
            app.select_next_column();
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
