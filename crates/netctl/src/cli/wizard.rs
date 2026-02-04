use clap::Args;
use colored::Colorize;
use inquire::{Confirm, Select, Text};
use miette::{IntoDiagnostic, Result};
use netctl_core::NetworkManager;

#[derive(Args)]
pub struct WizardArgs {}

impl WizardArgs {
    pub async fn execute(self) -> Result<()> {
        println!("{}", "⚡ netctl Configuration Wizard".bold().cyan());
        println!("{}", "=".repeat(60));
        println!("This wizard will guide you through common network configuration tasks.\n");

        let mgr = NetworkManager::new().await?;

        // Get available interfaces
        let links = mgr.list_links().await?;
        let interface_names: Vec<String> = links.iter().map(|l| l.name.clone()).collect();

        if interface_names.is_empty() {
            println!("{}", "No network interfaces found!".red());
            return Ok(());
        }

        // Select interface
        let interface = Select::new("Select network interface:", interface_names)
            .prompt()
            .into_diagnostic()?;

        println!("\n{} Selected interface: {}\n", "✓".green(), interface.cyan());

        // What to configure
        let options = vec![
            "Configure Static IP",
            "Enable/Disable Interface",
            "Set MTU",
            "Configure for DHCP (preparation)",
            "Complete Network Setup",
        ];

        let choice = Select::new("What would you like to configure?", options)
            .prompt()
            .into_diagnostic()?;

        match choice {
            "Configure Static IP" => self.wizard_static_ip(&mgr, &interface).await?,
            "Enable/Disable Interface" => self.wizard_interface_state(&mgr, &interface).await?,
            "Set MTU" => self.wizard_mtu(&mgr, &interface).await?,
            "Configure for DHCP (preparation)" => self.wizard_dhcp(&mgr, &interface).await?,
            "Complete Network Setup" => self.wizard_complete(&mgr, &interface).await?,
            _ => {}
        }

        Ok(())
    }

    async fn wizard_static_ip(&self, mgr: &NetworkManager, interface: &str) -> Result<()> {
        println!("\n{}", "Configure Static IP".bold());

        let ip_address = Text::new("Enter IP address (e.g., 192.168.1.100):")
            .prompt()
            .into_diagnostic()?;

        let netmask = Text::new("Enter network prefix/mask (e.g., 24 for /24):")
            .with_default("24")
            .prompt()
            .into_diagnostic()?;

        let full_address = format!("{}/{}", ip_address, netmask);

        println!("\n{}", "Summary:".bold());
        println!("  Interface: {}", interface.cyan());
        println!("  Address: {}", full_address.green());

        if Confirm::new("Apply this configuration?")
            .with_default(true)
            .prompt()
            .into_diagnostic()?
        {
            match full_address.parse() {
                Ok(ip_net) => {
                    mgr.add_address(interface, ip_net).await?;
                    println!("\n{} Configuration applied!", "✓".green());
                }
                Err(_) => {
                    println!("\n{} Invalid IP address format", "✗".red());
                }
            }
        } else {
            println!("\n{} Configuration cancelled", "ℹ".blue());
        }

        Ok(())
    }

    async fn wizard_interface_state(&self, mgr: &NetworkManager, interface: &str) -> Result<()> {
        println!("\n{}", "Enable/Disable Interface".bold());

        let state = Select::new("Select desired state:", vec!["up", "down"])
            .prompt()
            .into_diagnostic()?;

        if Confirm::new(&format!("Set {} to {}?", interface.cyan(), state))
            .with_default(true)
            .prompt()
            .into_diagnostic()?
        {
            match state {
                "up" => {
                    mgr.set_link_up(interface).await?;
                    println!("\n{} Interface {} is now up", "✓".green(), interface.cyan());
                }
                "down" => {
                    mgr.set_link_down(interface).await?;
                    println!("\n{} Interface {} is now down", "✓".green(), interface.cyan());
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn wizard_mtu(&self, mgr: &NetworkManager, interface: &str) -> Result<()> {
        println!("\n{}", "Configure MTU".bold());

        let mtu_options = vec![
            "1500 (Standard Ethernet)",
            "9000 (Jumbo Frames)",
            "1280 (IPv6 minimum)",
            "Custom value",
        ];

        let choice = Select::new("Select MTU:", mtu_options)
            .prompt()
            .into_diagnostic()?;

        let mtu: u32 = match choice {
            "1500 (Standard Ethernet)" => 1500,
            "9000 (Jumbo Frames)" => 9000,
            "1280 (IPv6 minimum)" => 1280,
            "Custom value" => {
                let custom = Text::new("Enter MTU value:")
                    .prompt()
                    .into_diagnostic()?;
                custom.parse().into_diagnostic()?
            }
            _ => 1500,
        };

        if Confirm::new(&format!("Set MTU to {} for {}?", mtu, interface.cyan()))
            .with_default(true)
            .prompt()
            .into_diagnostic()?
        {
            mgr.set_mtu(interface, mtu).await?;
            println!("\n{} MTU set to {} for {}", "✓".green(), mtu, interface.cyan());
        }

        Ok(())
    }

    async fn wizard_dhcp(&self, mgr: &NetworkManager, interface: &str) -> Result<()> {
        println!("\n{}", "Prepare Interface for DHCP".bold());
        println!("This will bring the interface up and set standard MTU.");
        println!("Note: Actual DHCP configuration requires systemd-networkd.");

        if Confirm::new("Proceed?")
            .with_default(true)
            .prompt()
            .into_diagnostic()?
        {
            // Set MTU to standard
            mgr.set_mtu(interface, 1500).await?;
            println!("  {} MTU set to 1500", "✓".green());

            // Bring interface up
            mgr.set_link_up(interface).await?;
            println!("  {} Interface is up", "✓".green());

            println!("\n{} Interface prepared for DHCP", "✓".green());
            println!("Configure systemd-networkd for actual DHCP client.");
        }

        Ok(())
    }

    async fn wizard_complete(&self, mgr: &NetworkManager, interface: &str) -> Result<()> {
        println!("\n{}", "Complete Network Setup".bold());
        println!("This wizard will configure all aspects of the interface.\n");

        // State
        let state = Select::new("Interface state:", vec!["up", "down"])
            .prompt()
            .into_diagnostic()?;

        // MTU
        let mtu_str = Text::new("MTU:")
            .with_default("1500")
            .prompt()
            .into_diagnostic()?;
        let mtu: u32 = mtu_str.parse().into_diagnostic()?;

        // IP configuration
        let use_static = Confirm::new("Configure static IP?")
            .with_default(true)
            .prompt()
            .into_diagnostic()?;

        let address = if use_static {
            let ip = Text::new("IP address:").prompt().into_diagnostic()?;
            let prefix = Text::new("Network prefix:")
                .with_default("24")
                .prompt()
                .into_diagnostic()?;
            Some(format!("{}/{}", ip, prefix))
        } else {
            None
        };

        // Summary
        println!("\n{}", "Configuration Summary:".bold());
        println!("  Interface: {}", interface.cyan());
        println!("  State: {}", state);
        println!("  MTU: {}", mtu);
        if let Some(ref addr) = address {
            println!("  Address: {}", addr.green());
        } else {
            println!("  Address: {} (DHCP)", "None".yellow());
        }

        if Confirm::new("\nApply this configuration?")
            .with_default(true)
            .prompt()
            .into_diagnostic()?
        {
            // Apply MTU
            mgr.set_mtu(interface, mtu).await?;
            println!("  {} MTU configured", "✓".green());

            // Apply address if static
            if let Some(addr) = address {
                match addr.parse() {
                    Ok(ip_net) => {
                        mgr.add_address(interface, ip_net).await?;
                        println!("  {} Address configured", "✓".green());
                    }
                    Err(_) => {
                        println!("  {} Invalid address, skipping", "⚠".yellow());
                    }
                }
            }

            // Apply state
            match state {
                "up" => {
                    mgr.set_link_up(interface).await?;
                    println!("  {} Interface is up", "✓".green());
                }
                "down" => {
                    mgr.set_link_down(interface).await?;
                    println!("  {} Interface is down", "✓".green());
                }
                _ => {}
            }

            println!("\n{} Complete configuration applied!", "✓".green().bold());
        } else {
            println!("\n{} Configuration cancelled", "ℹ".blue());
        }

        Ok(())
    }
}
