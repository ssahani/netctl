use clap::Args;
use colored::Colorize;
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Args)]
pub struct ValidateArgs {
    /// Path to configuration file to validate
    file: PathBuf,

    /// Strict mode - fail on warnings
    #[arg(long, short)]
    strict: bool,
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

impl ValidateArgs {
    pub async fn execute(self) -> Result<()> {
        println!("{}", "Validating network configuration...".bold().cyan());
        println!("File: {}\n", self.file.display());

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Read file
        let content = match fs::read_to_string(&self.file) {
            Ok(c) => c,
            Err(e) => {
                println!("{} Failed to read file: {}", "✗".red(), e);
                return Ok(());
            }
        };

        // Parse based on extension
        let config: NetworkConfig = if self.file.extension().and_then(|s| s.to_str()) == Some("toml") {
            match toml::from_str(&content) {
                Ok(c) => c,
                Err(e) => {
                    println!("{} Invalid TOML: {}", "✗".red(), e);
                    return Ok(());
                }
            }
        } else {
            match serde_yaml::from_str(&content) {
                Ok(c) => c,
                Err(e) => {
                    println!("{} Invalid YAML: {}", "✗".red(), e);
                    return Ok(());
                }
            }
        };

        println!("{} Configuration file parsed successfully", "✓".green());
        println!();

        // Validate each interface
        for (idx, iface) in config.interfaces.iter().enumerate() {
            println!("Validating interface {} ({})...", idx + 1, iface.name.cyan());

            self.validate_interface(iface, &mut errors, &mut warnings);
        }

        // Print summary
        println!();
        println!("{}", "=".repeat(60));

        if errors.is_empty() && warnings.is_empty() {
            println!("{}", "✓ Validation passed! No issues found.".green().bold());
            return Ok(());
        }

        if !warnings.is_empty() {
            println!("\n{} ({}):", "Warnings".yellow().bold(), warnings.len());
            for warning in &warnings {
                println!("  {} {}", "⚠".yellow(), warning);
            }
        }

        if !errors.is_empty() {
            println!("\n{} ({}):", "Errors".red().bold(), errors.len());
            for error in &errors {
                println!("  {} {}", "✗".red(), error);
            }
        }

        println!();
        if !errors.is_empty() {
            println!("{}", "✗ Validation failed!".red().bold());
            std::process::exit(1);
        } else if self.strict && !warnings.is_empty() {
            println!("{}", "✗ Validation failed in strict mode (warnings present)".red().bold());
            std::process::exit(1);
        } else {
            println!("{}", "✓ Validation passed with warnings".yellow().bold());
        }

        Ok(())
    }

    fn validate_interface(&self, iface: &InterfaceConfig, errors: &mut Vec<String>, warnings: &mut Vec<String>) {
        // Validate interface name
        if iface.name.is_empty() {
            errors.push(format!("Interface name cannot be empty"));
        } else if iface.name.len() > 15 {
            errors.push(format!(
                "Interface name '{}' is too long (max 15 characters)",
                iface.name
            ));
        }

        // Validate state
        if let Some(ref state) = iface.state {
            match state.to_lowercase().as_str() {
                "up" | "down" => {}
                _ => errors.push(format!(
                    "Invalid state '{}' for interface '{}' (must be 'up' or 'down')",
                    state, iface.name
                )),
            }
        }

        // Validate MTU
        if let Some(mtu) = iface.mtu {
            if mtu < 68 {
                errors.push(format!(
                    "MTU {} for interface '{}' is too small (minimum 68)",
                    mtu, iface.name
                ));
            } else if mtu > 65535 {
                errors.push(format!(
                    "MTU {} for interface '{}' is too large (maximum 65535)",
                    mtu, iface.name
                ));
            } else if mtu > 9000 && mtu != 65535 {
                warnings.push(format!(
                    "MTU {} for interface '{}' is unusually large (standard max is 9000)",
                    mtu, iface.name
                ));
            } else if mtu == 9000 {
                warnings.push(format!(
                    "Jumbo frames (MTU 9000) on interface '{}' require network infrastructure support",
                    iface.name
                ));
            }
        }

        // Validate addresses
        for addr in &iface.addresses {
            if let Err(e) = self.validate_ip_network(addr) {
                errors.push(format!(
                    "Invalid address '{}' for interface '{}': {}",
                    addr, iface.name, e
                ));
            }
        }

        if iface.addresses.is_empty() {
            warnings.push(format!(
                "Interface '{}' has no addresses configured",
                iface.name
            ));
        }

        // Check for duplicate addresses
        let mut seen_addresses = std::collections::HashSet::new();
        for addr in &iface.addresses {
            if !seen_addresses.insert(addr) {
                warnings.push(format!(
                    "Duplicate address '{}' on interface '{}'",
                    addr, iface.name
                ));
            }
        }

        println!("  {} Validated", "✓".green());
    }

    fn validate_ip_network(&self, addr: &str) -> Result<()> {
        // Check format: IP/PREFIX
        let parts: Vec<&str> = addr.split('/').collect();
        if parts.len() != 2 {
            return Err(miette::miette!(
                "Address must be in format IP/PREFIX (e.g., 192.168.1.100/24)"
            ));
        }

        // Validate IP
        let _ip: IpAddr = parts[0].parse().into_diagnostic()?;

        // Validate prefix
        let prefix: u8 = parts[1].parse().into_diagnostic()?;

        // Check prefix range based on IP version
        if parts[0].contains(':') {
            // IPv6
            if prefix > 128 {
                return Err(miette::miette!("IPv6 prefix must be 0-128"));
            }
        } else {
            // IPv4
            if prefix > 32 {
                return Err(miette::miette!("IPv4 prefix must be 0-32"));
            }
        }

        Ok(())
    }
}
