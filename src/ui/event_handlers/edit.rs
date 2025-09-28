use std::{cell::RefCell, rc::Rc};

use crossterm::event::KeyEvent;
use tui_textarea::{Input, Key};

use crate::{engine::app::App, engine::app_state::State, ui::card_editor::CardEditor};

pub fn handler<'a>(editor: Rc<RefCell<CardEditor>>, app: &mut App, key_event: KeyEvent) -> State<'a> {
    match key_event.into() {
        Input { key: Key::Esc, .. } => State::Normal,
        Input {
            key: Key::Char('s'),
            ctrl: true,
            ..
        } => {
            let card = editor.borrow().get_card();
            app.update_card(card);
            State::Normal
        }
        Input { key: Key::Tab, .. } => {
            editor.borrow_mut().next_field();
            State::Edit { editor }
        }
        input => {
            editor.borrow_mut().input(input);
            State::Edit { editor }
        }
    }
}
