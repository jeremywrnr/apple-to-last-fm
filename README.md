# apple-to-last-fm

Scrobble Apple Music plays to Last.fm on macOS.

## Requirements

- macOS
- A [Last.fm](https://www.last.fm) account

## Install

```bash
brew tap jeremywrnr/tap
brew install apple-to-last-fm
```

Or install from [crates.io](https://crates.io/crates/apple-to-last-fm):

```bash
cargo install apple-to-last-fm
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
apple-to-last-fm logs        Stream live logs from the daemon
apple-to-last-fm config      Show config path and current settings
```

## Config

Config is written automatically to `~/Library/Application Support/apple-to-last-fm/config.toml`
by the `auth` command. The only field you may want to set manually:

```toml
poll_interval_secs = 10  # how often to check Apple Music (default: 10)
```

## License

[MIT](https://jeremywrnr.com/mit-license/)
