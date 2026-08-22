// Copyright 2026 Zyvor AI Labs Private Limited
// SPDX-License-Identifier: Apache-2.0

//! Core types for netctl

pub mod error;
pub mod logging;
pub mod network;
pub mod traits;

pub use error::{Error, Result};
pub use network::{DhcpMode, IpNetwork, LinkInfo, LinkState, MacAddress, Route};
pub use traits::NetworkDevice;
