use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCmd {
    #[command(
        name = "set-token",
        about = "Save API bearer token to ~/.config/singularity/config.toml"
    )]
    SetToken {
        #[arg(help = "API bearer token from Singularity app")]
        token: String,
    },
}

pub fn run(cmd: ConfigCmd) -> Result<()> {
    match cmd {
        ConfigCmd::SetToken { token } => crate::config::set_token(&token),
    }
}
