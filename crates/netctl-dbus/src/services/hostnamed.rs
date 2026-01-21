use async_trait::async_trait;
use netctl_types::{Error, Result};
use std::sync::Arc;
use tracing::{debug, info, instrument};
use zbus::{proxy, Connection};

/// D-Bus proxy for systemd-hostnamed interface
#[proxy(
    interface = "org.freedesktop.hostname1",
    default_service = "org.freedesktop.hostname1",
    default_path = "/org/freedesktop/hostname1"
)]
trait Hostname {
    /// Set the static (configured) hostname
    fn set_static_hostname(&self, hostname: &str, interactive: bool) -> zbus::Result<()>;

    /// Set the pretty (human-readable) hostname
    fn set_pretty_hostname(&self, hostname: &str, interactive: bool) -> zbus::Result<()>;

    /// Get the static hostname
    #[zbus(property)]
    fn static_hostname(&self) -> zbus::Result<String>;

    /// Get the pretty hostname
    #[zbus(property)]
    fn pretty_hostname(&self) -> zbus::Result<String>;

    /// Get the transient (kernel) hostname
    #[zbus(property)]
    fn hostname(&self) -> zbus::Result<String>;

    /// Get the machine ID
    #[zbus(property)]
    fn machine_id(&self) -> zbus::Result<String>;
}

#[async_trait]
pub trait HostnamedOps {
    async fn set_static_hostname(&self, hostname: &str) -> Result<()>;
    async fn set_pretty_hostname(&self, hostname: &str) -> Result<()>;
    async fn get_static_hostname(&self) -> Result<String>;
    async fn get_pretty_hostname(&self) -> Result<String>;
    async fn get_hostname(&self) -> Result<String>;
    async fn get_machine_id(&self) -> Result<String>;
}

#[derive(Clone)]
pub struct HostnamedService {
    connection: Arc<Connection>,
}

impl HostnamedService {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }

    async fn create_proxy(&self) -> Result<HostnameProxy<'_>> {
        HostnameProxy::new(&self.connection)
            .await
            .map_err(|e| Error::dbus(format!("Failed to create hostnamed proxy: {}", e)))
    }
}

#[async_trait]
impl HostnamedOps for HostnamedService {
    #[instrument(skip(self), fields(hostname = %hostname))]
    async fn set_static_hostname(&self, hostname: &str) -> Result<()> {
        info!(hostname = %hostname, "setting static hostname");

        let proxy = self.create_proxy().await?;
        proxy
            .set_static_hostname(hostname, false)
            .await
            .map_err(|e| Error::dbus(format!("Failed to set static hostname: {}", e)))?;

        debug!(hostname = %hostname, "static hostname set successfully");
        Ok(())
    }

    #[instrument(skip(self), fields(hostname = %hostname))]
    async fn set_pretty_hostname(&self, hostname: &str) -> Result<()> {
        info!(hostname = %hostname, "setting pretty hostname");

        let proxy = self.create_proxy().await?;
        proxy
            .set_pretty_hostname(hostname, false)
            .await
            .map_err(|e| Error::dbus(format!("Failed to set pretty hostname: {}", e)))?;

        debug!(hostname = %hostname, "pretty hostname set successfully");
        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_static_hostname(&self) -> Result<String> {
        debug!("getting static hostname");

        let proxy = self.create_proxy().await?;
        let hostname = proxy
            .static_hostname()
            .await
            .map_err(|e| Error::dbus(format!("Failed to get static hostname: {}", e)))?;

        Ok(hostname)
    }

    #[instrument(skip(self))]
    async fn get_pretty_hostname(&self) -> Result<String> {
        debug!("getting pretty hostname");

        let proxy = self.create_proxy().await?;
        let hostname = proxy
            .pretty_hostname()
            .await
            .map_err(|e| Error::dbus(format!("Failed to get pretty hostname: {}", e)))?;

        Ok(hostname)
    }

    #[instrument(skip(self))]
    async fn get_hostname(&self) -> Result<String> {
        debug!("getting transient hostname");

        let proxy = self.create_proxy().await?;
        let hostname = proxy
            .hostname()
            .await
            .map_err(|e| Error::dbus(format!("Failed to get hostname: {}", e)))?;

        Ok(hostname)
    }

    #[instrument(skip(self))]
    async fn get_machine_id(&self) -> Result<String> {
        debug!("getting machine ID");

        let proxy = self.create_proxy().await?;
        let machine_id = proxy
            .machine_id()
            .await
            .map_err(|e| Error::dbus(format!("Failed to get machine ID: {}", e)))?;

        Ok(machine_id)
    }
}
