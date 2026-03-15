/// Last.fm API credentials for this application.
/// These identify the app, not the user — the user authenticates separately
/// with their own username/password via `apple-to-last-fm auth`.
///
/// Set LASTFM_API_KEY and LASTFM_API_SECRET as environment variables at build time.
/// For CI: add them as repository secrets and pass to the build step.
/// For local builds: export them in your shell before running `cargo build`.
#[cfg(debug_assertions)]
pub const API_KEY: &str = match option_env!("LASTFM_API_KEY") {
    Some(v) => v,
    None => "dev-key",
};
#[cfg(not(debug_assertions))]
pub const API_KEY: &str = env!("LASTFM_API_KEY");

#[cfg(debug_assertions)]
pub const API_SECRET: &str = match option_env!("LASTFM_API_SECRET") {
    Some(v) => v,
    None => "dev-secret",
};
#[cfg(not(debug_assertions))]
pub const API_SECRET: &str = env!("LASTFM_API_SECRET");
