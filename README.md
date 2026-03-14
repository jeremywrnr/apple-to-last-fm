# apple-to-last-fm

Scrobble Apple Music plays to Last.fm on macOS.

## Requirements

- macOS
- [Rust](https://rustup.rs/)
- A [Last.fm](https://www.last.fm) account

## Install

```bash
cargo build --release
sudo cp target/release/apple-to-last-fm /usr/local/bin/
```

## Setup

Authenticate with your Last.fm account (one time only):

```bash
apple-to-last-fm auth
```

Then install as a background daemon that starts automatically on login:

```bash
apple-to-last-fm install
```

That's it. Plays will be scrobbled automatically whenever Apple Music is running.

## Usage

```
apple-to-last-fm auth        Authenticate with Last.fm
apple-to-last-fm install     Install as a launchd daemon (starts on login)
apple-to-last-fm uninstall   Remove the daemon
apple-to-last-fm run         Run in the foreground (for testing)
apple-to-last-fm status      Show what is currently playing
apple-to-last-fm config      Show config path and current settings
```

## Logs

```bash
tail -f ~/Library/Logs/apple-to-last-fm/output.log
```

## Config

Config is written automatically to `~/Library/Application Support/apple-to-last-fm/config.toml`
by the `auth` command. The only field you may want to set manually:

```toml
poll_interval_secs = 10  # how often to check Apple Music (default: 10)
```

## License

[MIT](https://jeremywrnr.com/mit-license/)
