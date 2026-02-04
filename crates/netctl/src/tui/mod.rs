pub mod app;
pub mod colors;
pub mod dashboard;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use miette::{IntoDiagnostic, Result};
use ratatui::prelude::*;
use std::io::{self, stdout};
use std::time::{Duration, Instant};

pub use app::App;
pub use dashboard::Dashboard;

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

pub async fn run(mut app: App) -> Result<()> {
    let mut terminal = init_terminal()?;
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        // Get interface count for navigation
        let interface_count = match app.manager.list_links().await {
            Ok(links) => links.len(),
            Err(_) => 0,
        };

        // Draw UI
        terminal.draw(|frame| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    if let Err(e) = app.render(frame).await {
                        eprintln!("Render error: {}", e);
                    }
                })
            });
        }).into_diagnostic()?;

        // Calculate timeout
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Handle events
        if event::poll(timeout).into_diagnostic()? {
            match event::read().into_diagnostic()? {
                Event::Key(key) => {
                    handle_key_event(&mut app, key, interface_count);
                    if app.should_quit {
                        break;
                    }
                }
                _ => {}
            }
        }

        // Update tick
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    restore_terminal()?;
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent, interface_count: usize) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next(interface_count);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        _ => {}
    }
}
