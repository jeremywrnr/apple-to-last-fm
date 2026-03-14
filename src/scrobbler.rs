use rustfm_scrobble::{Scrobble, Scrobbler};

use crate::config::Config;
use crate::error::{AppError, Result};
use crate::player::Track;

pub struct LastFmClient {
    scrobbler: Scrobbler,
}

impl LastFmClient {
    /// Build a client from config. Requires a session key to already be present
    /// (i.e. the user has run `auth`).
    pub fn new(config: &Config) -> Result<Self> {
        let mut scrobbler = Scrobbler::new(&config.lastfm_api_key, &config.lastfm_api_secret);

        match &config.lastfm_session_key {
            Some(key) => scrobbler.authenticate_with_session_key(key),
            None => {
                return Err(AppError::Config(
                    "No session key in config. Run 'apple-to-last-fm auth' first.".to_string(),
                ))
            }
        }

        Ok(Self { scrobbler })
    }

    pub fn now_playing(&mut self, track: &Track) -> Result<()> {
        let scrobble = to_scrobble(track);
        self.scrobbler
            .now_playing(&scrobble)
            .map_err(|e| AppError::Scrobbler(format!("{:?}", e)))?;
        Ok(())
    }

    pub fn scrobble(&mut self, track: &Track) -> Result<()> {
        let scrobble = to_scrobble(track);
        self.scrobbler
            .scrobble(&scrobble)
            .map_err(|e| AppError::Scrobbler(format!("{:?}", e)))?;
        Ok(())
    }
}

fn to_scrobble(track: &Track) -> Scrobble {
    Scrobble::new(&track.artist, &track.title, &track.album)
}
