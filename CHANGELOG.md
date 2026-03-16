# Changelog

## 1.1.0

- Browser-based auth flow — no password ever touches the terminal
- `install` command now handles auth automatically (prompts if needed)
- `install --reauth` flag to force re-authentication
- Per-user API keys stored in config (no build-time secrets required)
- `cargo install apple-to-last-fm` works without any environment variables
- Daemon auto-uninstalls before reinstalling to avoid stale binaries
- Suppress noisy `launchctl` errors on fresh installs
- Save Last.fm username to config for personalized profile links
- Removed `rpassword` and `rustfm-scrobble` dependencies
- Added `justfile` with dev commands and release automation

## 1.0.2

- Replaced `rustfm-scrobble` with direct Last.fm API calls (`reqwest` + `md-5`)
- Removed `[patch.crates-io]` for `wrapped-vec` — unblocks publishing to crates.io
- Removed unused `tokio` dependency
- Cleaned up stale patches directory

## 1.0.0

- Feature-complete release
- Scrobble and now-playing support
- Background daemon via launchd (starts on login, auto-restarts)
- CLI: auth, install, uninstall, run, status, logs, config
- Published to crates.io and Homebrew

## 0.1.2

- First crates.io release

## 0.1.1

- CI and build improvements

## 0.1.0

- Initial release
