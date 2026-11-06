use anyhow::Result;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration stored in `~/.config/pastebin-ai/config.toml`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Authentication configuration (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,
}

/// Authentication credentials for the pastebin.ai API.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Bearer token from pastebin.ai (starts with `pba_`)
    pub token: String,
}

impl Config {
    /// Load configuration from disk.
    ///
    /// Returns an empty config if the file doesn't exist.
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config { auth: None })
        }
    }

    /// Save configuration to disk.
    ///
    /// Creates the config directory if it doesn't exist.
    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

/// Get the path to the config file.
///
/// Returns `~/.config/pastebin-ai/config.toml` on Linux,
/// `~/Library/Application Support/pastebin-ai/config.toml` on macOS,
/// or the equivalent on Windows.
fn config_path() -> Result<PathBuf> {
    let config_dir = config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
    Ok(config_dir.join("pastebin-ai").join("config.toml"))
}