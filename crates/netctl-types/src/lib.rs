//! Core types for netctl

pub mod error;
pub mod logging;
pub mod network;
pub mod traits;

pub use error::{Error, Result};
pub use network::{DhcpMode, IpNetwork, LinkInfo, LinkState, MacAddress, Route};
pub use traits::NetworkDevice;
