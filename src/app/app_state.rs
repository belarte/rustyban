use crossterm::event::KeyEvent;
use ratatui::Frame;

use super::{app::App, event_handler::{normal_mode, save_mode}, help::Help, save_to_file::Save};


#[derive(Debug, PartialEq, Eq)]
pub enum State<'a> {
    Normal,
    Save { save: Save<'a> },
    Help,
}

#[derive(Debug)]
pub struct AppState<'a> {
    pub state: State<'a>,
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        Self { state: State::Normal }
    }

    pub fn handle_events(&mut self, app: &mut App, event: KeyEvent) {
        match &self.state {
            State::Normal => self.state = normal_mode(app, event),
            State::Save{ save } => self.state = save_mode(save, app, event),
            State::Help   => self.state = State::Normal,
        }
    }

    pub fn render(&self, app: &App, frame: &mut Frame) {
        frame.render_widget(app, frame.area());

        match &self.state {
            State::Help   => frame.render_widget(Help, frame.area()),
            State::Save{ save } => frame.render_widget(save, frame.area()),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use crossterm::event::KeyCode;

    use crate::app::app_state::State;

    use super::*;

    #[test]
    fn handle_exit() -> Result<()> {
        let mut app = App::new("".into());
        let mut state = AppState::new();
        state.handle_events(&mut app, KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }

    #[test]
    fn toggle_help_popup() -> Result<()> {
        let mut app = App::new("".into());
        let mut state = AppState::new();
        assert_eq!(State::Normal, state.state);

        state.handle_events(&mut app, KeyCode::Char('?').into());
        assert_eq!(State::Help, state.state);

        state.handle_events(&mut app, KeyCode::Char('q').into());
        assert_eq!(State::Normal, state.state);

        Ok(())
    }
}

