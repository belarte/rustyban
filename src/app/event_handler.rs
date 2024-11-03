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

pub fn edit_mode<'a>(mut editor: CardEditor, app: &mut App, key_event: KeyEvent) -> State<'a> {
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

pub fn save_mode<'a>(mut save: Save<'a>, app: &mut App, key_event: KeyEvent) -> State<'a> {
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
