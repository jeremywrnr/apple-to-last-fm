use std::io::{self, Write};
use std::path::Path;

use crate::error::Result;
use crate::scrobbler;

const API_ACCOUNT_URL: &str = "https://www.last.fm/api/account/create";

/// Runs the interactive auth flow.
/// 1. Prompts for API key/secret (opens browser to create one if needed)
/// 2. Uses desktop auth flow: opens browser for user to approve, no password needed
pub fn run(config_path: &Path) -> Result<()> {
    let mut config = crate::config::Config::load(config_path)
        .unwrap_or_else(|_| crate::config::Config::new_empty());

    if config.lastfm_session_key.is_some() {
        println!(
            "Note: already authenticated — re-authenticating will replace the existing session key.\n"
        );
    }

    // Step 1: Get API credentials
    if config.lastfm_api_key.is_none() || config.lastfm_api_secret.is_none() {
        println!("Step 1: Create a Last.fm API account");
        println!("Opening {} ...\n", API_ACCOUNT_URL);
        let _ = std::process::Command::new("open")
            .arg(API_ACCOUNT_URL)
            .status();
        println!("Fill in the form with:");
        println!("  Application name:        apple-to-last-fm");
        println!("  Application description: Scrobble Apple Music plays to Last.fm");
        println!("  Callback URL:            (leave blank)");
        println!(
            "  Application homepage:    https://github.com/jeremywrnr/apple-to-last-fm (optional)\n"
        );
        println!("After submitting, enter the credentials below.\n");
        let api_key = prompt("API key: ")?;
        let api_secret = prompt("Shared secret: ")?;
        config.lastfm_api_key = Some(api_key);
        config.lastfm_api_secret = Some(api_secret);
        // Save immediately so credentials aren't lost if the next step fails
        config.save(config_path)?;
        println!();
    }

    // Step 2: Desktop auth flow — authorize via browser, no password needed
    let api_key = config.api_key()?;
    let api_secret = config.api_secret()?;

    println!("Step 2: Authorize this app");
    let token = scrobbler::get_token(api_key, api_secret)?;

    let auth_url = format!(
        "https://www.last.fm/api/auth/?api_key={}&token={}",
        api_key, token
    );
    println!("Opening Last.fm in your browser...\n");
    let _ = std::process::Command::new("open").arg(&auth_url).status();

    wait_for_enter("Press Enter after you've approved access...")?;

    let session = scrobbler::get_session(api_key, api_secret, &token)?;

    println!("Authenticated as {}.", session.name);

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

fn wait_for_enter(label: &str) -> Result<()> {
    print!("{}", label);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
