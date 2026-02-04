use clap::Args;
use colored::Colorize;
use miette::{IntoDiagnostic, Result};
use netctl_core::NetworkManager;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ExportArgs {
    /// Output file path
    output: PathBuf,

    /// Export format (yaml, toml, json)
    #[arg(short, long, default_value = "yaml")]
    format: String,

    /// Include only these interfaces (comma-separated)
    #[arg(long)]
    interfaces: Option<String>,

    /// Pretty print output
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExportConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<ExportMetadata>,
    interfaces: Vec<InterfaceExport>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExportMetadata {
    exported_at: String,
    exported_by: String,
    hostname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InterfaceExport {
    name: String,
    state: String,
    mtu: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    mac_address: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    addresses: Vec<String>,
}

impl ExportArgs {
    pub async fn execute(self) -> Result<()> {
        println!("{}", "Exporting network configuration...".cyan().bold());
        println!();

        let mgr = NetworkManager::new().await?;
        let links = mgr.list_links().await?;

        // Filter interfaces if specified
        let filtered_links: Vec<_> = if let Some(ref iface_list) = self.interfaces {
            let names: Vec<&str> = iface_list.split(',').map(|s| s.trim()).collect();
            links.into_iter().filter(|l| names.contains(&l.name.as_str())).collect()
        } else {
            links
        };

        if filtered_links.is_empty() {
            println!("{}", "No interfaces to export".yellow());
            return Ok(());
        }

        let interfaces: Vec<InterfaceExport> = filtered_links
            .into_iter()
            .map(|link| InterfaceExport {
                name: link.name,
                state: format!("{:?}", link.state).to_lowercase(),
                mtu: link.mtu,
                mac_address: link.mac_address.map(|m| m.to_string()),
                addresses: link.addresses.iter().map(|a| a.to_string()).collect(),
            })
            .collect();

        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("HOST"))
            .ok();

        let config = ExportConfig {
            metadata: Some(ExportMetadata {
                exported_at: chrono::Utc::now().to_rfc3339(),
                exported_by: "netctl".to_string(),
                hostname,
            }),
            interfaces,
        };

        let content = match self.format.to_lowercase().as_str() {
            "json" => {
                if self.pretty {
                    serde_json::to_string_pretty(&config).into_diagnostic()?
                } else {
                    serde_json::to_string(&config).into_diagnostic()?
                }
            }
            "toml" => toml::to_string_pretty(&config).into_diagnostic()?,
            "yaml" | _ => serde_yaml::to_string(&config).into_diagnostic()?,
        };

        fs::write(&self.output, content).into_diagnostic()?;

        println!("{} Configuration exported successfully", "✓".green());
        println!("  Format: {}", self.format.yellow());
        println!("  Interfaces: {}", config.interfaces.len());
        println!("  Output: {}", self.output.display());
        println!();

        Ok(())
    }
}
