use crate::NetlinkHandle;
use async_trait::async_trait;
use netctl_types::{Error, LinkInfo, LinkState, MacAddress, Result};
use tracing::{debug, info, instrument};

#[async_trait]
pub trait LinkOps {
    async fn get_link_by_name(&self, name: &str) -> Result<u32>;
    async fn list_links(&self) -> Result<Vec<LinkInfo>>;
    async fn get_link_info(&self, name: &str) -> Result<LinkInfo>;
    async fn set_link_up(&self, index: u32) -> Result<()>;
    async fn set_link_down(&self, index: u32) -> Result<()>;
    async fn set_link_mtu(&self, index: u32, mtu: u32) -> Result<()>;
}

#[async_trait]
impl LinkOps for NetlinkHandle {
    #[instrument(skip(self))]
    async fn get_link_by_name(&self, name: &str) -> Result<u32> {
        info!("getting link by name");
        use futures::TryStreamExt;

        let mut links = self
            .handle()
            .link()
            .get()
            .match_name(name.to_string())
            .execute();

        if let Some(link) = links
            .try_next()
            .await
            .map_err(|e| Error::netlink(format!("{}", e)))?
        {
            Ok(link.header.index)
        } else {
            Err(Error::InterfaceNotFound {
                name: name.to_string(),
            })
        }
    }

    #[instrument(skip(self))]
    async fn list_links(&self) -> Result<Vec<LinkInfo>> {
        info!("listing all links");
        use futures::TryStreamExt;
        use netlink_packet_route::link::LinkAttribute;
        use netlink_packet_route::link::LinkFlag;

        let mut links = self.handle().link().get().execute();
        let mut result = Vec::new();

        while let Some(link) = links
            .try_next()
            .await
            .map_err(|e| Error::netlink(format!("{}", e)))?
        {
            let index = link.header.index;
            let mut name = String::new();
            let mut mtu = 0;
            let mut mac = None;

            // Parse link attributes
            for attr in link.attributes.iter() {
                match attr {
                    LinkAttribute::IfName(n) => name = n.clone(),
                    LinkAttribute::Mtu(m) => mtu = *m,
                    LinkAttribute::Address(addr) if addr.len() == 6 => {
                        let mut octets = [0u8; 6];
                        octets.copy_from_slice(&addr[0..6]);
                        mac = Some(MacAddress::new(octets));
                    }
                    _ => {}
                }
            }

            // Check if UP flag is set
            let state = if link.header.flags.contains(&LinkFlag::Up) {
                LinkState::Up
            } else {
                LinkState::Down
            };

            debug!(index, name, mtu, "found link");

            result.push(LinkInfo {
                index,
                name,
                state,
                mtu,
                mac_address: mac,
                addresses: Vec::new(), // Will be populated if needed
            });
        }

        Ok(result)
    }

    #[instrument(skip(self), fields(name = %name))]
    async fn get_link_info(&self, name: &str) -> Result<LinkInfo> {
        info!(name = %name, "getting link info");
        use futures::TryStreamExt;
        use netlink_packet_route::link::LinkAttribute;
        use netlink_packet_route::link::LinkFlag;

        let mut links = self
            .handle()
            .link()
            .get()
            .match_name(name.to_string())
            .execute();

        if let Some(link) = links
            .try_next()
            .await
            .map_err(|e| Error::netlink(format!("{}", e)))?
        {
            let index = link.header.index;
            let mut link_name = String::new();
            let mut mtu = 0;
            let mut mac = None;

            for attr in link.attributes.iter() {
                match attr {
                    LinkAttribute::IfName(n) => link_name = n.clone(),
                    LinkAttribute::Mtu(m) => mtu = *m,
                    LinkAttribute::Address(addr) if addr.len() == 6 => {
                        let mut octets = [0u8; 6];
                        octets.copy_from_slice(&addr[0..6]);
                        mac = Some(MacAddress::new(octets));
                    }
                    _ => {}
                }
            }

            // Check if UP flag is set
            let state = if link.header.flags.contains(&LinkFlag::Up) {
                LinkState::Up
            } else {
                LinkState::Down
            };

            Ok(LinkInfo {
                index,
                name: link_name,
                state,
                mtu,
                mac_address: mac,
                addresses: Vec::new(), // TODO: Query addresses
            })
        } else {
            Err(Error::InterfaceNotFound {
                name: name.to_string(),
            })
        }
    }

    #[instrument(skip(self))]
    async fn set_link_up(&self, index: u32) -> Result<()> {
        info!("setting link up");
        self.handle()
            .link()
            .set(index)
            .up()
            .execute()
            .await
            .map_err(|e| Error::netlink(format!("{}", e)))
    }

    #[instrument(skip(self))]
    async fn set_link_down(&self, index: u32) -> Result<()> {
        info!("setting link down");
        self.handle()
            .link()
            .set(index)
            .down()
            .execute()
            .await
            .map_err(|e| Error::netlink(format!("{}", e)))
    }

    #[instrument(skip(self))]
    async fn set_link_mtu(&self, index: u32, mtu: u32) -> Result<()> {
        info!(mtu, "setting MTU");
        self.handle()
            .link()
            .set(index)
            .mtu(mtu)
            .execute()
            .await
            .map_err(|e| Error::netlink(format!("{}", e)))
    }
}
