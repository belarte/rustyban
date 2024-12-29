use crossterm::event::KeyEvent;
use ratatui::Frame;

use super::{
    app::App,
    card_editor::CardEditor,
    event_handler::{edit, normal, save},
    help::Help,
    save_to_file::Save,
};

#[derive(Debug, PartialEq, Eq)]
pub enum State<'a> {
    Normal,
    Save { save: Save<'a> },
    Edit { editor: CardEditor },
    Help,
    Quit,
}

#[derive(Debug)]
pub struct AppState<'a> {
    state: State<'a>,
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        Self { state: State::Normal }
    }

    pub fn should_continue(&self) -> bool {
        self.state != State::Quit
    }

    pub fn handle_events(&mut self, app: &mut App, event: KeyEvent) {
        match &self.state {
            State::Normal => self.state = normal::handler(app, event),
            State::Save { save } => self.state = save::handler(save.clone(), app, event),
            State::Edit { editor } => self.state = edit::handler(editor.clone(), app, event),
            State::Help => self.state = State::Normal,
            State::Quit => {}
        }
    }

    pub fn render(&self, app: &App, frame: &mut Frame) {
        frame.render_widget(app, frame.area());

        match &self.state {
            State::Normal => {}
            State::Save { save } => frame.render_widget(save, frame.area()),
            State::Edit { editor } => frame.render_widget(editor, frame.area()),
            State::Help => frame.render_widget(Help, frame.area()),
            State::Quit => {}
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

        assert!(state.should_continue());
        state.handle_events(&mut app, KeyCode::Char('q').into());
        assert!(!state.should_continue());
        assert_eq!(State::Quit, state.state);

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
