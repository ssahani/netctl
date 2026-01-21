use netctl_types::Result;
use rtnetlink::Handle;
use std::sync::Arc;

#[derive(Clone)]
pub struct NetlinkClient {
    handle: Arc<Handle>,
}

impl NetlinkClient {
    pub fn new() -> Result<(Self, impl std::future::Future<Output = ()>)> {
        let (conn, handle, _) = rtnetlink::new_connection()
            .map_err(|e| netctl_types::Error::netlink(format!("{}", e)))?;

        Ok((
            Self {
                handle: Arc::new(handle),
            },
            conn,
        ))
    }

    pub fn cloneable_handle(&self) -> NetlinkHandle {
        NetlinkHandle {
            handle: Arc::clone(&self.handle),
        }
    }
}

#[derive(Clone)]
pub struct NetlinkHandle {
    handle: Arc<Handle>,
}

impl NetlinkHandle {
    pub fn handle(&self) -> &Handle {
        &self.handle
    }
}
