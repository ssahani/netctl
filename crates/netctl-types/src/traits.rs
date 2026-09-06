// Copyright 2026 Zyvor AI Labs · https://zyvor.dev
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

#[async_trait]
pub trait NetworkDevice: Send + Sync {
    fn name(&self) -> &str;
    fn index(&self) -> u32;
}
