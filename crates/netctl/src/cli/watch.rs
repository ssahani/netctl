use clap::Args;
use miette::Result;
use netctl_core::NetworkManager;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Args)]
pub struct WatchArgs {
    /// Interface to watch (optional, watches all if not specified)
    interface: Option<String>,

    /// Refresh interval in seconds
    #[arg(short, long, default_value = "1")]
    interval: u64,
}

impl WatchArgs {
    pub async fn execute(self) -> Result<()> {
        let mgr = NetworkManager::new().await?;
        let interval = Duration::from_secs(self.interval);

        println!("Watching network interfaces (Ctrl+C to stop)");
        println!("Refresh interval: {}s\n", self.interval);

        loop {
            // Clear screen
            print!("\x1B[2J\x1B[1;1H");

            // Print timestamp
            println!("Updated: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));

            // Get interface data
            match mgr.list_links().await {
                Ok(links) => {
                    let filtered_links: Vec<_> = if let Some(ref name) = self.interface {
                        links.into_iter().filter(|l| &l.name == name).collect()
                    } else {
                        links
                    };

                    // Print header
                    println!("{:<6} {:<15} {:<8} {:<8} {:<20}", "INDEX", "NAME", "STATE", "MTU", "MAC ADDRESS");
                    println!("{}", "-".repeat(65));

                    // Print interfaces
                    for link in filtered_links {
                        println!(
                            "{:<6} {:<15} {:<8} {:<8} {:<20}",
                            link.index,
                            link.name,
                            format!("{:?}", link.state),
                            link.mtu,
                            link.mac_address.as_ref().map(|m| m.to_string()).unwrap_or_else(|| "-".to_string())
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching interfaces: {}", e);
                }
            }

            sleep(interval).await;
        }
    }
}
