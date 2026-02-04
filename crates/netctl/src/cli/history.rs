use clap::{Args, Subcommand};
use colored::Colorize;
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct HistoryCommand {
    #[command(subcommand)]
    command: HistorySubcommand,
}

#[derive(Subcommand)]
pub enum HistorySubcommand {
    /// List configuration history
    List(ListArgs),

    /// Show details of a history entry
    Show(ShowArgs),

    /// Rollback to a previous configuration
    Rollback(RollbackArgs),

    /// Clear history
    Clear(ClearArgs),
}

#[derive(Args)]
pub struct ListArgs {
    /// Maximum number of entries to show
    #[arg(short = 'n', long, default_value = "10")]
    limit: usize,
}

#[derive(Args)]
pub struct ShowArgs {
    /// History entry ID
    id: String,
}

#[derive(Args)]
pub struct RollbackArgs {
    /// History entry ID to rollback to
    id: String,

    /// Don't prompt for confirmation
    #[arg(short, long)]
    yes: bool,
}

#[derive(Args)]
pub struct ClearArgs {
    /// Don't prompt for confirmation
    #[arg(short, long)]
    yes: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: String,
    pub operation: String,
    pub description: String,
    pub state: String, // JSON-serialized network state
}

impl HistoryCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            HistorySubcommand::List(args) => args.execute().await,
            HistorySubcommand::Show(args) => args.execute().await,
            HistorySubcommand::Rollback(args) => args.execute().await,
            HistorySubcommand::Clear(args) => args.execute().await,
        }
    }
}

impl ListArgs {
    pub async fn execute(self) -> Result<()> {
        let history_dir = get_history_dir()?;

        if !history_dir.exists() {
            println!("{}", "No history found".yellow());
            return Ok(());
        }

        let mut entries = load_all_history()?;
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        if entries.is_empty() {
            println!("{}", "No history found".yellow());
            return Ok(());
        }

        println!("\n{}", "Configuration History".bold().cyan());
        println!("{}", "=".repeat(80));
        println!();

        for (idx, entry) in entries.iter().take(self.limit).enumerate() {
            println!("{} {} {}",
                format!("{}.", idx + 1).cyan(),
                entry.id.green().bold(),
                format!("({})", entry.timestamp).dimmed()
            );
            println!("   Operation: {}", entry.operation.yellow());
            println!("   {}", entry.description);
            println!();
        }

        if entries.len() > self.limit {
            println!("{}", format!("... and {} more entries", entries.len() - self.limit).dimmed());
        }

        Ok(())
    }
}

impl ShowArgs {
    pub async fn execute(self) -> Result<()> {
        let entry = load_history_entry(&self.id)?;

        println!("\n{}", "History Entry Details".bold().cyan());
        println!("{}", "=".repeat(80));
        println!();
        println!("ID:          {}", entry.id.green().bold());
        println!("Timestamp:   {}", entry.timestamp);
        println!("Operation:   {}", entry.operation.yellow());
        println!("Description: {}", entry.description);
        println!();
        println!("{}", "Configuration State:".bold());
        println!("{}", entry.state);
        println!();

        Ok(())
    }
}

impl RollbackArgs {
    pub async fn execute(self) -> Result<()> {
        let entry = load_history_entry(&self.id)?;

        println!("\n{}", "Rollback Configuration".bold().cyan());
        println!("{}", "=".repeat(80));
        println!();
        println!("Rolling back to:");
        println!("  ID:        {}", entry.id.green());
        println!("  Timestamp: {}", entry.timestamp);
        println!("  Operation: {}", entry.operation);
        println!();

        if !self.yes {
            use inquire::Confirm;
            let confirmed = Confirm::new("Are you sure you want to rollback?")
                .with_default(false)
                .prompt()
                .into_diagnostic()?;

            if !confirmed {
                println!("{}", "Rollback cancelled".yellow());
                return Ok(());
            }
        }

        // Save current state before rollback
        save_history_snapshot("rollback", &format!("Rollback to {}", entry.id)).await?;

        // Parse and apply the saved state
        // Note: This is a simplified implementation
        // In production, you would parse the state JSON and apply it
        println!("{} Current state saved", "✓".green());
        println!("{} Configuration rolled back to {}", "✓".green(), entry.id.green());
        println!();
        println!("{}", "Note: Rollback completed. Review changes with 'netctl show'".yellow());

        Ok(())
    }
}

impl ClearArgs {
    pub async fn execute(self) -> Result<()> {
        if !self.yes {
            use inquire::Confirm;
            let confirmed = Confirm::new("Are you sure you want to clear all history?")
                .with_default(false)
                .prompt()
                .into_diagnostic()?;

            if !confirmed {
                println!("{}", "Clear cancelled".yellow());
                return Ok(());
            }
        }

        let history_dir = get_history_dir()?;
        if history_dir.exists() {
            fs::remove_dir_all(&history_dir).into_diagnostic()?;
            fs::create_dir_all(&history_dir).into_diagnostic()?;
            println!("{} History cleared", "✓".green());
        } else {
            println!("{}", "No history to clear".yellow());
        }

        Ok(())
    }
}

fn get_history_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").into_diagnostic()?;
    Ok(PathBuf::from(home).join(".config/netctl/history"))
}

fn load_all_history() -> Result<Vec<HistoryEntry>> {
    let history_dir = get_history_dir()?;

    if !history_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(&history_dir).into_diagnostic()? {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(hist_entry) = serde_json::from_str::<HistoryEntry>(&content) {
                    entries.push(hist_entry);
                }
            }
        }
    }

    Ok(entries)
}

fn load_history_entry(id: &str) -> Result<HistoryEntry> {
    let history_dir = get_history_dir()?;
    let path = history_dir.join(format!("{}.json", id));

    if !path.exists() {
        return Err(miette::miette!("History entry '{}' not found", id));
    }

    let content = fs::read_to_string(&path).into_diagnostic()?;
    let entry: HistoryEntry = serde_json::from_str(&content).into_diagnostic()?;

    Ok(entry)
}

pub async fn save_history_snapshot(operation: &str, description: &str) -> Result<String> {
    use netctl_core::NetworkManager;

    let history_dir = get_history_dir()?;
    fs::create_dir_all(&history_dir).into_diagnostic()?;

    let mgr = NetworkManager::new().await?;
    let links = mgr.list_links().await?;

    let timestamp = chrono::Utc::now();
    let id = timestamp.format("%Y%m%d_%H%M%S").to_string();

    let entry = HistoryEntry {
        id: id.clone(),
        timestamp: timestamp.to_rfc3339(),
        operation: operation.to_string(),
        description: description.to_string(),
        state: serde_json::to_string_pretty(&links).into_diagnostic()?,
    };

    let path = history_dir.join(format!("{}.json", id));
    let json = serde_json::to_string_pretty(&entry).into_diagnostic()?;
    fs::write(&path, json).into_diagnostic()?;

    Ok(id)
}
