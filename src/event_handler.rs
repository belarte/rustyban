use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::App;

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
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Char('?') => app.toggle_help(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_events() -> io::Result<()> {
        let mut app = App::new();
        handle_key_event(&mut app, KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}

