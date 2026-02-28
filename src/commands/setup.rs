use std::io::{self, Write};

use anyhow::Result;

pub fn run() -> Result<()> {
    let config = crate::config::load_config()?;
    let path = crate::config::config_path()?;

    // Step 1: Token
    let current_token_hint = config.token.as_ref().map(|t| {
        if t.len() > 8 {
            format!("{}...{}", &t[..4], &t[t.len() - 4..])
        } else {
            "****".to_string()
        }
    });
    if let Some(ref hint) = current_token_hint {
        print!("API token [current: {}]: ", hint);
    } else {
        print!("API token: ");
    }
    io::stdout().flush()?;
    let mut token_input = String::new();
    io::stdin().read_line(&mut token_input)?;
    let token_input = token_input.trim();

    // Step 2: Timezone
    let current_tz = config.timezone.as_deref().unwrap_or("UTC");
    print!("Timezone [current: {}]: ", current_tz);
    io::stdout().flush()?;
    let mut tz_input = String::new();
    io::stdin().read_line(&mut tz_input)?;
    let tz_input = tz_input.trim();

    // Apply
    let mut new_config = config;

    if !token_input.is_empty() {
        new_config.token = Some(token_input.to_string());
    }

    if !tz_input.is_empty() {
        tz_input
            .parse::<chrono_tz::Tz>()
            .map_err(|_| anyhow::anyhow!("invalid timezone: {}", tz_input))?;
        new_config.timezone = Some(tz_input.to_string());
    }

    crate::config::save_config(&new_config)?;
    println!("Configuration saved to {}", path.display());
    Ok(())
}
