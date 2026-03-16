use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};

const DEFAULT_POLL_INTERVAL_SECS: u64 = 10;
const APP_DIR: &str = "apple-to-last-fm";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Last.fm API key (obtained from https://www.last.fm/api/account/create)
    #[serde(default)]
    pub lastfm_api_key: Option<String>,

    /// Last.fm API shared secret
    #[serde(default)]
    pub lastfm_api_secret: Option<String>,

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
            lastfm_api_key: None,
            lastfm_api_secret: None,
            lastfm_session_key: None,
            poll_interval_secs: DEFAULT_POLL_INTERVAL_SECS,
        }
    }

    /// Returns true if all Last.fm credentials are present.
    pub fn is_authenticated(&self) -> bool {
        self.lastfm_api_key.is_some()
            && self.lastfm_api_secret.is_some()
            && self.lastfm_session_key.is_some()
    }

    pub fn api_key(&self) -> Result<&str> {
        self.lastfm_api_key.as_deref().ok_or_else(|| {
            AppError::Config("No API key in config. Run 'apple-to-last-fm auth' first.".into())
        })
    }

    pub fn api_secret(&self) -> Result<&str> {
        self.lastfm_api_secret.as_deref().ok_or_else(|| {
            AppError::Config("No API secret in config. Run 'apple-to-last-fm auth' first.".into())
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_with_session_key() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");

        let mut config = Config::new_empty();
        config.lastfm_session_key = Some("abc123".to_string());
        config.poll_interval_secs = 30;
        config.save(&path).unwrap();

        let loaded = Config::load(&path).unwrap();
        assert_eq!(loaded.lastfm_session_key.as_deref(), Some("abc123"));
        assert_eq!(loaded.poll_interval_secs, 30);
    }

    #[test]
    fn roundtrip_empty_uses_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");

        Config::new_empty().save(&path).unwrap();

        let loaded = Config::load(&path).unwrap();
        assert_eq!(loaded.lastfm_session_key, None);
        assert_eq!(loaded.poll_interval_secs, DEFAULT_POLL_INTERVAL_SECS);
    }

    #[test]
    fn load_missing_file_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.toml");
        assert!(Config::load(&path).is_err());
    }

    #[test]
    fn save_creates_parent_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a").join("b").join("config.toml");
        Config::new_empty().save(&path).unwrap();
        assert!(path.exists());
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
