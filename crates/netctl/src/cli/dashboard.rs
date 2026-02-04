use clap::Args;
use miette::{IntoDiagnostic, Result};

#[derive(Args)]
pub struct DashboardArgs {}

impl DashboardArgs {
    pub async fn execute(self) -> Result<()> {
        use crate::tui::{init_terminal, restore_terminal, Dashboard, TuiApp};

        let mut terminal = init_terminal()?;
        let mut app = TuiApp::new();
        let mut dashboard = Dashboard::new().await?;

        // Main loop
        loop {
            // Refresh data if needed
            if dashboard.should_refresh() {
                dashboard.mark_refreshed();
            }

            // Render
            terminal.draw(|frame| {
                // Use tokio block_in_place to allow async in sync context
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        if let Err(e) = dashboard.render(frame).await {
                            eprintln!("Render error: {}", e);
                        }
                    })
                });
            }).into_diagnostic()?;

            // Handle events
            app.handle_events()?;

            if app.should_quit() {
                break;
            }
        }

        restore_terminal()?;
        Ok(())
    }
}
