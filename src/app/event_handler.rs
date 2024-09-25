use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use tui_textarea::{Input, Key};

use crate::app::{App, app::State};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event)
        }
        _ => {}
    };

    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match app.state {
        State::Normal => normal_mode(app, key_event),
        State::Help   => app.state = State::Normal,
        State::Save   => save_mode(app, key_event),
    }
}

fn normal_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('w') => app.write(),
        KeyCode::Char('W') => app.state = State::Save,
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('?') => app.state = State::Help,
        _ => {}
    }
}

fn save_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.into() {
        Input { key: Key::Esc, .. } => app.state = State::Normal,
        Input { key: Key::Enter, .. } => {
            app.write_to_file(app.save_to_file.get());
            app.save_to_file.clear();
            app.state = State::Normal;
        }
        input => app.save_to_file.push(input),
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

        handle_key_event(&mut app, KeyCode::Char('?').into());
        assert_eq!(State::Help, app.state);

        handle_key_event(&mut app, KeyCode::Char('q').into());
        assert_eq!(State::Normal, app.state);

        Ok(())
    }
}

