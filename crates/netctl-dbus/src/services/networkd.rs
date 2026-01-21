use async_trait::async_trait;
use netctl_types::{Error, Result};
use std::sync::Arc;
use tracing::{debug, info, instrument};
use zbus::{proxy, Connection};

/// D-Bus proxy for systemd-networkd Manager interface
#[proxy(
    interface = "org.freedesktop.network1.Manager",
    default_service = "org.freedesktop.network1",
    default_path = "/org/freedesktop/network1"
)]
trait Manager {
    /// Reload networkd configuration files
    fn reload(&self) -> zbus::Result<()>;

    /// Reconfigure a specific link by interface index
    fn reconfigure_link(&self, ifindex: i32) -> zbus::Result<()>;

    /// Get link object path by interface index
    fn get_link(&self, ifindex: i32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

#[async_trait]
pub trait NetworkdOps {
    async fn reload(&self) -> Result<()>;
    async fn reconfigure_link(&self, index: u32) -> Result<()>;
    async fn get_link_path(&self, index: u32) -> Result<String>;
}

#[derive(Clone)]
pub struct NetworkdService {
    connection: Arc<Connection>,
}

impl NetworkdService {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }

    async fn create_proxy(&self) -> Result<ManagerProxy<'_>> {
        ManagerProxy::new(&self.connection)
            .await
            .map_err(|e| Error::dbus(format!("Failed to create networkd proxy: {}", e)))
    }
}

#[async_trait]
impl NetworkdOps for NetworkdService {
    #[instrument(skip(self))]
    async fn reload(&self) -> Result<()> {
        info!("reloading systemd-networkd configuration");

        let proxy = self.create_proxy().await?;
        proxy
            .reload()
            .await
            .map_err(|e| Error::dbus(format!("Failed to reload networkd: {}", e)))?;

        debug!("systemd-networkd reloaded successfully");
        Ok(())
    }

    #[instrument(skip(self), fields(ifindex = %index))]
    async fn reconfigure_link(&self, index: u32) -> Result<()> {
        info!(ifindex = %index, "reconfiguring link via networkd");

        let proxy = self.create_proxy().await?;
        proxy
            .reconfigure_link(index as i32)
            .await
            .map_err(|e| Error::dbus(format!("Failed to reconfigure link {}: {}", index, e)))?;

        debug!(ifindex = %index, "link reconfigured successfully");
        Ok(())
    }

    #[instrument(skip(self), fields(ifindex = %index))]
    async fn get_link_path(&self, index: u32) -> Result<String> {
        debug!(ifindex = %index, "getting link D-Bus path");

        let proxy = self.create_proxy().await?;
        let path = proxy
            .get_link(index as i32)
            .await
            .map_err(|e| Error::dbus(format!("Failed to get link path for {}: {}", index, e)))?;

        Ok(path.to_string())
    }
}
