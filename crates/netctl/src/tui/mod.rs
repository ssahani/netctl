pub mod colors;
pub mod dashboard;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use miette::{IntoDiagnostic, Result};
use ratatui::prelude::*;
use std::io::{self, stdout};
use std::time::Duration;

pub use dashboard::Dashboard;

pub struct TuiApp {
    should_quit: bool,
}

impl TuiApp {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100)).into_diagnostic()? {
            if let Event::Key(key) = event::read().into_diagnostic()? {
                self.handle_key_event(key);
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode().into_diagnostic()?;
    stdout().execute(EnterAlternateScreen).into_diagnostic()?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout())).into_diagnostic()?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<()> {
    disable_raw_mode().into_diagnostic()?;
    stdout().execute(LeaveAlternateScreen).into_diagnostic()?;
    Ok(())
}
