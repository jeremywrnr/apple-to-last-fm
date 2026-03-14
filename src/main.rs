mod auth;
mod config;
mod credentials;
mod daemon;
mod error;
mod player;
mod scrobbler;
mod state;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::{error, info, warn};
use tracing_subscriber::prelude::*;
use tracing_subscriber::filter::Targets;

use config::{default_config_path, Config};
use error::Result;
use scrobbler::LastFmClient;
use state::{Action, ScrobbleStateMachine};

#[derive(Parser)]
#[command(name = "apple-to-last-fm", about = "Scrobble Apple Music to Last.fm")]
struct Cli {
    /// Path to config file
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Authenticate with Last.fm and save a session key to config
    Auth,

    /// Install as a launchd agent (starts on login, restarts automatically)
    Install,

    /// Remove the launchd agent
    Uninstall,

    /// Stream live logs from the daemon in the terminal
    Logs,

    /// Start scrobbling (runs in foreground)
    Run,

    /// Show what is currently playing in Apple Music
    Status,

    /// Print the resolved config path and contents
    Config,
}

fn main() -> Result<()> {
    // Only forward logs from this crate — suppress rustls/ureq/etc. noise.
    let filter = Targets::new().with_target("apple_to_last_fm", tracing::Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_oslog::OsLogger::new("com.apple-to-last-fm", "default").with_filter(filter))
        .init();

    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(default_config_path);

    match cli.command {
        Command::Auth => auth::run(&config_path),
        Command::Install => daemon::install(),
        Command::Uninstall => daemon::uninstall(),
        Command::Logs => cmd_logs(),
        Command::Status => cmd_status(),
        Command::Config => cmd_config(&config_path),
        Command::Run => cmd_run(&config_path),
    }
}

fn cmd_logs() -> Result<()> {
    std::process::Command::new("log")
        .args([
            "stream",
            "--predicate",
            "subsystem == \"com.apple-to-last-fm\"",
            "--level",
            "info",
        ])
        .status()?;
    Ok(())
}


fn cmd_status() -> Result<()> {
    match player::current_track()? {
        Some(track) => println!("Now playing: {}", track),
        None => println!("Apple Music is not playing."),
    }
    Ok(())
}

fn cmd_config(config_path: &std::path::Path) -> Result<()> {
    println!("Config path: {}", config_path.display());
    match Config::load(config_path) {
        Ok(config) => {
            println!("  poll_interval_secs: {}", config.poll_interval_secs);
            println!(
                "  lastfm_session_key: {}",
                config
                    .lastfm_session_key
                    .as_deref()
                    .unwrap_or("(not set — run 'auth' first)"),
            );
        }
        Err(_) => println!("  (no config file — run 'auth' to set up)"),
    }
    Ok(())
}

fn cmd_run(config_path: &std::path::Path) -> Result<()> {
    let config = Config::load(config_path)?;
    let mut client = LastFmClient::new(&config)?;
    let mut sm = ScrobbleStateMachine::new();
    let interval = std::time::Duration::from_secs(config.poll_interval_secs);

    info!(
        "Starting scrobbler (polling every {}s)",
        config.poll_interval_secs
    );
    println!("Scrobbler running. Press Ctrl-C to stop.");

    loop {
        let current = match player::current_track() {
            Ok(track) => track,
            Err(e) => {
                warn!("Error polling Apple Music: {}", e);
                None
            }
        };

        for action in sm.tick(current.as_ref()) {
            match action {
                Action::SendNowPlaying(track) => {
                    info!("Now playing: {}", track);
                    if let Err(e) = client.now_playing(&track) {
                        error!("Failed to send now-playing: {}", e);
                    }
                }
                Action::Scrobble(track) => {
                    info!("Scrobbling: {}", track);
                    if let Err(e) = client.scrobble(&track) {
                        error!("Failed to scrobble: {}", e);
                    }
                }
            }
        }

        std::thread::sleep(interval);
    }
}
