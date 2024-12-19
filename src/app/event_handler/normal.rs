use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{app::{App, InsertPosition}, app_state::State, card_editor::CardEditor, save_to_file::Save};

pub fn handler<'a>(app: &mut App, key_event: KeyEvent) -> State<'a> {
    match key_event.code {
        // Card navigation
        KeyCode::Char('h') | KeyCode::Left => navigate(app, Navigation::PrevColumn),
        KeyCode::Char('j') | KeyCode::Down => navigate(app, Navigation::NextCard),
        KeyCode::Char('k') | KeyCode::Up => navigate(app, Navigation::PrevCard),
        KeyCode::Char('l') | KeyCode::Right => navigate(app, Navigation::NextColumn),

        // Card marking
        KeyCode::Char('H') => card_marking(app, Operation::MarkUndone),
        KeyCode::Char('J') => card_marking(app, Operation::DecreasePriority),
        KeyCode::Char('K') => card_marking(app, Operation::IncreasePriority),
        KeyCode::Char('L') => card_marking(app, Operation::MarkDone),

        // Card edition
        KeyCode::Char('i') => card_edition(app, Edition::InsertAtCurrentPosition),
        KeyCode::Char('I') => card_edition(app, Edition::InsertTop),
        KeyCode::Char('e') | KeyCode::Enter => card_edition(app, Edition::EditCurrent),

        // Other operations
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

enum Navigation {
    PrevColumn,
    NextColumn,
    PrevCard,
    NextCard,
}

fn navigate<'a>(app: &mut App, nav: Navigation) -> State<'a> {
    match nav {
        Navigation::PrevColumn => app.select_prev_column(),
        Navigation::NextColumn => app.select_next_column(),
        Navigation::PrevCard => app.select_prev_card(),
        Navigation::NextCard => app.select_next_card(),
    }

    State::Normal
}

enum Operation {
    MarkUndone,
    DecreasePriority,
    IncreasePriority,
    MarkDone,
}

fn card_marking<'a>(app: &mut App, operation: Operation) -> State<'a> {
    match operation {
        Operation::MarkUndone => app.mark_card_undone(),
        Operation::DecreasePriority => app.decrease_priority(),
        Operation::IncreasePriority => app.increase_priority(),
        Operation::MarkDone => app.mark_card_done(),
    }

    State::Normal
}

enum Edition {
    EditCurrent,
    InsertAtCurrentPosition,
    InsertTop,
}

fn card_edition<'a>(app: &mut App, operation: Edition) -> State<'a> {
    let card = match operation {
        Edition::EditCurrent => app.get_selected_card(),
        Edition::InsertAtCurrentPosition => app.insert_card(InsertPosition::Current),
        Edition::InsertTop => app.insert_card(InsertPosition::Top),
    };

    match card {
        Some(card) => State::Edit {
            editor: CardEditor::new(card),
        },
        None => State::Normal,
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
