use clap::{Args, Subcommand};
use miette::Result;
use netctl_core::NetworkManager;

#[derive(Args)]
pub struct LinkCommand {
    #[command(subcommand)]
    command: LinkSubcommand,
}

#[derive(Subcommand)]
pub enum LinkSubcommand {
    /// Set link properties
    Set(SetArgs),
}

#[derive(Args)]
pub struct SetArgs {
    /// Interface name
    interface: String,

    /// Property to set (state, mtu, mac)
    property: String,

    /// Value to set
    value: String,
}

impl LinkCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            LinkSubcommand::Set(args) => args.execute().await,
        }
    }
}

impl SetArgs {
    pub async fn execute(self) -> Result<()> {
        let mgr = NetworkManager::new().await?;

        match self.property.as_str() {
            "state" => {
                match self.value.as_str() {
                    "up" => {
                        mgr.set_link_up(&self.interface).await?;
                        println!("✓ Interface {} is now up", self.interface);
                    }
                    "down" => {
                        mgr.set_link_down(&self.interface).await?;
                        println!("✓ Interface {} is now down", self.interface);
                    }
                    _ => {
                        return Err(miette::miette!(
                            "Invalid state '{}'. Use 'up' or 'down'",
                            self.value
                        ))
                    }
                }
            }
            "mtu" => {
                let mtu: u32 = self.value.parse().map_err(|_| {
                    miette::miette!("Invalid MTU '{}'. Must be a number", self.value)
                })?;
                mgr.set_mtu(&self.interface, mtu).await?;
                println!("✓ MTU set to {} for {}", mtu, self.interface);
            }
            "mac" => {
                // TODO: Implement MAC address setting
                // For now, return an error indicating it's not implemented
                return Err(miette::miette!(
                    "MAC address setting not yet implemented. Property: {}, Value: {}",
                    self.property,
                    self.value
                ));
            }
            _ => {
                return Err(miette::miette!(
                    "Unknown property '{}'. Valid properties: state, mtu, mac",
                    self.property
                ))
            }
        }

        Ok(())
    }
}
