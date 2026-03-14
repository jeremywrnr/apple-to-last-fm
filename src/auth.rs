use std::io::{self, Write};
use std::path::Path;

use rustfm_scrobble::Scrobbler;

use crate::credentials;
use crate::error::{AppError, Result};

/// Runs the interactive auth flow.
/// Prompts for Last.fm username and password, exchanges them for a session key,
/// and saves it to the config file. No API credentials are needed from the user.
pub fn run(config_path: &Path) -> Result<()> {
    println!("Authenticating with Last.fm");
    println!("(Your password is not stored — only the session key is saved.)\n");

    let username = prompt("Last.fm username: ")?;
    let password = prompt_password("Last.fm password: ")?;

    let mut scrobbler = Scrobbler::new(credentials::API_KEY, credentials::API_SECRET);
    let session = scrobbler
        .authenticate_with_password(&username, &password)
        .map_err(|e| AppError::Scrobbler(format!("{:?}", e)))?;

    println!("\nAuthenticated as {}.", session.name);

    // Load existing config if present (preserves poll_interval_secs etc.),
    // otherwise start fresh.
    let mut config = crate::config::Config::load(config_path)
        .unwrap_or_else(|_| crate::config::Config::new_empty());

    config.lastfm_session_key = Some(session.key);
    config.save(config_path)?;

    println!("Session key saved to {}.", config_path.display());
    println!("You can now run: apple-to-last-fm run");

    Ok(())
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
