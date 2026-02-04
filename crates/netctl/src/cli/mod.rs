pub mod address;
pub mod dashboard;
pub mod link;
pub mod profile;
pub mod show;
pub mod watch;

use clap::{Parser, Subcommand};
use miette::Result;

#[derive(Parser)]
#[command(name = "netctl")]
#[command(author, version, about = "Modern network configuration tool", long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show network interfaces
    Show(show::ShowArgs),

    /// Manage network links
    Link(link::LinkCommand),

    /// Manage IP addresses
    #[command(name = "addr")]
    Address(address::AddressCommand),

    /// Real-time TUI dashboard
    Dashboard(dashboard::DashboardArgs),

    /// Watch interfaces (continuous monitoring)
    Watch(watch::WatchArgs),

    /// Manage network profiles (save/load configurations)
    Profile(profile::ProfileCommand),
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Show(args) => args.execute().await,
            Commands::Link(cmd) => cmd.execute().await,
            Commands::Address(cmd) => cmd.execute().await,
            Commands::Dashboard(args) => args.execute().await,
            Commands::Watch(args) => args.execute().await,
            Commands::Profile(cmd) => cmd.execute().await,
        }
    }
}
