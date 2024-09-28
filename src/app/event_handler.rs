use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use tui_textarea::{Input, Key};

use crate::app::{App, app::State};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            app.state = handle_key_event(app, key_event);
        }
        _ => {}
    };

    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) -> State {
    match app.state {
        State::Normal => normal_mode(app, key_event),
        State::Help   => State::Normal,
        State::Save   => save_mode(app, key_event),
    }
}

fn normal_mode(app: &mut App, key_event: KeyEvent) -> State {
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

fn save_mode(app: &mut App, key_event: KeyEvent) -> State {
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

#[cfg(test)]
mod tests {
    use crate::app::app::State;

    use super::*;

    #[test]
    fn handle_exit() -> io::Result<()> {
        let mut app = App::new("".into());
        handle_key_event(&mut app, KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }

    #[test]
    fn toggle_help_popup() -> io::Result<()> {
        let mut app = App::new("".into());
        assert_eq!(State::Normal, app.state);

        let state = handle_key_event(&mut app, KeyCode::Char('?').into());
        assert_eq!(State::Help, state);

        let state = handle_key_event(&mut app, KeyCode::Char('q').into());
        assert_eq!(State::Normal, state);

        Ok(())
    }
}

