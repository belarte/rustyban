use crossterm::event::{KeyCode, KeyEvent};
use tui_textarea::{Input, Key};

use crate::app::{App, app_state::State};

pub fn normal_mode(app: &mut App, key_event: KeyEvent) -> State {
    match key_event.code {
        KeyCode::Char('w') => {
            app.write();
            State::Normal
        }
        KeyCode::Char('W') => State::Save,
        KeyCode::Char('q') => {
            app.exit();
            State::Normal
        }
        KeyCode::Char('?') => State::Help,
        _ => State::Normal
    }
}

pub fn save_mode(app: &mut App, key_event: KeyEvent) -> State {
    match key_event.into() {
        Input { key: Key::Esc, .. } => State::Normal,
        Input { key: Key::Enter, .. } => {
            app.write_to_file(app.save_to_file.get());
            app.save_to_file.clear();
            State::Normal
        }
        input => {
            app.save_to_file.push(input);
            State::Save
        }
    }
}
