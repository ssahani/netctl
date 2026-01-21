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

    /// Set state (up/down)
    #[arg(long)]
    state: Option<String>,

    /// Set MTU
    #[arg(long)]
    mtu: Option<u32>,
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

        if let Some(state) = self.state.as_deref() {
            match state {
                "up" => {
                    mgr.set_link_up(&self.interface).await?;
                    println!("✓ Interface {} is now up", self.interface);
                }
                "down" => {
                    mgr.set_link_down(&self.interface).await?;
                    println!("✓ Interface {} is now down", self.interface);
                }
                _ => return Err(miette::miette!("Invalid state: {}", state)),
            }
        }

        if let Some(mtu) = self.mtu {
            mgr.set_mtu(&self.interface, mtu).await?;
            println!("✓ MTU set to {} for {}", mtu, self.interface);
        }

        Ok(())
    }
}
