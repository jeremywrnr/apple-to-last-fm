use std::time::Instant;

use crate::player::Track;

/// Minimum seconds played before a track is eligible to scrobble.
const MIN_SCROBBLE_SECS: f64 = 30.0;
/// Minimum fraction of track duration that must have played.
const MIN_SCROBBLE_FRACTION: f64 = 0.5;

/// Actions the state machine wants the caller to perform.
#[derive(Debug, Clone)]
pub enum Action {
    SendNowPlaying(Track),
    Scrobble(Track),
}

#[derive(Debug)]
enum State {
    Idle,
    Tracking {
        track: Track,
        started_at: Instant,
        scrobbled: bool,
    },
}

/// Tracks scrobble eligibility across poll ticks.
///
/// The caller drives the machine by calling [`tick`] with whatever Apple Music
/// is currently playing (or `None` if stopped/paused). The machine returns a
/// list of [`Action`]s that should be executed against the Last.fm API.
pub struct ScrobbleStateMachine {
    state: State,
}

impl ScrobbleStateMachine {
    pub fn new() -> Self {
        Self { state: State::Idle }
    }

    /// Convenience wrapper for production use — passes `Instant::now()`.
    pub fn tick(&mut self, current: Option<&Track>) -> Vec<Action> {
        self.tick_at(current, Instant::now())
    }

    /// Core tick logic, accepting an explicit `now` so tests can control time.
    pub fn tick_at(&mut self, current: Option<&Track>, now: Instant) -> Vec<Action> {
        let mut actions = Vec::new();

        match &mut self.state {
            State::Idle => {
                if let Some(track) = current {
                    actions.push(Action::SendNowPlaying(track.clone()));
                    self.state = State::Tracking {
                        track: track.clone(),
                        started_at: now,
                        scrobbled: false,
                    };
                }
            }

            State::Tracking { track, started_at, scrobbled } => {
                match current {
                    // Player stopped or paused — go idle.
                    None => {
                        self.state = State::Idle;
                    }

                    // Different song started — send now-playing and reset.
                    Some(new_track) if !new_track.is_same_song(track) => {
                        actions.push(Action::SendNowPlaying(new_track.clone()));
                        self.state = State::Tracking {
                            track: new_track.clone(),
                            started_at: now,
                            scrobbled: false,
                        };
                    }

                    // Same song still playing — check scrobble threshold.
                    Some(_) => {
                        if !*scrobbled {
                            let elapsed = now.duration_since(*started_at).as_secs_f64();
                            let threshold = (track.duration_secs * MIN_SCROBBLE_FRACTION)
                                .max(MIN_SCROBBLE_SECS);
                            if elapsed >= threshold {
                                actions.push(Action::Scrobble(track.clone()));
                                *scrobbled = true;
                            }
                        }
                    }
                }
            }
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn track(title: &str) -> Track {
        Track {
            title: title.to_string(),
            artist: "Artist".to_string(),
            album: "Album".to_string(),
            duration_secs: 200.0,
        }
    }

    fn long_track(title: &str, duration_secs: f64) -> Track {
        Track {
            title: title.to_string(),
            artist: "Artist".to_string(),
            album: "Album".to_string(),
            duration_secs,
        }
    }

    fn has_now_playing(actions: &[Action]) -> bool {
        actions
            .iter()
            .any(|a| matches!(a, Action::SendNowPlaying(_)))
    }

    fn has_scrobble(actions: &[Action]) -> bool {
        actions.iter().any(|a| matches!(a, Action::Scrobble(_)))
    }

    #[test]
    fn idle_no_track_stays_idle() {
        let mut sm = ScrobbleStateMachine::new();
        let actions = sm.tick(None);
        assert!(actions.is_empty());
    }

    #[test]
    fn track_starts_sends_now_playing() {
        let mut sm = ScrobbleStateMachine::new();
        let t = track("Song A");
        let actions = sm.tick(Some(&t));
        assert!(has_now_playing(&actions));
        assert!(!has_scrobble(&actions));
    }

    #[test]
    fn no_scrobble_before_threshold() {
        let mut sm = ScrobbleStateMachine::new();
        let t = track("Song A");
        let start = Instant::now();

        sm.tick_at(Some(&t), start);

        // 15 seconds in — below both 30s and 50% of 200s (100s)
        let actions = sm.tick_at(Some(&t), start + Duration::from_secs(15));
        assert!(!has_scrobble(&actions));
    }

    #[test]
    fn scrobbles_after_50_percent_of_short_track() {
        let mut sm = ScrobbleStateMachine::new();
        // 60s track: threshold = max(30s, 30s) = 30s
        let t = long_track("Short Song", 60.0);
        let start = Instant::now();

        sm.tick_at(Some(&t), start);

        let actions = sm.tick_at(Some(&t), start + Duration::from_secs(31));
        assert!(has_scrobble(&actions));
    }

    #[test]
    fn scrobbles_at_50_percent_of_long_track() {
        let mut sm = ScrobbleStateMachine::new();
        // 300s track: threshold = max(30s, 150s) = 150s
        let t = long_track("Long Song", 300.0);
        let start = Instant::now();

        sm.tick_at(Some(&t), start);

        // 100s in — past 30s but not past 50%
        let actions = sm.tick_at(Some(&t), start + Duration::from_secs(100));
        assert!(!has_scrobble(&actions));

        // 151s in — past 50%
        let actions = sm.tick_at(Some(&t), start + Duration::from_secs(151));
        assert!(has_scrobble(&actions));
    }

    #[test]
    fn no_double_scrobble() {
        let mut sm = ScrobbleStateMachine::new();
        let t = track("Song A");
        let start = Instant::now();

        sm.tick_at(Some(&t), start);

        // First tick past threshold — should scrobble
        let actions = sm.tick_at(Some(&t), start + Duration::from_secs(120));
        assert!(has_scrobble(&actions));

        // Second tick past threshold — should NOT scrobble again
        let actions = sm.tick_at(Some(&t), start + Duration::from_secs(180));
        assert!(!has_scrobble(&actions));
    }

    #[test]
    fn track_change_sends_new_now_playing() {
        let mut sm = ScrobbleStateMachine::new();
        let t1 = track("Song A");
        let t2 = track("Song B");
        let start = Instant::now();

        sm.tick_at(Some(&t1), start);

        let actions = sm.tick_at(Some(&t2), start + Duration::from_secs(5));
        assert!(has_now_playing(&actions));
        assert!(!has_scrobble(&actions));
    }

    #[test]
    fn stop_then_resume_sends_now_playing_again() {
        let mut sm = ScrobbleStateMachine::new();
        let t = track("Song A");
        let start = Instant::now();

        sm.tick_at(Some(&t), start);
        sm.tick_at(None, start + Duration::from_secs(10)); // stopped

        let actions = sm.tick_at(Some(&t), start + Duration::from_secs(20));
        assert!(has_now_playing(&actions));
    }

    #[test]
    fn no_scrobble_if_track_stopped_before_threshold() {
        let mut sm = ScrobbleStateMachine::new();
        let t = track("Song A");
        let start = Instant::now();

        sm.tick_at(Some(&t), start);

        // Stop before threshold
        let actions = sm.tick_at(None, start + Duration::from_secs(10));
        assert!(!has_scrobble(&actions));
    }
}
