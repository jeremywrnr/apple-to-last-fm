//! Last.fm API credentials for this application.
//! Set LASTFM_API_KEY and LASTFM_API_SECRET as environment variables at build time.
//! For CI: add them as repository secrets and pass to the build step.
//! For local builds: export them in your shell before running `cargo build`.

pub const API_KEY: &str = match option_env!("LASTFM_API_KEY") {
    Some(v) => v,
    None => "dev-key",
};

pub const API_SECRET: &str = match option_env!("LASTFM_API_SECRET") {
    Some(v) => v,
    None => "dev-secret",
};
