use crate::NetlinkHandle;
use async_trait::async_trait;
use netctl_types::{Error, IpNetwork, Result};
use tracing::{info, instrument};

#[async_trait]
pub trait AddressOps {
    async fn add_address(&self, index: u32, network: IpNetwork) -> Result<()>;
    async fn delete_address(&self, index: u32, network: IpNetwork) -> Result<()>;
}

#[async_trait]
impl AddressOps for NetlinkHandle {
    #[instrument(skip(self))]
    async fn add_address(&self, index: u32, network: IpNetwork) -> Result<()> {
        info!(%network, "adding address");
        self.handle()
            .address()
            .add(index, network.addr, network.prefix_len)
            .execute()
            .await
            .map_err(|e| Error::netlink(format!("{}", e)))
    }

    #[instrument(skip(self))]
    async fn delete_address(&self, _index: u32, network: IpNetwork) -> Result<()> {
        info!(%network, "deleting address");
        // TODO: Implement address deletion with rtnetlink 0.14 API
        // The API changed to use AddressMessage instead of individual parameters
        Err(Error::Generic(
            "delete_address not yet implemented".to_string(),
        ))
    }
}
