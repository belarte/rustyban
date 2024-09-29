use std::io::Result;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::app::App;
use crate::app::event_handler::handle_key_event;

#[derive(Debug)]
pub struct AppRunner<'a> {
    app: App<'a>,
}

impl<'a> AppRunner<'a> {
    pub fn new(file_name: String) -> AppRunner<'a> {
        Self {
            app: App::new(file_name),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.app.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.app.state = handle_key_event(&mut self.app, key_event);
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.app, frame.area());
    }
}

