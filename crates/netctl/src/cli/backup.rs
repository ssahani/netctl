use clap::{Args, Subcommand};
use colored::Colorize;
use miette::{IntoDiagnostic, Result};
use netctl_core::NetworkManager;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct BackupCommand {
    #[command(subcommand)]
    command: BackupSubcommand,
}

#[derive(Subcommand)]
pub enum BackupSubcommand {
    /// Create a backup of current network configuration
    Create(CreateArgs),

    /// List all backups
    List,

    /// Restore from a backup
    Restore(RestoreArgs),

    /// Delete a backup
    Delete(DeleteArgs),

    /// Export backup to file
    Export(ExportArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    /// Backup name
    name: String,

    /// Backup description
    #[arg(short, long)]
    description: Option<String>,
}

#[derive(Args)]
pub struct RestoreArgs {
    /// Backup name to restore
    name: String,

    /// Don't prompt for confirmation
    #[arg(short, long)]
    yes: bool,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// Backup name to delete
    name: String,
}

#[derive(Args)]
pub struct ExportArgs {
    /// Backup name to export
    name: String,

    /// Output file path
    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Backup {
    name: String,
    description: Option<String>,
    created_at: String,
    interfaces: Vec<InterfaceBackup>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InterfaceBackup {
    name: String,
    index: u32,
    state: String,
    mtu: u32,
    mac_address: Option<String>,
    addresses: Vec<String>,
}

impl BackupCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            BackupSubcommand::Create(args) => args.execute().await,
            BackupSubcommand::List => list_backups().await,
            BackupSubcommand::Restore(args) => args.execute().await,
            BackupSubcommand::Delete(args) => args.execute().await,
            BackupSubcommand::Export(args) => args.execute().await,
        }
    }
}

impl CreateArgs {
    pub async fn execute(self) -> Result<()> {
        println!("{}", "Creating network backup...".cyan().bold());

        let mgr = NetworkManager::new().await?;
        let links = mgr.list_links().await?;

        let interfaces: Vec<InterfaceBackup> = links
            .into_iter()
            .map(|link| InterfaceBackup {
                name: link.name,
                index: link.index,
                state: format!("{:?}", link.state),
                mtu: link.mtu,
                mac_address: link.mac_address.map(|m| m.to_string()),
                addresses: link.addresses.iter().map(|a| a.to_string()).collect(),
            })
            .collect();

        let backup = Backup {
            name: self.name.clone(),
            description: self.description,
            created_at: chrono::Utc::now().to_rfc3339(),
            interfaces,
        };

        let backup_dir = get_backup_dir()?;
        fs::create_dir_all(&backup_dir).into_diagnostic()?;

        let backup_path = backup_dir.join(format!("{}.json", self.name));
        let json = serde_json::to_string_pretty(&backup).into_diagnostic()?;
        fs::write(&backup_path, json).into_diagnostic()?;

        println!("{} Backup '{}' created successfully", "✓".green(), self.name.green());
        println!("  Location: {}", backup_path.display());
        println!("  Interfaces: {}", backup.interfaces.len());
        println!();

        Ok(())
    }
}

impl RestoreArgs {
    pub async fn execute(self) -> Result<()> {
        let backup = load_backup(&self.name)?;

        println!("{}", "Restore Network Configuration".bold().cyan());
        println!("{}", "=".repeat(80));
        println!();
        println!("Backup: {}", backup.name.green());
        if let Some(desc) = &backup.description {
            println!("Description: {}", desc);
        }
        println!("Created: {}", backup.created_at);
        println!("Interfaces: {}", backup.interfaces.len());
        println!();

        if !self.yes {
            use inquire::Confirm;
            let confirmed = Confirm::new("Restore this backup?")
                .with_default(false)
                .prompt()
                .into_diagnostic()?;

            if !confirmed {
                println!("{}", "Restore cancelled".yellow());
                return Ok(());
            }
        }

        // Create a backup of current state before restoring
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let auto_backup_name = format!("auto_before_restore_{}", timestamp);

        println!("Creating automatic backup of current state...");
        let mgr = NetworkManager::new().await?;
        let current_links = mgr.list_links().await?;

        let auto_backup = Backup {
            name: auto_backup_name.clone(),
            description: Some(format!("Automatic backup before restoring '{}'", backup.name)),
            created_at: chrono::Utc::now().to_rfc3339(),
            interfaces: current_links
                .into_iter()
                .map(|link| InterfaceBackup {
                    name: link.name,
                    index: link.index,
                    state: format!("{:?}", link.state),
                    mtu: link.mtu,
                    mac_address: link.mac_address.map(|m| m.to_string()),
                    addresses: link.addresses.iter().map(|a| a.to_string()).collect(),
                })
                .collect(),
        };

        let backup_dir = get_backup_dir()?;
        let auto_backup_path = backup_dir.join(format!("{}.json", auto_backup_name));
        let json = serde_json::to_string_pretty(&auto_backup).into_diagnostic()?;
        fs::write(&auto_backup_path, json).into_diagnostic()?;

        println!("{} Current state backed up as '{}'", "✓".green(), auto_backup_name.cyan());
        println!();

        // Apply backup configuration
        println!("Restoring configuration...");
        for iface in &backup.interfaces {
            println!("  {} {}", "→".blue(), iface.name);

            // Set MTU
            mgr.set_mtu(&iface.name, iface.mtu).await?;

            // Set state
            match iface.state.as_str() {
                "Up" => {
                    mgr.set_link_up(&iface.name).await?;
                }
                "Down" => {
                    mgr.set_link_down(&iface.name).await?;
                }
                _ => {}
            }

            // Note: Address restoration would require address deletion support
        }

        println!();
        println!("{} Backup restored successfully", "✓".green().bold());
        println!("{}", "Note: Address restoration requires manual configuration".yellow());

        Ok(())
    }
}

impl DeleteArgs {
    pub async fn execute(self) -> Result<()> {
        let backup_path = get_backup_dir()?.join(format!("{}.json", self.name));

        if !backup_path.exists() {
            return Err(miette::miette!("Backup '{}' not found", self.name));
        }

        fs::remove_file(&backup_path).into_diagnostic()?;
        println!("{} Backup '{}' deleted", "✓".green(), self.name);

        Ok(())
    }
}

impl ExportArgs {
    pub async fn execute(self) -> Result<()> {
        let backup = load_backup(&self.name)?;

        let json = serde_json::to_string_pretty(&backup).into_diagnostic()?;
        fs::write(&self.output, json).into_diagnostic()?;

        println!("{} Backup '{}' exported to {}",
            "✓".green(),
            self.name.green(),
            self.output.display()
        );

        Ok(())
    }
}

async fn list_backups() -> Result<()> {
    let backup_dir = get_backup_dir()?;

    if !backup_dir.exists() {
        println!("{}", "No backups found".yellow());
        return Ok(());
    }

    let entries = fs::read_dir(&backup_dir).into_diagnostic()?;
    let mut backups = Vec::new();

    for entry in entries {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(backup) = serde_json::from_str::<Backup>(&content) {
                    backups.push(backup);
                }
            }
        }
    }

    if backups.is_empty() {
        println!("{}", "No backups found".yellow());
        return Ok(());
    }

    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    println!("\n{}", "Network Backups".bold().cyan());
    println!("{}", "=".repeat(80));
    println!();

    for backup in backups {
        println!("{}", backup.name.green().bold());
        if let Some(desc) = backup.description {
            println!("  Description: {}", desc);
        }
        println!("  Created: {}", backup.created_at);
        println!("  Interfaces: {}", backup.interfaces.len());
        println!();
    }

    Ok(())
}

fn get_backup_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").into_diagnostic()?;
    Ok(PathBuf::from(home).join(".config/netctl/backups"))
}

fn load_backup(name: &str) -> Result<Backup> {
    let backup_path = get_backup_dir()?.join(format!("{}.json", name));

    if !backup_path.exists() {
        return Err(miette::miette!("Backup '{}' not found", name));
    }

    let content = fs::read_to_string(&backup_path).into_diagnostic()?;
    let backup: Backup = serde_json::from_str(&content).into_diagnostic()?;

    Ok(backup)
}
