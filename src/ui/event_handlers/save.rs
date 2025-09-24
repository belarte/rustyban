use crossterm::event::KeyEvent;
use tui_textarea::{Input, Key};

use crate::{engine::app_state::State, engine::save_to_file::Save, engine::app::App};

pub fn handler<'a>(mut save: Save<'a>, app: &mut App, key_event: KeyEvent) -> State<'a> {
    match key_event.into() {
        Input { key: Key::Esc, .. } => State::Normal,
        Input { key: Key::Enter, .. } => {
            app.write_to_file(save.get());
            State::Normal
        }
        input => {
            save.push(input);
            State::Save { save }
        }
    }
}
