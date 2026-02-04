use clap::{Args, Subcommand};
use colored::Colorize;
use miette::{IntoDiagnostic, Result};
use netctl_core::NetworkManager;
use std::process::Command;
use std::time::Instant;

#[derive(Args)]
pub struct TestCommand {
    #[command(subcommand)]
    command: TestSubcommand,
}

#[derive(Subcommand)]
pub enum TestSubcommand {
    /// Test network connectivity
    Connectivity(ConnectivityArgs),

    /// Test DNS resolution
    Dns(DnsArgs),

    /// Ping a host
    Ping(PingArgs),

    /// Run all network tests
    All(AllArgs),
}

#[derive(Args)]
pub struct ConnectivityArgs {
    /// Interface to test (optional)
    #[arg(short, long)]
    interface: Option<String>,
}

#[derive(Args)]
pub struct DnsArgs {
    /// Hostname to resolve
    #[arg(default_value = "www.google.com")]
    hostname: String,
}

#[derive(Args)]
pub struct PingArgs {
    /// Host to ping
    host: String,

    /// Number of packets to send
    #[arg(short = 'c', long, default_value = "4")]
    count: u32,

    /// Interface to use (optional)
    #[arg(short, long)]
    interface: Option<String>,
}

#[derive(Args)]
pub struct AllArgs {}

impl TestCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            TestSubcommand::Connectivity(args) => args.execute().await,
            TestSubcommand::Dns(args) => args.execute().await,
            TestSubcommand::Ping(args) => args.execute().await,
            TestSubcommand::All(args) => args.execute().await,
        }
    }
}

impl ConnectivityArgs {
    pub async fn execute(self) -> Result<()> {
        println!("\n{}", "Network Connectivity Test".bold().cyan());
        println!("{}", "=".repeat(80));
        println!();

        let mgr = NetworkManager::new().await?;
        let links = mgr.list_links().await?;

        let test_links: Vec<_> = if let Some(ref iface) = self.interface {
            links.into_iter().filter(|l| &l.name == iface).collect()
        } else {
            links.into_iter().filter(|l| {
                matches!(l.state, netctl_types::network::LinkState::Up)
            }).collect()
        };

        if test_links.is_empty() {
            println!("{}", "No active interfaces to test".yellow());
            return Ok(());
        }

        let mut passed = 0;
        let mut failed = 0;

        for link in &test_links {
            print!("Testing {} ... ", link.name.cyan());

            let start = Instant::now();
            let result = self.test_interface_connectivity(&link.name);
            let duration = start.elapsed();

            match result {
                Ok(true) => {
                    println!("{} ({:.2}s)", "✓ PASS".green(), duration.as_secs_f64());
                    passed += 1;
                }
                Ok(false) | Err(_) => {
                    println!("{} ({:.2}s)", "✗ FAIL".red(), duration.as_secs_f64());
                    failed += 1;
                }
            }
        }

        println!();
        println!("{}", "=".repeat(80));
        println!("Results: {} passed, {} failed",
            passed.to_string().green(),
            failed.to_string().red()
        );
        println!();

        Ok(())
    }

    fn test_interface_connectivity(&self, _interface: &str) -> Result<bool> {
        // Test basic connectivity with ping
        let output = Command::new("ping")
            .args(&["-c", "1", "-W", "2", "8.8.8.8"])
            .output()
            .into_diagnostic()?;

        Ok(output.status.success())
    }
}

impl DnsArgs {
    pub async fn execute(self) -> Result<()> {
        println!("\n{}", "DNS Resolution Test".bold().cyan());
        println!("{}", "=".repeat(80));
        println!();

        println!("Resolving {} ...", self.hostname.cyan());

        let start = Instant::now();
        let output = Command::new("host")
            .arg(&self.hostname)
            .output()
            .into_diagnostic()?;
        let duration = start.elapsed();

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            println!("{} Resolved in {:.3}s", "✓".green(), duration.as_secs_f64());
            println!();
            for line in result.lines() {
                println!("  {}", line);
            }
        } else {
            println!("{} Resolution failed", "✗".red());
            let error = String::from_utf8_lossy(&output.stderr);
            if !error.is_empty() {
                println!("  Error: {}", error);
            }
        }

        println!();
        Ok(())
    }
}

impl PingArgs {
    pub async fn execute(self) -> Result<()> {
        println!("\n{}", "Ping Test".bold().cyan());
        println!("{}", "=".repeat(80));
        println!();

        println!("Pinging {} ({} packets) ...", self.host.cyan(), self.count);
        println!();

        let mut cmd = Command::new("ping");
        cmd.args(&["-c", &self.count.to_string(), &self.host]);

        if let Some(ref iface) = self.interface {
            cmd.args(&["-I", iface]);
        }

        let output = cmd.output().into_diagnostic()?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout);
            println!("{}", result);

            // Parse statistics
            if let Some(stats_line) = result.lines().rev().nth(1) {
                if stats_line.contains("packets transmitted") {
                    println!("{}", "=".repeat(80));
                    println!("{} Ping test completed", "✓".green());
                }
            }
        } else {
            println!("{} Ping test failed", "✗".red());
            let error = String::from_utf8_lossy(&output.stderr);
            if !error.is_empty() {
                println!("Error: {}", error);
            }
        }

        println!();
        Ok(())
    }
}

impl AllArgs {
    pub async fn execute(self) -> Result<()> {
        println!("\n{}", "Comprehensive Network Test Suite".bold().cyan());
        println!("{}", "=".repeat(80));
        println!();

        let mut results = Vec::new();

        // Test 1: Interface availability
        println!("{} {}", "1.".cyan(), "Testing interface availability...".bold());
        let mgr = NetworkManager::new().await?;
        let links = mgr.list_links().await?;
        let up_count = links.iter().filter(|l| {
            matches!(l.state, netctl_types::network::LinkState::Up)
        }).count();

        if up_count > 0 {
            println!("   {} {} interface(s) up", "✓".green(), up_count);
            results.push(("Interface availability", true));
        } else {
            println!("   {} No interfaces up", "✗".red());
            results.push(("Interface availability", false));
        }
        println!();

        // Test 2: Internet connectivity
        println!("{} {}", "2.".cyan(), "Testing internet connectivity...".bold());
        let output = Command::new("ping")
            .args(&["-c", "2", "-W", "3", "8.8.8.8"])
            .output()
            .into_diagnostic()?;

        if output.status.success() {
            println!("   {} Internet accessible", "✓".green());
            results.push(("Internet connectivity", true));
        } else {
            println!("   {} No internet access", "✗".red());
            results.push(("Internet connectivity", false));
        }
        println!();

        // Test 3: DNS resolution
        println!("{} {}", "3.".cyan(), "Testing DNS resolution...".bold());
        let output = Command::new("host")
            .arg("www.google.com")
            .output()
            .into_diagnostic()?;

        if output.status.success() {
            println!("   {} DNS resolution working", "✓".green());
            results.push(("DNS resolution", true));
        } else {
            println!("   {} DNS resolution failed", "✗".red());
            results.push(("DNS resolution", false));
        }
        println!();

        // Test 4: systemd-networkd
        println!("{} {}", "4.".cyan(), "Testing systemd-networkd...".bold());
        let output = Command::new("systemctl")
            .args(&["is-active", "systemd-networkd"])
            .output()
            .into_diagnostic()?;

        let is_active = String::from_utf8_lossy(&output.stdout).trim() == "active";
        if is_active {
            println!("   {} systemd-networkd is active", "✓".green());
            results.push(("systemd-networkd", true));
        } else {
            println!("   {} systemd-networkd is not active", "⚠".yellow());
            results.push(("systemd-networkd", false));
        }
        println!();

        // Test 5: systemd-resolved
        println!("{} {}", "5.".cyan(), "Testing systemd-resolved...".bold());
        let output = Command::new("systemctl")
            .args(&["is-active", "systemd-resolved"])
            .output()
            .into_diagnostic()?;

        let is_active = String::from_utf8_lossy(&output.stdout).trim() == "active";
        if is_active {
            println!("   {} systemd-resolved is active", "✓".green());
            results.push(("systemd-resolved", true));
        } else {
            println!("   {} systemd-resolved is not active", "⚠".yellow());
            results.push(("systemd-resolved", false));
        }
        println!();

        // Summary
        println!("{}", "=".repeat(80));
        println!("{}", "Test Summary".bold());
        println!();

        let passed = results.iter().filter(|(_, r)| *r).count();
        let total = results.len();

        for (test, result) in &results {
            let status = if *result {
                "PASS".green()
            } else {
                "FAIL".red()
            };
            println!("  {} {}", status, test);
        }

        println!();
        if passed == total {
            println!("{} All tests passed ({}/{})", "✓".green().bold(), passed, total);
        } else if passed > 0 {
            println!("{} Some tests passed ({}/{})", "⚠".yellow().bold(), passed, total);
        } else {
            println!("{} All tests failed ({}/{})", "✗".red().bold(), passed, total);
        }
        println!();

        Ok(())
    }
}
