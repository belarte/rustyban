use crossterm::event::KeyEvent;
use tui_textarea::{Input, Key};

use crate::app::{app::App, app_state::State, card_editor::CardEditor};

pub fn handler<'a>(mut editor: CardEditor, app: &mut App, key_event: KeyEvent) -> State<'a> {
    match key_event.into() {
        Input { key: Key::Esc, .. } => State::Normal,
        Input {
            key: Key::Char('s'),
            ctrl: true,
            ..
        } => {
            let card = editor.get_card();
            app.update_card(card);
            State::Normal
        }
        Input { key: Key::Tab, .. } => {
            editor.next_field();
            State::Edit { editor }
        }
        input => {
            editor.input(input);
            State::Edit { editor }
        }
    }
}
