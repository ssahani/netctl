use clap::Args;
use colored::Colorize;
use miette::Result;
use netctl_core::NetworkManager;
use std::collections::HashSet;

#[derive(Args)]
pub struct DiffArgs {
    /// First profile/state to compare ('current' for current state)
    first: String,

    /// Second profile/state to compare
    second: String,
}

impl DiffArgs {
    pub async fn execute(self) -> Result<()> {
        let first_state = self.load_state(&self.first).await?;
        let second_state = self.load_state(&self.second).await?;

        println!("\n{}", "Network Configuration Diff".bold().cyan());
        println!("{}", "=".repeat(60));
        println!("Comparing: {} vs {}\n", self.first.green(), self.second.yellow());

        self.compare_states(&first_state, &second_state)?;

        Ok(())
    }

    async fn load_state(&self, name: &str) -> Result<Vec<InterfaceState>> {
        if name == "current" {
            // Load current network state
            let mgr = NetworkManager::new().await?;
            let links = mgr.list_links().await?;

            Ok(links
                .into_iter()
                .map(|link| InterfaceState {
                    name: link.name,
                    state: format!("{:?}", link.state),
                    mtu: link.mtu,
                    mac: link.mac_address.map(|m| m.to_string()),
                    addresses: link.addresses.iter().map(|a| a.to_string()).collect(),
                })
                .collect())
        } else {
            // Load from profile
            use crate::cli::profile::load_profile;
            let profile = load_profile(name)?;

            Ok(profile
                .interfaces
                .into_iter()
                .map(|iface| InterfaceState {
                    name: iface.name,
                    state: iface.state,
                    mtu: iface.mtu,
                    mac: iface.mac_address,
                    addresses: iface.addresses,
                })
                .collect())
        }
    }

    fn compare_states(&self, first: &[InterfaceState], second: &[InterfaceState]) -> Result<()> {
        let first_names: HashSet<_> = first.iter().map(|i| &i.name).collect();
        let second_names: HashSet<_> = second.iter().map(|i| &i.name).collect();

        // Find interfaces only in first
        for name in first_names.difference(&second_names) {
            println!("{} Interface '{}' (only in {})",
                "-".red(), name.red(), self.first);
        }

        // Find interfaces only in second
        for name in second_names.difference(&first_names) {
            println!("{} Interface '{}' (only in {})",
                "+".green(), name.green(), self.second);
        }

        // Compare common interfaces
        for iface1 in first {
            if let Some(iface2) = second.iter().find(|i| i.name == iface1.name) {
                self.compare_interface(iface1, iface2)?;
            }
        }

        Ok(())
    }

    fn compare_interface(&self, iface1: &InterfaceState, iface2: &InterfaceState) -> Result<()> {
        // Check for differences
        if iface1.state != iface2.state
            || iface1.mtu != iface2.mtu
            || iface1.mac != iface2.mac
            || iface1.addresses != iface2.addresses
        {
            println!("\n{} {}:", "~".yellow(), iface1.name.yellow());
        }

        if iface1.state != iface2.state {
            println!("  {} state: {} → {}",
                "~".yellow(),
                iface1.state.red(),
                iface2.state.green()
            );
        }

        if iface1.mtu != iface2.mtu {
            println!("  {} mtu: {} → {}",
                "~".yellow(),
                iface1.mtu.to_string().red(),
                iface2.mtu.to_string().green()
            );
        }

        if iface1.mac != iface2.mac {
            println!("  {} mac: {} → {}",
                "~".yellow(),
                iface1.mac.as_deref().unwrap_or("none").red(),
                iface2.mac.as_deref().unwrap_or("none").green()
            );
        }

        if iface1.addresses != iface2.addresses {
            let addr1_set: HashSet<_> = iface1.addresses.iter().collect();
            let addr2_set: HashSet<_> = iface2.addresses.iter().collect();

            for addr in addr1_set.difference(&addr2_set) {
                println!("  {} address: {}", "-".red(), addr.red());
            }

            for addr in addr2_set.difference(&addr1_set) {
                println!("  {} address: {}", "+".green(), addr.green());
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct InterfaceState {
    name: String,
    state: String,
    mtu: u32,
    mac: Option<String>,
    addresses: Vec<String>,
}
