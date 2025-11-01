use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    domain::{event_handlers::AppOperations, InsertPosition},
    engine::app::App,
    engine::app_state::State,
    engine::save_to_file::Save,
    ui::card_editor::CardEditor,
};

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
        KeyCode::Char('a') => card_edition(app, Edition::InsertAtNextPosition),
        KeyCode::Char('I') => card_edition(app, Edition::InsertTop),
        KeyCode::Char('A') => card_edition(app, Edition::InsertBottom),
        KeyCode::Char('e') | KeyCode::Enter => card_edition(app, Edition::EditCurrent),
        KeyCode::Char('x') | KeyCode::Delete => card_edition(app, Edition::RemoveCurrent),

        // Undo/Redo
        KeyCode::Char('u') => {
            <App as AppOperations>::undo(app);
            State::Normal
        }
        KeyCode::Char('r') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            <App as AppOperations>::redo(app);
            State::Normal
        }

        // Other operations
        KeyCode::Esc => {
            app.disable_selection();
            State::Normal
        }
        KeyCode::Char('w') => {
            app.write();
            State::Normal
        }
        KeyCode::Char('W') => State::Save {
            save: Rc::new(RefCell::new(Save::new())),
        },
        KeyCode::Char('q') => State::Quit,
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
    RemoveCurrent,
    InsertAtCurrentPosition,
    InsertAtNextPosition,
    InsertTop,
    InsertBottom,
}

fn card_edition<'a>(app: &mut App, operation: Edition) -> State<'a> {
    let card = match operation {
        Edition::EditCurrent => app.get_selected_card(),
        Edition::RemoveCurrent => {
            app.remove_card();
            None
        }
        Edition::InsertAtCurrentPosition => app.insert_card(InsertPosition::Current),
        Edition::InsertAtNextPosition => app.insert_card(InsertPosition::Next),
        Edition::InsertTop => app.insert_card(InsertPosition::Top),
        Edition::InsertBottom => app.insert_card(InsertPosition::Bottom),
    };

    match card {
        Some(card) => State::Edit {
            editor: Rc::new(RefCell::new(CardEditor::new(card))),
        },
        None => State::Normal,
    }
}

#[cfg(test)]
mod tests {
    use std::{char, io::Result};

    use crate::domain::event_handlers::AppOperations;
    use crate::domain::InsertPosition;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use crate::{engine::app::App, engine::app_state::State, ui::event_handlers::normal::handler};

    fn build_event(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())
    }

    fn build_event_with_modifier(c: char, modifier: KeyModifiers) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), modifier)
    }

    #[test]
    fn exit() -> Result<()> {
        let mut app = App::new("res/test_board.json");
        let state = handler(&mut app, build_event('q'));
        assert!(matches!(state, State::Quit));

        Ok(())
    }

    #[test]
    fn card_navigation() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        let keys = vec!['h', 'j', 'k', 'l', 'H', 'J', 'K', 'L'];

        for key in keys {
            let state = handler(&mut app, build_event(key));
            assert!(matches!(state, State::Normal));
        }

        Ok(())
    }

    #[test]
    fn switch_to_edit_mode() -> Result<()> {
        let mut app = App::new("res/test_board.json");
        app.select_next_card();

        let keys = vec!['e', 'i'];

        for key in keys {
            let state = handler(&mut app, build_event(key));
            assert!(matches!(state, State::Edit { .. }));
        }

        Ok(())
    }

    #[test]
    fn edit_card_does_nothing_when_selection_is_disabled() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        let keys = vec!['e', 'i'];

        for key in keys {
            let state = handler(&mut app, build_event(key));
            assert!(matches!(state, State::Normal));
        }

        Ok(())
    }

    #[test]
    fn help() -> Result<()> {
        let mut app = App::new("res/test_board.json");
        let state = handler(&mut app, build_event('?'));
        assert!(matches!(state, State::Help));

        Ok(())
    }

    #[test]
    fn undo_keybinding() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        app.select_next_card();
        let initial_size = {
            let board = app.board().as_ref().borrow();
            board.column(0).unwrap().size()
        };

        app.insert_card(InsertPosition::Current);
        assert!(app.can_undo());

        let state = handler(&mut app, build_event('u'));
        assert!(matches!(state, State::Normal));
        assert!(!app.can_undo());

        {
            let board = app.board().as_ref().borrow();
            assert_eq!(board.column(0).unwrap().size(), initial_size);
        }

        Ok(())
    }

    #[test]
    fn redo_keybinding() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        app.select_next_card();
        let initial_size = {
            let board = app.board().as_ref().borrow();
            board.column(0).unwrap().size()
        };

        app.insert_card(InsertPosition::Current);
        app.undo();
        assert!(app.can_redo());

        let state = handler(&mut app, build_event_with_modifier('r', KeyModifiers::CONTROL));
        assert!(matches!(state, State::Normal));
        assert!(!app.can_redo());

        {
            let board = app.board().as_ref().borrow();
            assert_eq!(board.column(0).unwrap().size(), initial_size + 1);
        }

        Ok(())
    }

    #[test]
    fn undo_keybinding_when_nothing_to_undo() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        assert!(!app.can_undo());
        let state = handler(&mut app, build_event('u'));
        assert!(matches!(state, State::Normal));
        assert!(!app.can_undo());

        Ok(())
    }

    #[test]
    fn redo_keybinding_when_nothing_to_redo() -> Result<()> {
        let mut app = App::new("res/test_board.json");

        assert!(!app.can_redo());
        let state = handler(&mut app, build_event_with_modifier('r', KeyModifiers::CONTROL));
        assert!(matches!(state, State::Normal));
        assert!(!app.can_redo());

        Ok(())
    }
}
