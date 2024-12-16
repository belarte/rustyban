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

#[cfg(test)]
mod tests {
    use std::{char, io::Result};

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use crate::app::{app::App, app_state::State};

    use super::handler;

    fn build_event(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())
    }

    #[test]
    fn card_navigation() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());

        let keys = vec!['h', 'j', 'k', 'l', 'H', 'J', 'K', 'L'];

        for key in keys {
            let state = handler(&mut app, build_event(key));
            assert_eq!(State::Normal, state);
        }

        Ok(())
    }

    #[test]
    fn switch_to_edit_mode() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());
        app.select_next_card();

        let keys = vec!['e', 'i'];

        for key in keys {
            let state = handler(&mut app, build_event(key));
            assert!(matches!(state, State::Edit{ .. }));
        }

        Ok(())
    }

    #[test]
    fn edit_card_does_nothing_when_selection_is_disabled() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());

        let keys = vec!['e', 'i'];

        for key in keys {
            let state = handler(&mut app, build_event(key));
            assert_eq!(State::Normal, state);
        }

        Ok(())
    }

    #[test]
    fn help() -> Result<()> {
        let mut app = App::new("res/test_board.json".to_string());
        let state = handler(&mut app, build_event('?'));
        assert_eq!(State::Help, state);

        Ok(())
    }
}
