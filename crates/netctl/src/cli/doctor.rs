use clap::Args;
use colored::Colorize;
use miette::Result;
use netctl_core::NetworkManager;
use std::process::Command;

#[derive(Args)]
pub struct DoctorArgs {
    /// Run detailed diagnostics
    #[arg(long, short)]
    verbose: bool,
}

impl DoctorArgs {
    pub async fn execute(self) -> Result<()> {
        println!("{}", "🔍 netctl System Diagnostics".bold().cyan());
        println!("{}", "=".repeat(60));
        println!();

        let mut all_ok = true;

        // Check 1: Network interfaces
        all_ok &= self.check_interfaces().await;

        // Check 2: systemd services
        all_ok &= self.check_systemd_services();

        // Check 3: D-Bus availability
        all_ok &= self.check_dbus();

        // Check 4: Permissions
        all_ok &= self.check_permissions();

        // Check 5: Network connectivity
        all_ok &= self.check_connectivity();

        // Check 6: DNS resolution
        all_ok &= self.check_dns();

        println!();
        println!("{}", "=".repeat(60));
        if all_ok {
            println!("{}", "✓ All checks passed!".green().bold());
        } else {
            println!("{}", "⚠ Some issues detected. See above for details.".yellow().bold());
        }

        Ok(())
    }

    async fn check_interfaces(&self) -> bool {
        print!("{} Checking network interfaces... ", "→".blue());

        match NetworkManager::new().await {
            Ok(mgr) => match mgr.list_links().await {
                Ok(links) => {
                    let up_count = links.iter().filter(|l| {
                        matches!(l.state, netctl_types::network::LinkState::Up)
                    }).count();

                    println!("{}", "✓".green());
                    if self.verbose {
                        println!("    Found {} interface(s), {} up", links.len(), up_count);
                        for link in &links {
                            println!("    - {}: {:?}", link.name, link.state);
                        }
                    }
                    true
                }
                Err(e) => {
                    println!("{}", "✗".red());
                    println!("    Error: {}", e);
                    false
                }
            },
            Err(e) => {
                println!("{}", "✗".red());
                println!("    Error: {}", e);
                false
            }
        }
    }

    fn check_systemd_services(&self) -> bool {
        print!("{} Checking systemd services... ", "→".blue());

        let services = vec!["systemd-networkd", "systemd-resolved"];
        let mut all_running = true;

        for service in &services {
            let output = Command::new("systemctl")
                .args(&["is-active", service])
                .output();

            match output {
                Ok(out) => {
                    let status = String::from_utf8_lossy(&out.stdout);
                    let is_active = status.trim() == "active";

                    if !is_active {
                        all_running = false;
                        if self.verbose {
                            println!();
                            println!("    {} is not running", service);
                        }
                    }
                }
                Err(_) => {
                    all_running = false;
                }
            }
        }

        if all_running {
            println!("{}", "✓".green());
            if self.verbose {
                for service in services {
                    println!("    {} is active", service);
                }
            }
        } else {
            println!("{}", "⚠".yellow());
            println!("    Some systemd services are not running");
            println!("    This may limit functionality");
        }

        true // Don't fail on this, just warn
    }

    fn check_dbus(&self) -> bool {
        print!("{} Checking D-Bus connection... ", "→".blue());

        let output = Command::new("busctl")
            .args(&["list"])
            .output();

        match output {
            Ok(_) => {
                println!("{}", "✓".green());
                if self.verbose {
                    println!("    D-Bus system bus is accessible");
                }
                true
            }
            Err(e) => {
                println!("{}", "✗".red());
                println!("    Error: {}", e);
                false
            }
        }
    }

    fn check_permissions(&self) -> bool {
        print!("{} Checking permissions... ", "→".blue());

        // Check if running as root or with capabilities
        let is_root = Command::new("id")
            .args(&["-u"])
            .output()
            .map(|out| String::from_utf8_lossy(&out.stdout).trim() == "0")
            .unwrap_or(false);

        if is_root {
            println!("{}", "✓".green());
            if self.verbose {
                println!("    Running as root");
            }
            true
        } else {
            println!("{}", "⚠".yellow());
            println!("    Not running as root");
            println!("    Some operations may require sudo/root privileges");
            true // Don't fail, just warn
        }
    }

    fn check_connectivity(&self) -> bool {
        print!("{} Checking network connectivity... ", "→".blue());

        // Ping a reliable host
        let output = Command::new("ping")
            .args(&["-c", "1", "-W", "2", "8.8.8.8"])
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    println!("{}", "✓".green());
                    if self.verbose {
                        println!("    Internet connectivity OK");
                    }
                    true
                } else {
                    println!("{}", "⚠".yellow());
                    println!("    No internet connectivity");
                    println!("    This is normal if you're configuring an isolated network");
                    true // Don't fail
                }
            }
            Err(_) => {
                println!("{}", "⚠".yellow());
                println!("    Could not test connectivity (ping not available)");
                true // Don't fail
            }
        }
    }

    fn check_dns(&self) -> bool {
        print!("{} Checking DNS resolution... ", "→".blue());

        let output = Command::new("host")
            .args(&["www.google.com"])
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    println!("{}", "✓".green());
                    if self.verbose {
                        println!("    DNS resolution working");
                    }
                    true
                } else {
                    println!("{}", "⚠".yellow());
                    println!("    DNS resolution failed");
                    println!("    Check /etc/resolv.conf or systemd-resolved configuration");
                    true // Don't fail
                }
            }
            Err(_) => {
                println!("{}", "⚠".yellow());
                println!("    Could not test DNS (host command not available)");
                true // Don't fail
            }
        }
    }
}
