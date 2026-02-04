pub mod address;
pub mod apply;
pub mod backup;
pub mod completion;
pub mod diff;
pub mod doctor;
pub mod export;
pub mod history;
pub mod link;
pub mod profile;
pub mod show;
pub mod stats;
pub mod test;
pub mod tui;
pub mod validate;
pub mod watch;
pub mod wizard;

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

    /// Real-time TUI dashboard (interactive interface)
    Tui(tui::TuiArgs),

    /// Watch interfaces (continuous monitoring)
    Watch(watch::WatchArgs),

    /// Manage network profiles (save/load configurations)
    Profile(profile::ProfileCommand),

    /// Apply declarative configuration from file
    Apply(apply::ApplyArgs),

    /// Compare network configurations
    Diff(diff::DiffArgs),

    /// Interactive configuration wizard
    Wizard(wizard::WizardArgs),

    /// Run system diagnostics
    Doctor(doctor::DoctorArgs),

    /// Show network statistics
    Stats(stats::StatsArgs),

    /// Validate configuration file
    Validate(validate::ValidateArgs),

    /// Generate shell completion scripts
    Completion(completion::CompletionArgs),

    /// Manage configuration history and rollback
    History(history::HistoryCommand),

    /// Backup and restore network configuration
    Backup(backup::BackupCommand),

    /// Test network connectivity and functionality
    Test(test::TestCommand),

    /// Export network configuration to file
    Export(export::ExportArgs),
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Show(args) => args.execute().await,
            Commands::Link(cmd) => cmd.execute().await,
            Commands::Address(cmd) => cmd.execute().await,
            Commands::Tui(args) => args.execute().await,
            Commands::Watch(args) => args.execute().await,
            Commands::Profile(cmd) => cmd.execute().await,
            Commands::Apply(args) => args.execute().await,
            Commands::Diff(args) => args.execute().await,
            Commands::Wizard(args) => args.execute().await,
            Commands::Doctor(args) => args.execute().await,
            Commands::Stats(args) => args.execute().await,
            Commands::Validate(args) => args.execute().await,
            Commands::Completion(args) => args.execute().await,
            Commands::History(cmd) => cmd.execute().await,
            Commands::Backup(cmd) => cmd.execute().await,
            Commands::Test(cmd) => cmd.execute().await,
            Commands::Export(args) => args.execute().await,
        }
    }
}
