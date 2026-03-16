use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use md5::{Digest, Md5};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::config::Config;
use crate::credentials;
use crate::error::{AppError, Result};
use crate::player::Track;

const API_URL: &str = "https://ws.audioscrobbler.com/2.0/";

pub struct LastFmClient {
    client: Client,
    session_key: String,
}

impl LastFmClient {
    /// Build a client from config. Requires a session key (i.e. user has run `auth`).
    pub fn new(config: &Config) -> Result<Self> {
        let session_key = config.lastfm_session_key.clone().ok_or_else(|| {
            AppError::Config(
                "No session key in config. Run 'apple-to-last-fm auth' first.".to_string(),
            )
        })?;

        Ok(Self {
            client: Client::new(),
            session_key,
        })
    }

    pub fn now_playing(&self, track: &Track) -> Result<()> {
        let mut params = BTreeMap::new();
        params.insert("method".into(), "track.updateNowPlaying".into());
        params.insert("artist".into(), track.artist.clone());
        params.insert("track".into(), track.title.clone());
        params.insert("album".into(), track.album.clone());
        params.insert("sk".into(), self.session_key.clone());
        post_signed(&self.client, &mut params)?;
        Ok(())
    }

    pub fn scrobble(&self, track: &Track) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let mut params = BTreeMap::new();
        params.insert("method".into(), "track.scrobble".into());
        params.insert("artist".into(), track.artist.clone());
        params.insert("track".into(), track.title.clone());
        params.insert("album".into(), track.album.clone());
        params.insert("timestamp".into(), timestamp);
        params.insert("sk".into(), self.session_key.clone());
        post_signed(&self.client, &mut params)?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Session {
    pub name: String,
    pub key: String,
}

/// Authenticate with Last.fm using username and password.
/// Returns a session with name and key.
pub fn authenticate(username: &str, password: &str) -> Result<Session> {
    let mut params = BTreeMap::new();
    params.insert("method".into(), "auth.getMobileSession".into());
    params.insert("username".into(), username.into());
    params.insert("password".into(), password.into());

    let client = Client::new();
    let body = post_signed(&client, &mut params)?;

    #[derive(Deserialize)]
    struct Response {
        session: Session,
    }

    let parsed: Response = serde_json::from_str(&body)
        .map_err(|_| AppError::Scrobbler(format!("Authentication failed: {}", body)))?;

    Ok(parsed.session)
}

/// Sign params, POST to the Last.fm API, check for errors, and return the response body.
fn post_signed(client: &Client, params: &mut BTreeMap<String, String>) -> Result<String> {
    params.insert("api_key".into(), credentials::API_KEY.into());
    let api_sig = sign(params);
    params.insert("api_sig".into(), api_sig);
    params.insert("format".into(), "json".into());

    let resp = client
        .post(API_URL)
        .form(&params)
        .send()
        .map_err(|e| AppError::Scrobbler(e.to_string()))?;

    let status = resp.status();
    let body = resp
        .text()
        .map_err(|e| AppError::Scrobbler(e.to_string()))?;

    if !status.is_success() {
        return Err(AppError::Scrobbler(format!("HTTP {}: {}", status, body)));
    }

    // Check for Last.fm API-level errors (returned as 200 with error field)
    #[derive(Deserialize)]
    struct ApiError {
        #[serde(default)]
        error: u32,
        #[serde(default)]
        message: String,
    }

    if let Ok(err) = serde_json::from_str::<ApiError>(&body)
        && err.error != 0
    {
        return Err(AppError::Scrobbler(format!(
            "Last.fm error {}: {}",
            err.error, err.message
        )));
    }

    Ok(body)
}

fn sign(params: &BTreeMap<String, String>) -> String {
    let mut sig_input = String::new();
    for (k, v) in params {
        if k != "format" {
            sig_input.push_str(k);
            sig_input.push_str(v);
        }
    }
    sig_input.push_str(credentials::API_SECRET);
    format!("{:x}", Md5::digest(sig_input.as_bytes()))
}
