use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};

const DEFAULT_POLL_INTERVAL_SECS: u64 = 10;
const APP_DIR: &str = "apple-to-last-fm";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Last.fm session key (obtained after running `auth`, stored here)
    #[serde(default)]
    pub lastfm_session_key: Option<String>,

    /// How often to poll Apple Music for the current track, in seconds
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
}

fn default_poll_interval() -> u64 {
    DEFAULT_POLL_INTERVAL_SECS
}

impl Config {
    pub fn new_empty() -> Self {
        Config {
            lastfm_session_key: None,
            poll_interval_secs: DEFAULT_POLL_INTERVAL_SECS,
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AppError::Config(format!("cannot read {}: {}", path.display(), e)))?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Returns the default config file path:
/// ~/Library/Application Support/apple-to-last-fm/config.toml  (macOS)
pub fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(APP_DIR)
        .join(CONFIG_FILE)
}
