use clap::{Args, Subcommand};
use miette::{IntoDiagnostic, Result};
use netctl_core::NetworkManager;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ProfileCommand {
    #[command(subcommand)]
    command: ProfileSubcommand,
}

#[derive(Subcommand)]
pub enum ProfileSubcommand {
    /// Save current network configuration as a profile
    Save(SaveArgs),
    /// Load a network profile
    Load(LoadArgs),
    /// List all saved profiles
    List,
    /// Delete a profile
    Delete(DeleteArgs),
    /// Show profile details
    Show(ShowArgs),
}

#[derive(Args)]
pub struct SaveArgs {
    /// Profile name
    name: String,

    /// Profile description
    #[arg(short, long)]
    description: Option<String>,
}

#[derive(Args)]
pub struct LoadArgs {
    /// Profile name to load
    name: String,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// Profile name to delete
    name: String,
}

#[derive(Args)]
pub struct ShowArgs {
    /// Profile name to show
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkProfile {
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub interfaces: Vec<InterfaceConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterfaceConfig {
    pub name: String,
    pub state: String,
    pub mtu: u32,
    pub mac_address: Option<String>,
    pub addresses: Vec<String>,
}

impl ProfileCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            ProfileSubcommand::Save(args) => args.execute().await,
            ProfileSubcommand::Load(args) => args.execute().await,
            ProfileSubcommand::List => list_profiles().await,
            ProfileSubcommand::Delete(args) => args.execute().await,
            ProfileSubcommand::Show(args) => args.execute().await,
        }
    }
}

impl SaveArgs {
    pub async fn execute(self) -> Result<()> {
        let mgr = NetworkManager::new().await?;
        let links = mgr.list_links().await?;

        let interfaces: Vec<InterfaceConfig> = links
            .into_iter()
            .map(|link| InterfaceConfig {
                name: link.name,
                state: format!("{:?}", link.state),
                mtu: link.mtu,
                mac_address: link.mac_address.map(|m| m.to_string()),
                addresses: link.addresses.iter().map(|a| a.to_string()).collect(),
            })
            .collect();

        let profile = NetworkProfile {
            name: self.name.clone(),
            description: self.description,
            created_at: chrono::Utc::now().to_rfc3339(),
            interfaces,
        };

        let profile_dir = get_profile_dir()?;
        fs::create_dir_all(&profile_dir).into_diagnostic()?;

        let profile_path = profile_dir.join(format!("{}.yaml", self.name));
        let yaml = serde_yaml::to_string(&profile).into_diagnostic()?;
        fs::write(&profile_path, yaml).into_diagnostic()?;

        println!("✓ Profile '{}' saved to {}", self.name, profile_path.display());
        println!("  {} interface(s) saved", profile.interfaces.len());

        Ok(())
    }
}

impl LoadArgs {
    pub async fn execute(self) -> Result<()> {
        let profile = load_profile(&self.name)?;

        println!("Loading profile '{}'...", profile.name);
        if let Some(desc) = &profile.description {
            println!("Description: {}", desc);
        }

        let mgr = NetworkManager::new().await?;

        for iface in &profile.interfaces {
            println!("\nConfiguring {}...", iface.name);

            // Set state
            match iface.state.to_lowercase().as_str() {
                "up" => {
                    mgr.set_link_up(&iface.name).await?;
                    println!("  ✓ State: up");
                }
                "down" => {
                    mgr.set_link_down(&iface.name).await?;
                    println!("  ✓ State: down");
                }
                _ => {}
            }

            // Set MTU
            mgr.set_mtu(&iface.name, iface.mtu).await?;
            println!("  ✓ MTU: {}", iface.mtu);

            // TODO: Set addresses
            if !iface.addresses.is_empty() {
                println!("  ⚠ Address configuration not yet implemented");
                for addr in &iface.addresses {
                    println!("    - {}", addr);
                }
            }
        }

        println!("\n✓ Profile '{}' loaded successfully", profile.name);
        Ok(())
    }
}

impl DeleteArgs {
    pub async fn execute(self) -> Result<()> {
        let profile_path = get_profile_dir()?.join(format!("{}.yaml", self.name));

        if !profile_path.exists() {
            return Err(miette::miette!("Profile '{}' not found", self.name));
        }

        fs::remove_file(&profile_path).into_diagnostic()?;
        println!("✓ Profile '{}' deleted", self.name);

        Ok(())
    }
}

impl ShowArgs {
    pub async fn execute(self) -> Result<()> {
        let profile = load_profile(&self.name)?;

        println!("Profile: {}", profile.name);
        if let Some(desc) = &profile.description {
            println!("Description: {}", desc);
        }
        println!("Created: {}", profile.created_at);
        println!("\nInterfaces:");

        for iface in &profile.interfaces {
            println!("\n  {}:", iface.name);
            println!("    State: {}", iface.state);
            println!("    MTU: {}", iface.mtu);
            if let Some(mac) = &iface.mac_address {
                println!("    MAC: {}", mac);
            }
            if !iface.addresses.is_empty() {
                println!("    Addresses:");
                for addr in &iface.addresses {
                    println!("      - {}", addr);
                }
            }
        }

        Ok(())
    }
}

async fn list_profiles() -> Result<()> {
    let profile_dir = get_profile_dir()?;

    if !profile_dir.exists() {
        println!("No profiles found");
        return Ok(());
    }

    let entries = fs::read_dir(&profile_dir).into_diagnostic()?;
    let mut profiles = Vec::new();

    for entry in entries {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(profile) = serde_yaml::from_str::<NetworkProfile>(&content) {
                    profiles.push(profile);
                }
            }
        }
    }

    if profiles.is_empty() {
        println!("No profiles found");
        return Ok(());
    }

    println!("Available profiles:\n");
    for profile in profiles {
        println!("  {}", profile.name);
        if let Some(desc) = profile.description {
            println!("    Description: {}", desc);
        }
        println!("    Interfaces: {}", profile.interfaces.len());
        println!("    Created: {}", profile.created_at);
        println!();
    }

    Ok(())
}

fn get_profile_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").into_diagnostic()?;
    Ok(PathBuf::from(home).join(".config/netctl/profiles"))
}

fn load_profile(name: &str) -> Result<NetworkProfile> {
    let profile_path = get_profile_dir()?.join(format!("{}.yaml", name));

    if !profile_path.exists() {
        return Err(miette::miette!("Profile '{}' not found", name));
    }

    let content = fs::read_to_string(&profile_path).into_diagnostic()?;
    let profile: NetworkProfile = serde_yaml::from_str(&content).into_diagnostic()?;

    Ok(profile)
}
