use crate::services::{HostnamedService, NetworkdService, ResolvedService};
use netctl_types::Result;
use std::sync::Arc;
use zbus::Connection;

#[derive(Clone)]
pub struct DbusClient {
    connection: Arc<Connection>,
}

impl DbusClient {
    pub async fn new() -> Result<Self> {
        let conn = Connection::system()
            .await
            .map_err(|e| netctl_types::Error::dbus(format!("{}", e)))?;

        Ok(Self {
            connection: Arc::new(conn),
        })
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn networkd(&self) -> NetworkdService {
        NetworkdService::new(Arc::clone(&self.connection))
    }

    pub fn resolved(&self) -> ResolvedService {
        ResolvedService::new(Arc::clone(&self.connection))
    }

    pub fn hostnamed(&self) -> HostnamedService {
        HostnamedService::new(Arc::clone(&self.connection))
    }
}
