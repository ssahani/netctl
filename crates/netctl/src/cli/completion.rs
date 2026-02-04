use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use miette::Result;
use std::io;

#[derive(Args)]
pub struct CompletionArgs {
    /// Shell to generate completion for
    #[arg(value_enum)]
    shell: Shell,
}

impl CompletionArgs {
    pub async fn execute(self) -> Result<()> {
        let mut cmd = crate::cli::Cli::command();
        let name = cmd.get_name().to_string();

        generate(self.shell, &mut cmd, name, &mut io::stdout());

        Ok(())
    }
}
