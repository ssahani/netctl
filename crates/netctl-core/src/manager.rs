use netctl_dbus::{
    services::{HostnamedOps, NetworkdOps, ResolvedOps},
    DbusClient,
};
use netctl_netlink::{AddressOps, LinkOps, NetlinkClient};
use netctl_types::{IpNetwork, LinkInfo, Result};
use std::net::IpAddr;
use tracing::{info, instrument};

pub struct NetworkManager {
    netlink: NetlinkClient,
    dbus: DbusClient,
}

impl NetworkManager {
    pub async fn new() -> Result<Self> {
        let (netlink, conn) = NetlinkClient::new()?;
        tokio::spawn(conn);

        let dbus = DbusClient::new().await?;

        Ok(Self { netlink, dbus })
    }

    // Link queries

    #[instrument(skip(self))]
    pub async fn list_links(&self) -> Result<Vec<LinkInfo>> {
        info!("listing all network links");
        let handle = self.netlink.cloneable_handle();
        handle.list_links().await
    }

    #[instrument(skip(self))]
    pub async fn get_link_info(&self, ifname: &str) -> Result<LinkInfo> {
        info!(%ifname, "getting link information");
        let handle = self.netlink.cloneable_handle();
        handle.get_link_info(ifname).await
    }

    // Link management

    #[instrument(skip(self))]
    pub async fn set_link_up(&self, ifname: &str) -> Result<()> {
        info!(%ifname, "bringing link up");
        let handle = self.netlink.cloneable_handle();
        let index = handle.get_link_by_name(ifname).await?;
        handle.set_link_up(index).await
    }

    #[instrument(skip(self))]
    pub async fn set_link_down(&self, ifname: &str) -> Result<()> {
        info!(%ifname, "bringing link down");
        let handle = self.netlink.cloneable_handle();
        let index = handle.get_link_by_name(ifname).await?;
        handle.set_link_down(index).await
    }

    #[instrument(skip(self))]
    pub async fn set_mtu(&self, ifname: &str, mtu: u32) -> Result<()> {
        info!(%ifname, mtu, "setting MTU");
        let handle = self.netlink.cloneable_handle();
        let index = handle.get_link_by_name(ifname).await?;
        handle.set_link_mtu(index, mtu).await
    }

    #[instrument(skip(self))]
    pub async fn add_address(&self, ifname: &str, network: IpNetwork) -> Result<()> {
        info!(%ifname, %network, "adding address");
        let handle = self.netlink.cloneable_handle();
        let index = handle.get_link_by_name(ifname).await?;
        handle.add_address(index, network).await
    }

    // D-Bus operations - systemd-networkd

    #[instrument(skip(self))]
    pub async fn reload_networkd(&self) -> Result<()> {
        info!("reloading systemd-networkd");
        self.dbus.networkd().reload().await
    }

    #[instrument(skip(self))]
    pub async fn reconfigure_link(&self, ifname: &str) -> Result<()> {
        info!(%ifname, "reconfiguring link via networkd");
        let handle = self.netlink.cloneable_handle();
        let index = handle.get_link_by_name(ifname).await?;
        self.dbus.networkd().reconfigure_link(index).await
    }

    // D-Bus operations - systemd-resolved

    #[instrument(skip(self, servers), fields(ifname = %ifname, server_count = servers.len()))]
    pub async fn set_dns_servers(&self, ifname: &str, servers: Vec<IpAddr>) -> Result<()> {
        info!(%ifname, server_count = servers.len(), "setting DNS servers");
        let handle = self.netlink.cloneable_handle();
        let index = handle.get_link_by_name(ifname).await?;
        self.dbus.resolved().set_link_dns(index, servers).await
    }

    #[instrument(skip(self, domains), fields(ifname = %ifname, domain_count = domains.len()))]
    pub async fn set_dns_domains(&self, ifname: &str, domains: Vec<String>) -> Result<()> {
        info!(%ifname, domain_count = domains.len(), "setting DNS domains");
        let handle = self.netlink.cloneable_handle();
        let index = handle.get_link_by_name(ifname).await?;
        self.dbus.resolved().set_link_domains(index, domains).await
    }

    #[instrument(skip(self))]
    pub async fn revert_dns(&self, ifname: &str) -> Result<()> {
        info!(%ifname, "reverting DNS configuration");
        let handle = self.netlink.cloneable_handle();
        let index = handle.get_link_by_name(ifname).await?;
        self.dbus.resolved().revert_link(index).await
    }

    #[instrument(skip(self))]
    pub async fn flush_dns_caches(&self) -> Result<()> {
        info!("flushing DNS caches");
        self.dbus.resolved().flush_caches().await
    }

    // D-Bus operations - systemd-hostnamed

    #[instrument(skip(self))]
    pub async fn set_hostname(&self, hostname: &str) -> Result<()> {
        info!(hostname = %hostname, "setting static hostname");
        self.dbus.hostnamed().set_static_hostname(hostname).await
    }

    #[instrument(skip(self))]
    pub async fn get_hostname(&self) -> Result<String> {
        info!("getting hostname");
        self.dbus.hostnamed().get_hostname().await
    }

    #[instrument(skip(self))]
    pub async fn get_machine_id(&self) -> Result<String> {
        info!("getting machine ID");
        self.dbus.hostnamed().get_machine_id().await
    }
}
