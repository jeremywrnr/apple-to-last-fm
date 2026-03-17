use std::process::Command;

use apple_music::{AppleMusic, PlayerState};
use tracing::debug;

use crate::error::Result;

/// Normalized track data we care about for scrobbling.
#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    /// Duration in seconds
    pub duration_secs: f64,
}

impl Track {
    /// Returns true if this track is the same song as another (ignoring play position).
    pub fn is_same_song(&self, other: &Track) -> bool {
        self.title == other.title && self.artist == other.artist && self.album == other.album
    }
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" by {} ({})", self.title, self.artist, self.album)
    }
}

/// Check if Apple Music is running without launching it.
pub fn is_music_running() -> bool {
    Command::new("pgrep")
        .args(["-x", "Music"])
        .output()
        .is_ok_and(|o| o.status.success())
}

/// Returns the currently playing track, or `None` if Apple Music is stopped/paused.
pub fn current_track() -> Result<Option<Track>> {
    if !is_music_running() {
        debug!("Apple Music is not running — skipping");
        return Ok(None);
    }

    let app_data = AppleMusic::get_application_data()?;

    match app_data.player_state {
        Some(PlayerState::Playing) => {}
        state => {
            debug!("Apple Music player state: {:?} — skipping", state);
            return Ok(None);
        }
    }

    match AppleMusic::get_current_track() {
        Ok(raw) => {
            let track = Track {
                title: raw.name,
                artist: raw.artist,
                album: raw.album,
                duration_secs: raw.duration,
            };
            debug!("Current track: {}", track);
            Ok(Some(track))
        }
        Err(apple_music::Error::NotPlaying) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_track(title: &str, artist: &str, album: &str) -> Track {
        Track {
            title: title.to_string(),
            artist: artist.to_string(),
            album: album.to_string(),
            duration_secs: 200.0,
        }
    }

    #[test]
    fn same_song_matches() {
        let a = make_track("Song A", "Artist", "Album");
        let b = make_track("Song A", "Artist", "Album");
        assert!(a.is_same_song(&b));
    }

    #[test]
    fn different_title_no_match() {
        let a = make_track("Song A", "Artist", "Album");
        let b = make_track("Song B", "Artist", "Album");
        assert!(!a.is_same_song(&b));
    }

    #[test]
    fn different_artist_no_match() {
        let a = make_track("Song A", "Artist 1", "Album");
        let b = make_track("Song A", "Artist 2", "Album");
        assert!(!a.is_same_song(&b));
    }

    #[test]
    fn display_format() {
        let t = make_track("Bohemian Rhapsody", "Queen", "A Night at the Opera");
        assert_eq!(
            format!("{}", t),
            "\"Bohemian Rhapsody\" by Queen (A Night at the Opera)"
        );
    }
}
