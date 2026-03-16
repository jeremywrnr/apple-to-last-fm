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

```bash
apple-to-last-fm install
```

This handles everything: authenticates with Last.fm (if not already done) and installs a
background daemon that starts automatically on login. No passwords are stored — authentication
is handled entirely through your browser. Plays will be scrobbled whenever Apple Music is running.

You can manage authorized applications at [Last.fm settings](https://www.last.fm/settings/applications).

## Usage

```
apple-to-last-fm install     Authenticate (if needed) and install as a daemon
apple-to-last-fm uninstall   Remove the daemon
apple-to-last-fm auth        Re-authenticate with Last.fm
apple-to-last-fm status      Show what is currently playing
apple-to-last-fm logs        Stream live logs from the daemon
apple-to-last-fm run         Run in the foreground (for testing)
apple-to-last-fm config      Show config path and current settings
```

## Verify

Check that scrobbles are appearing on your [Last.fm profile](https://www.last.fm/user/jwrnr)
(replace `jwrnr` with your username). Play a song in Apple Music and it should show up
within a couple of minutes.

![Recent Tracks on Last.fm](https://raw.githubusercontent.com/jeremywrnr/apple-to-last-fm/main/assets/playing.png)

## Debugging

Stream live daemon logs to see scrobbles and errors:

```
$ apple-to-last-fm logs
2026-03-16 00:08:29 apple-to-last-fm: Now playing: "Dirty Work" by Steely Dan (Can't Buy A Thrill)
2026-03-16 00:10:13 apple-to-last-fm: Scrobbling: "Dirty Work" by Steely Dan (Can't Buy A Thrill)
```

## Config

Config is written automatically to `~/Library/Application Support/apple-to-last-fm/config.toml`
by the `auth` command. The only field you may want to set manually:

```toml
poll_interval_secs = 10  # how often to check Apple Music (default: 10)
```

## Development

Requires [just](https://just.systems/man/en/) for task running.

```bash
git clone https://github.com/jeremywrnr/apple-to-last-fm.git
cd apple-to-last-fm
just        # list available commands
just build  # build in debug mode
just test   # run tests
just check  # run tests, linter, and format check
```

## License

[MIT](https://jeremywrnr.com/mit-license/)
