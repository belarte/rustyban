use std::io::Result;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::engine::app::App;
use crate::engine::app_state::AppState;

/// The main terminal UI runner for the rustyban application.
///
/// `AppRunner` coordinates the terminal user interface, handling the event loop,
/// rendering, and state management. It combines an [`App`] instance (for business logic)
/// with an internal state manager (for UI state management) to provide the complete terminal experience.
///
/// # Architecture
///
/// The AppRunner follows a typical terminal UI pattern:
/// 1. **Event Loop**: Continuously reads keyboard input from the terminal
/// 2. **Event Handling**: Processes events through the state machine
/// 3. **Rendering**: Draws the current UI state to the terminal
/// 4. **State Management**: Manages different UI modes (normal, edit, help, etc.)
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,no_run
/// use rustyban::AppRunner;
/// use ratatui::init;
///
/// # fn main() -> std::io::Result<()> {
/// // Initialize the terminal
/// let mut terminal = init();
///
/// // Create and run the application
/// let mut app_runner = AppRunner::new("my_board.json");
/// app_runner.run(&mut terminal)?;
///
/// // Restore the terminal
/// ratatui::restore();
/// # Ok(())
/// # }
/// ```
///
/// ## Complete Application Setup
///
/// ```rust,no_run
/// use rustyban::AppRunner;
/// use ratatui::{init, restore};
/// use std::io::Result;
///
/// fn main() -> Result<()> {
///     // Initialize terminal - this sets up raw mode and alternate screen
///     let mut terminal = init();
///
///     // Run the application
///     let result = run_app(&mut terminal);
///
///     // Always restore terminal, even if the app panics
///     restore();
///
///     result
/// }
///
/// fn run_app(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
///     let mut app_runner = AppRunner::new("kanban.json");
///     app_runner.run(terminal)
/// }
/// ```
///
/// [`App`]: crate::engine::App
#[derive(Debug)]
pub struct AppRunner<'a> {
    app: App,
    state: AppState<'a>,
}

impl<'a> AppRunner<'a> {
    /// Creates a new AppRunner instance.
    ///
    /// This initializes both the application logic (App) and the UI state management
    /// (AppState) with their default configurations. The app will be associated with
    /// the specified file name for board persistence.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustyban::AppRunner;
    ///
    /// let app_runner = AppRunner::new("my_kanban_board.json");
    /// // The app runner is now ready to be used with a terminal
    /// ```
    pub fn new(file_name: &str) -> AppRunner<'a> {
        Self {
            app: App::new(file_name),
            state: AppState::new(),
        }
    }

    /// Runs the main application loop.
    ///
    /// This method starts the terminal UI and enters the main event loop, which continues
    /// until the user chooses to quit the application. The loop handles:
    /// - Drawing the current UI state to the terminal
    /// - Reading keyboard input events
    /// - Processing events through the state machine
    /// - Updating the application state
    ///
    /// # Arguments
    ///
    /// * `terminal` - A mutable reference to the initialized terminal instance
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the user exits the application normally, or an `Err`
    /// if there's an I/O error with the terminal.
    ///
    /// # Errors
    ///
    /// This method can return an error if:
    /// - Terminal drawing operations fail
    /// - Keyboard input reading fails
    /// - Other terminal I/O operations encounter problems
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rustyban::AppRunner;
    /// use ratatui::init;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let mut terminal = init();
    /// let mut app_runner = AppRunner::new("board.json");
    ///
    /// // This will run until the user presses 'q' to quit
    /// app_runner.run(&mut terminal)?;
    ///
    /// ratatui::restore();
    /// # Ok(())
    /// # }
    /// ```
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
