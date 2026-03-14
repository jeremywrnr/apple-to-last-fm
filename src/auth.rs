use std::io::{self, Write};
use std::path::Path;

use rustfm_scrobble::Scrobbler;

use crate::config::Config;
use crate::error::{AppError, Result};

/// Runs the interactive auth flow.
///
/// On first run (no config file), also prompts for API key and secret.
/// On subsequent runs, loads the existing config and just re-authenticates.
/// Either way, the session key is written back to the config file on success.
pub fn run(config_path: &Path) -> Result<()> {
    println!("Authenticating with Last.fm");
    println!("(Your password is not stored — only the session key is saved.)\n");

    let mut config = load_or_create_config(config_path)?;

    let username = prompt("Last.fm username: ")?;
    let password = prompt_password("Last.fm password: ")?;

    let mut scrobbler = Scrobbler::new(&config.lastfm_api_key, &config.lastfm_api_secret);
    let session = scrobbler
        .authenticate_with_password(&username, &password)
        .map_err(|e| AppError::Scrobbler(format!("{:?}", e)))?;

    println!("\nAuthenticated as {}.", session.name);

    config.lastfm_session_key = Some(session.key);
    config.save(config_path)?;

    println!("Session key saved to {}.", config_path.display());
    println!("You can now run: apple-to-last-fm run");

    Ok(())
}

/// Loads an existing config, or walks the user through creating one from scratch.
fn load_or_create_config(config_path: &Path) -> Result<Config> {
    match Config::load(config_path) {
        Ok(config) => Ok(config),
        Err(_) => {
            println!("No config found at {}.", config_path.display());
            println!("Get your API credentials at https://www.last.fm/api/account/create\n");

            let api_key = prompt("Last.fm API key: ")?;
            let api_secret = prompt("Last.fm API secret: ")?;

            Ok(Config {
                lastfm_api_key: api_key,
                lastfm_api_secret: api_secret,
                lastfm_session_key: None,
                poll_interval_secs: 10,
            })
        }
    }
}

fn prompt(label: &str) -> Result<String> {
    print!("{}", label);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_password(label: &str) -> Result<String> {
    print!("{}", label);
    io::stdout().flush()?;
    rpassword::read_password().map_err(AppError::Io)
}
