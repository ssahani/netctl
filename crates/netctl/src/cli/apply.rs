use clap::Args;
use colored::Colorize;
use miette::{IntoDiagnostic, Result};
use netctl_core::NetworkManager;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ApplyArgs {
    /// Path to configuration file (YAML or TOML)
    file: PathBuf,

    /// Dry run - show what would be applied without making changes
    #[arg(long, short = 'n')]
    dry_run: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkConfig {
    interfaces: Vec<InterfaceConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InterfaceConfig {
    name: String,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    mtu: Option<u32>,
    #[serde(default)]
    addresses: Vec<String>,
}

impl ApplyArgs {
    pub async fn execute(self) -> Result<()> {
        // Read and parse configuration file
        let config = self.read_config()?;

        println!("{}", "Network Configuration Apply".bold().cyan());
        println!("{}", "=".repeat(60));
        println!("File: {}", self.file.display());

        if self.dry_run {
            println!("{}\n", "DRY RUN - No changes will be made".yellow().bold());
        }
        println!();

        let mgr = NetworkManager::new().await?;

        for iface in &config.interfaces {
            self.apply_interface(&mgr, iface).await?;
        }

        if self.dry_run {
            println!("\n{}", "Dry run complete - no changes were made".yellow());
        } else {
            println!("\n{}", "Configuration applied successfully!".green().bold());
        }

        Ok(())
    }

    fn read_config(&self) -> Result<NetworkConfig> {
        let content = fs::read_to_string(&self.file).into_diagnostic()?;

        let config = if self.file.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&content).into_diagnostic()?
        } else {
            // Default to YAML
            serde_yaml::from_str(&content).into_diagnostic()?
        };

        Ok(config)
    }

    async fn apply_interface(&self, mgr: &NetworkManager, iface: &InterfaceConfig) -> Result<()> {
        println!("{} {}", "Configuring".cyan().bold(), iface.name.cyan());

        // Apply state
        if let Some(state) = &iface.state {
            match state.to_lowercase().as_str() {
                "up" => {
                    if self.dry_run {
                        println!("  {} Would bring interface up", "→".blue());
                    } else {
                        mgr.set_link_up(&iface.name).await?;
                        println!("  {} Interface is now up", "✓".green());
                    }
                }
                "down" => {
                    if self.dry_run {
                        println!("  {} Would bring interface down", "→".blue());
                    } else {
                        mgr.set_link_down(&iface.name).await?;
                        println!("  {} Interface is now down", "✓".green());
                    }
                }
                _ => {
                    println!("  {} Invalid state '{}', skipping", "⚠".yellow(), state);
                }
            }
        }

        // Apply MTU
        if let Some(mtu) = iface.mtu {
            if self.dry_run {
                println!("  {} Would set MTU to {}", "→".blue(), mtu);
            } else {
                mgr.set_mtu(&iface.name, mtu).await?;
                println!("  {} MTU set to {}", "✓".green(), mtu);
            }
        }

        // Apply addresses
        for addr in &iface.addresses {
            if self.dry_run {
                println!("  {} Would add address {}", "→".blue(), addr);
            } else {
                // Parse address
                match addr.parse() {
                    Ok(ip_net) => {
                        mgr.add_address(&iface.name, ip_net).await?;
                        println!("  {} Added address {}", "✓".green(), addr);
                    }
                    Err(_) => {
                        println!("  {} Invalid address '{}', skipping", "⚠".yellow(), addr);
                    }
                }
            }
        }

        Ok(())
    }
}
