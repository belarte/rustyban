use std::{cell::RefCell, rc::Rc};

use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::{
    engine::app::App,
    engine::save_to_file::Save,
    ui::card_editor::CardEditor,
    ui::event_handlers::{edit, normal, save},
    ui::help::Help,
};

#[derive(Debug, PartialEq)]
pub enum State<'a> {
    Normal,
    Save { save: Rc<RefCell<Save<'a>>> },
    Edit { editor: Rc<RefCell<CardEditor>> },
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
            State::Save { save } => {
                let save_widget = save.borrow();
                frame.render_widget(&*save_widget, frame.area());
            }
            State::Edit { editor } => {
                let editor_widget = editor.borrow();
                frame.render_widget(&*editor_widget, frame.area());
            }
            State::Help => frame.render_widget(Help, frame.area()),
            State::Quit => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use crossterm::event::KeyCode;

    use crate::engine::app_state::State;

    use super::*;

    #[test]
    fn handle_exit() -> Result<()> {
        let mut app = App::new("".into());
        let mut state = AppState::new();

        assert!(state.should_continue());
        state.handle_events(&mut app, KeyCode::Char('q').into());
        assert!(!state.should_continue());
        assert!(matches!(state.state, State::Quit));

        Ok(())
    }

    #[test]
    fn toggle_help_popup() -> Result<()> {
        let mut app = App::new("".into());
        let mut state = AppState::new();
        assert!(matches!(state.state, State::Normal));

        state.handle_events(&mut app, KeyCode::Char('?').into());
        assert!(matches!(state.state, State::Help));

        state.handle_events(&mut app, KeyCode::Char('q').into());
        assert!(matches!(state.state, State::Normal));

        Ok(())
    }
}
