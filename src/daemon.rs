use std::path::PathBuf;
use std::process::Command;

use crate::error::{AppError, Result};

const LABEL: &str = "com.apple-to-last-fm";

fn plist_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Library")
        .join("LaunchAgents")
        .join(format!("{}.plist", LABEL))
}

fn log_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Library")
        .join("Logs")
        .join("apple-to-last-fm")
}

pub fn install() -> Result<()> {
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
    let status = Command::new("launchctl")
        .args(["bootstrap", &format!("gui/{}", uid), plist.to_str().unwrap()])
        .status()?;

    if !status.success() {
        return Err(AppError::Config(
            "launchctl bootstrap failed — is the daemon already installed? Try 'uninstall' first."
                .to_string(),
        ));
    }

    println!("Daemon installed and started.");
    println!("Logs: {}/", log_dir.display());
    println!("To stop: apple-to-last-fm uninstall");

    Ok(())
}

pub fn uninstall() -> Result<()> {
    let plist = plist_path();

    if !plist.exists() {
        println!("Daemon is not installed (no plist at {}).", plist.display());
        return Ok(());
    }

    let uid = get_uid()?;
    let _ = Command::new("launchctl")
        .args(["bootout", &format!("gui/{}", uid), plist.to_str().unwrap()])
        .status();

    std::fs::remove_file(&plist)?;
    println!("Daemon stopped and uninstalled.");

    Ok(())
}

fn get_uid() -> Result<u32> {
    // SAFETY: getuid() is always safe to call.
    Ok(unsafe { libc::getuid() })
}
