mod config;
mod error;
mod player;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::info;

use config::{default_config_path, Config};
use error::Result;

#[derive(Parser)]
#[command(name = "apple-to-last-fm", about = "Scrobble Apple Music to Last.fm")]
struct Cli {
    /// Path to config file (default: ~/.config/apple-to-last-fm/config.toml)
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start scrobbling (runs in foreground)
    Run,

    /// Show what is currently playing in Apple Music
    Status,

    /// Print the resolved config path and contents
    Config,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(default_config_path);

    match cli.command {
        Command::Status => cmd_status(),
        Command::Config => cmd_config(&config_path),
        Command::Run => cmd_run(&config_path),
    }
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
    let config = Config::load(config_path)?;
    println!("  poll_interval_secs: {}", config.poll_interval_secs);
    println!("  lastfm_api_key:     {}", config.lastfm_api_key);
    println!(
        "  lastfm_session_key: {}",
        config
            .lastfm_session_key
            .as_deref()
            .unwrap_or("(not set — run 'auth' first)"),
    );
    Ok(())
}

fn cmd_run(config_path: &std::path::Path) -> Result<()> {
    let config = Config::load(config_path)?;
    let interval = std::time::Duration::from_secs(config.poll_interval_secs);

    info!(
        "Starting scrobbler (polling every {}s)",
        config.poll_interval_secs
    );
    println!("Scrobbler running. Press Ctrl-C to stop.");

    loop {
        match player::current_track() {
            Ok(Some(track)) => info!("Now playing: {}", track),
            Ok(None) => {}
            Err(e) => eprintln!("Error polling Apple Music: {}", e),
        }
        std::thread::sleep(interval);
    }
}
