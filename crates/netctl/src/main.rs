mod cli;
mod ui;

use clap::Parser;
use cli::Cli;
use miette::Result;
use netctl_types::logging::{init_tracing, LogFormat, LogLevel};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing(LogFormat::Pretty, LogLevel::Info);

    let cli = Cli::parse();
    cli.execute().await
}
