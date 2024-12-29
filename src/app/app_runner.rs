use std::io::Result;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::app::App;
use crate::app::AppState;

#[derive(Debug)]
pub struct AppRunner<'a> {
    app: App,
    state: AppState<'a>,
}

impl<'a> AppRunner<'a> {
    pub fn new(file_name: String) -> AppRunner<'a> {
        Self {
            app: App::new(file_name),
            state: AppState::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.state.should_continue() {
            terminal.draw(|frame| self.draw(frame))?;

            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.state.handle_events(&mut self.app, key_event);
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        self.state.render(&self.app, frame)
    }
}
