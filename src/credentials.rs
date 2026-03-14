/// Last.fm API credentials for this application.
/// These identify the app, not the user — the user authenticates separately
/// with their own username/password via `apple-to-last-fm auth`.
///
/// Set LASTFM_API_KEY and LASTFM_API_SECRET as environment variables at build time.
/// For CI: add them as repository secrets and pass to the build step.
/// For local builds: export them in your shell before running `cargo build`.
pub const API_KEY: &str = env!("LASTFM_API_KEY");
pub const API_SECRET: &str = env!("LASTFM_API_SECRET");
