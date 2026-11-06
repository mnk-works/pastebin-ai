use anyhow::Result;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub token: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config { auth: None })
        }
    }

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

fn config_path() -> Result<PathBuf> {
    let config_dir = config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
    Ok(config_dir.join("pastebin-ai").join("config.toml"))
}