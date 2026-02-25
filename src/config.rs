use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};

const ENV_VAR: &str = "SINGULARITY_TOKEN";

fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("could not determine config directory")?;
    Ok(config_dir.join("singularity").join("config.toml"))
}

pub fn resolve_token() -> Result<String> {
    if let Ok(token) = std::env::var(ENV_VAR) {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    let path = config_path()?;
    if path.exists() {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let table: toml::Table = content.parse().context("invalid config file")?;
        if let Some(token) = table.get("token").and_then(|v| v.as_str()) {
            if !token.is_empty() {
                return Ok(token.to_string());
            }
        }
    }

    bail!(
        "no API token found. Set {} env var or run: singularity config set-token <TOKEN>",
        ENV_VAR
    )
}

pub fn set_token(token: &str) -> Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let content = format!("token = \"{}\"\n", token);
    fs::write(&path, content).with_context(|| format!("failed to write {}", path.display()))?;
    println!("Token saved to {}", path.display());
    Ok(())
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
        let table: toml::Table = content.parse().unwrap();
        let token = table.get("token").and_then(|v| v.as_str()).unwrap();
        assert_eq!(token, "from-file");
    }

    #[test]
    fn resolve_token_empty_toml_yields_none() {
        let content = "token = \"\"\n";
        let table: toml::Table = content.parse().unwrap();
        let token = table.get("token").and_then(|v| v.as_str()).unwrap();
        assert!(token.is_empty());
    }

    #[test]
    fn config_path_is_under_config_dir() {
        let path = config_path().unwrap();
        assert!(path.ends_with("singularity/config.toml"));
    }
}
