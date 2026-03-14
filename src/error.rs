use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // apple_music::Error doesn't implement std::error::Error, so we store it as a string.
    #[error("Apple Music error: {0}")]
    Player(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}

impl From<apple_music::Error> for AppError {
    fn from(e: apple_music::Error) -> Self {
        AppError::Player(format!("{:?}", e))
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
