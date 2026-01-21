use clap::Args;
use miette::Result;
use netctl_core::NetworkManager;
use netctl_types::{LinkInfo, LinkState};

#[derive(Args)]
pub struct ShowArgs {
    /// Interface name (if not specified, shows all interfaces)
    interface: Option<String>,

    /// Output in JSON format
    #[arg(short, long)]
    json: bool,
}

impl ShowArgs {
    pub async fn execute(self) -> Result<()> {
        let mgr = NetworkManager::new().await?;

        if let Some(ref ifname) = self.interface {
            // Show specific interface
            let link = mgr.get_link_info(ifname).await?;

            if self.json {
                let json = serde_json::to_string_pretty(&link)
                    .map_err(|e| miette::miette!("JSON serialization failed: {}", e))?;
                println!("{}", json);
            } else {
                print_link_details(&link);
            }
        } else {
            // Show all interfaces
            let links = mgr.list_links().await?;

            if self.json {
                let json = serde_json::to_string_pretty(&links)
                    .map_err(|e| miette::miette!("JSON serialization failed: {}", e))?;
                println!("{}", json);
            } else {
                print_links_table(&links);
            }
        }

        Ok(())
    }
}

fn print_links_table(links: &[LinkInfo]) {
    println!(
        "{:<5} {:<15} {:<8} {:<8} {:<20}",
        "INDEX", "NAME", "STATE", "MTU", "MAC ADDRESS"
    );
    println!("{}", "-".repeat(60));

    for link in links {
        let state_str = match link.state {
            LinkState::Up => "UP",
            LinkState::Down => "DOWN",
        };

        let mac_str = link
            .mac_address
            .map(|m| m.to_string())
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{:<5} {:<15} {:<8} {:<8} {:<20}",
            link.index, link.name, state_str, link.mtu, mac_str
        );
    }

    println!("\nTotal: {} interface(s)", links.len());
}

fn print_link_details(link: &LinkInfo) {
    println!("Interface: {}", link.name);
    println!("  Index: {}", link.index);
    println!(
        "  State: {}",
        match link.state {
            LinkState::Up => "UP",
            LinkState::Down => "DOWN",
        }
    );
    println!("  MTU: {}", link.mtu);

    if let Some(mac) = link.mac_address {
        println!("  MAC Address: {}", mac);
    }

    if !link.addresses.is_empty() {
        println!("  Addresses:");
        for addr in &link.addresses {
            println!("    {}", addr);
        }
    }
}
