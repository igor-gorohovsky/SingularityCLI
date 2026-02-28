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
    #[command(name = "set-timezone", about = "Save timezone (IANA format) to config")]
    SetTimezone {
        #[arg(help = "IANA timezone name (e.g. Europe/Kyiv, America/New_York)")]
        timezone: String,
    },
}

pub fn run(cmd: ConfigCmd) -> Result<()> {
    match cmd {
        ConfigCmd::SetToken { token } => crate::config::set_token(&token),
        ConfigCmd::SetTimezone { timezone } => {
            timezone
                .parse::<chrono_tz::Tz>()
                .map_err(|_| anyhow::anyhow!("invalid timezone: {}", timezone))?;
            crate::config::set_timezone(&timezone)
        }
    }
}
