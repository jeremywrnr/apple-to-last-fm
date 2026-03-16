use std::path::PathBuf;
use std::process::Command;

use crate::error::{AppError, Result};

pub const LABEL: &str = "com.apple-to-last-fm";

fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

fn plist_path() -> PathBuf {
    home_dir()
        .join("Library")
        .join("LaunchAgents")
        .join(format!("{}.plist", LABEL))
}

fn log_dir() -> PathBuf {
    home_dir()
        .join("Library")
        .join("Logs")
        .join("apple-to-last-fm")
}

fn gui_session(uid: u32) -> String {
    format!("gui/{}", uid)
}

pub fn install() -> Result<()> {
    // Remove any existing daemon first so we always use the current binary.
    uninstall()?;

    let binary = std::env::current_exe()
        .map_err(|e| AppError::Config(format!("cannot determine binary path: {}", e)))?;

    let log_dir = log_dir();
    std::fs::create_dir_all(&log_dir)?;

    let plist = plist_path();
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{label}</string>

    <key>ProgramArguments</key>
    <array>
        <string>{binary}</string>
        <string>run</string>
    </array>

    <!-- Start immediately and restart automatically if it exits -->
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>

    <key>StandardOutPath</key>
    <string>{log_dir}/output.log</string>
    <key>StandardErrorPath</key>
    <string>{log_dir}/error.log</string>
</dict>
</plist>
"#,
        label = LABEL,
        binary = binary.display(),
        log_dir = log_dir.display(),
    );

    std::fs::write(&plist, plist_content)?;
    println!("Wrote {}.", plist.display());

    // Bootstrap into the current user's GUI session.
    let uid = get_uid()?;
    let plist_str = plist
        .to_str()
        .ok_or_else(|| AppError::Config("plist path is not valid UTF-8".to_string()))?;
    let status = Command::new("launchctl")
        .args(["bootstrap", &gui_session(uid), plist_str])
        .status()?;

    if !status.success() {
        return Err(AppError::Config(
            "launchctl bootstrap failed — is the daemon already installed? Try 'uninstall' first."
                .to_string(),
        ));
    }

    println!("Scrobbler installed and running in the background.");
    println!("It will start automatically on login.\n");
    println!("Play a song in Apple Music, then verify with:");
    println!("  apple-to-last-fm logs\n");
    println!("To remove: apple-to-last-fm uninstall");

    Ok(())
}

pub fn uninstall() -> Result<()> {
    let uid = get_uid()?;
    let session = gui_session(uid);

    // Try bootout by plist path first, then by service label as fallback.
    let plist = plist_path();
    if plist.exists() {
        let plist_str = plist.to_string_lossy();
        let _ = Command::new("launchctl")
            .args(["bootout", &session, &*plist_str])
            .stderr(std::process::Stdio::null())
            .status();
        std::fs::remove_file(&plist)?;
    }

    let service = format!("{}/{}", session, LABEL);
    let _ = Command::new("launchctl")
        .args(["bootout", &service])
        .stderr(std::process::Stdio::null())
        .status();

    println!("Daemon stopped and uninstalled.");
    Ok(())
}

fn get_uid() -> Result<u32> {
    // SAFETY: getuid() is always safe to call.
    Ok(unsafe { libc::getuid() })
}
