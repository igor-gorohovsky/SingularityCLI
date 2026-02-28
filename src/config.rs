use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

const ENV_VAR: &str = "SINGULARITY_TOKEN";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

pub fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("could not determine config directory")?;
    Ok(config_dir.join("singularity").join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    match fs::read_to_string(&path) {
        Ok(content) => {
            let config: Config = toml::from_str(&content).context("invalid config file")?;
            Ok(config)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
        Err(e) => Err(e).with_context(|| format!("failed to read {}", path.display())),
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let content = toml::to_string_pretty(config).context("failed to serialize config")?;
    fs::write(&path, content).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub fn resolve_token() -> Result<String> {
    if let Ok(token) = std::env::var(ENV_VAR)
        && !token.is_empty()
    {
        return Ok(token);
    }

    let config = load_config()?;
    if let Some(token) = config.token
        && !token.is_empty()
    {
        return Ok(token);
    }

    bail!(
        "no API token found. Set {} env var or run: singularity config set-token <TOKEN>",
        ENV_VAR
    )
}

pub fn set_token(token: &str) -> Result<()> {
    let mut config = load_config()?;
    config.token = Some(token.to_string());
    save_config(&config)?;
    let path = config_path()?;
    println!("Token saved to {}", path.display());
    Ok(())
}

pub fn set_timezone(timezone: &str) -> Result<()> {
    let mut config = load_config()?;
    config.timezone = Some(timezone.to_string());
    save_config(&config)?;
    let path = config_path()?;
    println!("Timezone saved to {}", path.display());
    Ok(())
}

pub fn resolve_token_and_timezone() -> Result<(String, Option<chrono_tz::Tz>)> {
    let env_token = std::env::var(ENV_VAR).ok().filter(|t| !t.is_empty());
    let config = load_config()?;

    let token = match env_token {
        Some(t) => t,
        None => config.token.filter(|t| !t.is_empty()).ok_or_else(|| {
            anyhow::anyhow!(
                "no API token found. Set {} env var or run: singularity config set-token <TOKEN>",
                ENV_VAR
            )
        })?,
    };

    let tz = config.timezone.and_then(|s| {
        s.parse::<chrono_tz::Tz>()
            .map_err(|_| eprintln!("warning: invalid timezone '{}' in config, using UTC", s))
            .ok()
    });

    Ok((token, tz))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_token_from_env_var() {
        let key = "SINGULARITY_TOKEN_TEST_1";
        unsafe { std::env::set_var(key, "my-secret") };
        let val = std::env::var(key).unwrap();
        assert_eq!(val, "my-secret");
        unsafe { std::env::remove_var(key) };
    }

    #[test]
    fn resolve_token_reads_toml() {
        let content = "token = \"from-file\"\n";
        let config: Config = toml::from_str(content).unwrap();
        assert_eq!(config.token.as_deref(), Some("from-file"));
    }

    #[test]
    fn resolve_token_empty_toml_yields_none() {
        let content = "token = \"\"\n";
        let config: Config = toml::from_str(content).unwrap();
        assert!(config.token.as_deref().unwrap().is_empty());
    }

    #[test]
    fn config_path_is_under_config_dir() {
        let path = config_path().unwrap();
        assert!(path.ends_with("singularity/config.toml"));
    }

    #[test]
    fn config_roundtrip_with_timezone() {
        let config: Config =
            toml::from_str("token = \"abc\"\ntimezone = \"Europe/Kyiv\"\n").unwrap();
        assert_eq!(config.token.as_deref(), Some("abc"));
        assert_eq!(config.timezone.as_deref(), Some("Europe/Kyiv"));
        let serialized = toml::to_string_pretty(&config).unwrap();
        assert!(serialized.contains("token"));
        assert!(serialized.contains("timezone"));
    }

    #[test]
    fn config_default_has_no_fields() {
        let config = Config::default();
        assert!(config.token.is_none());
        assert!(config.timezone.is_none());
    }

    #[test]
    fn config_deserialize_token_only() {
        let config: Config = toml::from_str("token = \"abc\"\n").unwrap();
        assert_eq!(config.token.as_deref(), Some("abc"));
        assert!(config.timezone.is_none());
    }
}
