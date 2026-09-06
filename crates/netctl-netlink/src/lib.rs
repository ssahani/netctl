// Copyright 2026 Zyvor AI Labs · https://zyvor.dev
// SPDX-License-Identifier: Apache-2.0

pub mod client;
pub mod ops;

pub use client::{NetlinkClient, NetlinkHandle};
pub use ops::{AddressOps, LinkOps};
