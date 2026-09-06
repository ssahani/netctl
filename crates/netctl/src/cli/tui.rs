// Copyright 2026 Zyvor AI Labs · https://zyvor.dev
// SPDX-License-Identifier: Apache-2.0

use clap::Args;
use miette::Result;

#[derive(Args)]
pub struct TuiArgs {}

impl TuiArgs {
    pub async fn execute(self) -> Result<()> {
        let app = crate::tui::App::new().await?;
        crate::tui::run(app).await
    }
}
