use clap::Args;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};
use miette::Result;
use netctl_core::NetworkManager;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct StatsArgs {
    /// Interface to show statistics for (optional, shows all if not specified)
    interface: Option<String>,

    /// Show detailed statistics
    #[arg(long, short)]
    detailed: bool,

    /// Output format (table, json)
    #[arg(long, short = 'f', default_value = "table")]
    format: String,
}

impl StatsArgs {
    pub async fn execute(self) -> Result<()> {
        let mgr = NetworkManager::new().await?;
        let links = mgr.list_links().await?;

        let filtered_links: Vec<_> = if let Some(ref name) = self.interface {
            links.into_iter().filter(|l| &l.name == name).collect()
        } else {
            links
        };

        if filtered_links.is_empty() {
            println!("{}", "No interfaces found".yellow());
            return Ok(());
        }

        match self.format.as_str() {
            "json" => self.print_json(&filtered_links)?,
            _ => self.print_table(&filtered_links)?,
        }

        Ok(())
    }

    fn print_table(&self, links: &[netctl_types::network::LinkInfo]) -> Result<()> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        // Add header
        if self.detailed {
            table.set_header(vec![
                Cell::new("Interface").fg(Color::Cyan),
                Cell::new("State").fg(Color::Cyan),
                Cell::new("RX Bytes").fg(Color::Cyan),
                Cell::new("TX Bytes").fg(Color::Cyan),
                Cell::new("RX Packets").fg(Color::Cyan),
                Cell::new("TX Packets").fg(Color::Cyan),
                Cell::new("RX Errors").fg(Color::Cyan),
                Cell::new("TX Errors").fg(Color::Cyan),
            ]);
        } else {
            table.set_header(vec![
                Cell::new("Interface").fg(Color::Cyan),
                Cell::new("State").fg(Color::Cyan),
                Cell::new("RX").fg(Color::Cyan),
                Cell::new("TX").fg(Color::Cyan),
                Cell::new("Errors").fg(Color::Cyan),
            ]);
        }

        for link in links {
            let stats = self.read_interface_stats(&link.name)?;

            if self.detailed {
                let state_cell = match link.state {
                    netctl_types::network::LinkState::Up => Cell::new("UP").fg(Color::Green),
                    netctl_types::network::LinkState::Down => Cell::new("DOWN").fg(Color::Red),
                };

                table.add_row(vec![
                    Cell::new(&link.name),
                    state_cell,
                    Cell::new(self.format_bytes(stats.rx_bytes)),
                    Cell::new(self.format_bytes(stats.tx_bytes)),
                    Cell::new(stats.rx_packets.to_string()),
                    Cell::new(stats.tx_packets.to_string()),
                    Cell::new(stats.rx_errors.to_string()).fg(if stats.rx_errors > 0 {
                        Color::Red
                    } else {
                        Color::Green
                    }),
                    Cell::new(stats.tx_errors.to_string()).fg(if stats.tx_errors > 0 {
                        Color::Red
                    } else {
                        Color::Green
                    }),
                ]);
            } else {
                let state_cell = match link.state {
                    netctl_types::network::LinkState::Up => Cell::new("UP").fg(Color::Green),
                    netctl_types::network::LinkState::Down => Cell::new("DOWN").fg(Color::Red),
                };

                let total_errors = stats.rx_errors + stats.tx_errors;
                table.add_row(vec![
                    Cell::new(&link.name),
                    state_cell,
                    Cell::new(self.format_bytes(stats.rx_bytes)),
                    Cell::new(self.format_bytes(stats.tx_bytes)),
                    Cell::new(total_errors.to_string()).fg(if total_errors > 0 {
                        Color::Red
                    } else {
                        Color::Green
                    }),
                ]);
            }
        }

        println!();
        println!("{}", table);
        println!();

        Ok(())
    }

    fn print_json(&self, links: &[netctl_types::network::LinkInfo]) -> Result<()> {
        use serde_json::json;

        let stats: Vec<_> = links
            .iter()
            .map(|link| {
                let s = self.read_interface_stats(&link.name).unwrap_or_default();
                json!({
                    "interface": link.name,
                    "state": format!("{:?}", link.state),
                    "rx_bytes": s.rx_bytes,
                    "tx_bytes": s.tx_bytes,
                    "rx_packets": s.rx_packets,
                    "tx_packets": s.tx_packets,
                    "rx_errors": s.rx_errors,
                    "tx_errors": s.tx_errors,
                })
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&stats).unwrap());
        Ok(())
    }

    fn read_interface_stats(&self, interface: &str) -> Result<InterfaceStats> {
        let base_path = format!("/sys/class/net/{}/statistics", interface);

        Ok(InterfaceStats {
            rx_bytes: self.read_stat(&base_path, "rx_bytes")?,
            tx_bytes: self.read_stat(&base_path, "tx_bytes")?,
            rx_packets: self.read_stat(&base_path, "rx_packets")?,
            tx_packets: self.read_stat(&base_path, "tx_packets")?,
            rx_errors: self.read_stat(&base_path, "rx_errors")?,
            tx_errors: self.read_stat(&base_path, "tx_errors")?,
        })
    }

    fn read_stat(&self, base_path: &str, stat_name: &str) -> Result<u64> {
        let path = Path::new(base_path).join(stat_name);
        let content = fs::read_to_string(&path).unwrap_or_else(|_| "0".to_string());
        Ok(content.trim().parse().unwrap_or(0))
    }

    fn format_bytes(&self, bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if bytes >= TB {
            format!("{:.2} TB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
}

#[derive(Default)]
struct InterfaceStats {
    rx_bytes: u64,
    tx_bytes: u64,
    rx_packets: u64,
    tx_packets: u64,
    rx_errors: u64,
    tx_errors: u64,
}
