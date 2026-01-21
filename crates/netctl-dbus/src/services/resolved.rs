use async_trait::async_trait;
use netctl_types::{Error, Result};
use std::net::IpAddr;
use std::sync::Arc;
use tracing::{debug, info, instrument};
use zbus::{proxy, Connection};

/// D-Bus proxy for systemd-resolved Manager interface
#[proxy(
    interface = "org.freedesktop.resolve1.Manager",
    default_service = "org.freedesktop.resolve1",
    default_path = "/org/freedesktop/resolve1"
)]
trait ResolveManager {
    /// Set DNS servers for a specific link
    fn set_link_dns(&self, ifindex: i32, addresses: Vec<(i32, Vec<u8>)>) -> zbus::Result<()>;

    /// Set DNS search domains for a specific link
    fn set_link_domains(&self, ifindex: i32, domains: Vec<(String, bool)>) -> zbus::Result<()>;

    /// Revert DNS settings for a link to defaults
    fn revert_link(&self, ifindex: i32) -> zbus::Result<()>;

    /// Flush all caches
    fn flush_caches(&self) -> zbus::Result<()>;
}

#[async_trait]
pub trait ResolvedOps {
    async fn set_link_dns(&self, index: u32, servers: Vec<IpAddr>) -> Result<()>;
    async fn set_link_domains(&self, index: u32, domains: Vec<String>) -> Result<()>;
    async fn revert_link(&self, index: u32) -> Result<()>;
    async fn flush_caches(&self) -> Result<()>;
}

#[derive(Clone)]
pub struct ResolvedService {
    connection: Arc<Connection>,
}

impl ResolvedService {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }

    async fn create_proxy(&self) -> Result<ResolveManagerProxy<'_>> {
        ResolveManagerProxy::new(&self.connection)
            .await
            .map_err(|e| Error::dbus(format!("Failed to create resolved proxy: {}", e)))
    }

    fn ip_to_dbus_format(addr: &IpAddr) -> (i32, Vec<u8>) {
        match addr {
            IpAddr::V4(ipv4) => (2, ipv4.octets().to_vec()), // AF_INET = 2
            IpAddr::V6(ipv6) => (10, ipv6.octets().to_vec()), // AF_INET6 = 10
        }
    }
}

#[async_trait]
impl ResolvedOps for ResolvedService {
    #[instrument(skip(self, servers), fields(ifindex = %index, server_count = servers.len()))]
    async fn set_link_dns(&self, index: u32, servers: Vec<IpAddr>) -> Result<()> {
        info!(
            ifindex = %index,
            server_count = servers.len(),
            "setting DNS servers for link"
        );

        let addresses: Vec<_> = servers.iter().map(Self::ip_to_dbus_format).collect();

        let proxy = self.create_proxy().await?;
        proxy
            .set_link_dns(index as i32, addresses)
            .await
            .map_err(|e| Error::dbus(format!("Failed to set DNS servers: {}", e)))?;

        debug!(ifindex = %index, "DNS servers configured successfully");
        Ok(())
    }

    #[instrument(skip(self, domains), fields(ifindex = %index, domain_count = domains.len()))]
    async fn set_link_domains(&self, index: u32, domains: Vec<String>) -> Result<()> {
        info!(
            ifindex = %index,
            domain_count = domains.len(),
            "setting DNS domains for link"
        );

        // Convert to (domain, routing_only) tuples
        // routing_only=false means the domain is used for both search and routing
        let domain_tuples: Vec<_> = domains.into_iter().map(|d| (d, false)).collect();

        let proxy = self.create_proxy().await?;
        proxy
            .set_link_domains(index as i32, domain_tuples)
            .await
            .map_err(|e| Error::dbus(format!("Failed to set DNS domains: {}", e)))?;

        debug!(ifindex = %index, "DNS domains configured successfully");
        Ok(())
    }

    #[instrument(skip(self), fields(ifindex = %index))]
    async fn revert_link(&self, index: u32) -> Result<()> {
        info!(ifindex = %index, "reverting DNS configuration for link");

        let proxy = self.create_proxy().await?;
        proxy
            .revert_link(index as i32)
            .await
            .map_err(|e| Error::dbus(format!("Failed to revert DNS config: {}", e)))?;

        debug!(ifindex = %index, "DNS configuration reverted successfully");
        Ok(())
    }

    #[instrument(skip(self))]
    async fn flush_caches(&self) -> Result<()> {
        info!("flushing DNS caches");

        let proxy = self.create_proxy().await?;
        proxy
            .flush_caches()
            .await
            .map_err(|e| Error::dbus(format!("Failed to flush DNS caches: {}", e)))?;

        debug!("DNS caches flushed successfully");
        Ok(())
    }
}
