use clap::{Args, Subcommand};
use miette::Result;
use netctl_core::NetworkManager;
use netctl_types::IpNetwork;

#[derive(Args)]
pub struct AddressCommand {
    #[command(subcommand)]
    command: AddressSubcommand,
}

#[derive(Subcommand)]
pub enum AddressSubcommand {
    /// Add IP address
    Add(AddArgs),
}

#[derive(Args)]
pub struct AddArgs {
    /// Interface name
    interface: String,

    /// IP address with prefix (e.g., 192.168.1.10/24)
    address: String,
}

impl AddressCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            AddressSubcommand::Add(args) => args.execute().await,
        }
    }
}

impl AddArgs {
    pub async fn execute(self) -> Result<()> {
        let network: IpNetwork = self
            .address
            .parse()
            .map_err(|e: netctl_types::Error| miette::miette!("{}", e))?;

        let mgr = NetworkManager::new().await?;
        mgr.add_address(&self.interface, network).await?;

        println!("✓ Address {} added to {}", network, self.interface);
        Ok(())
    }
}
