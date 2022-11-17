use config::ConfigError;
use serde_json::Error;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
#[allow(dead_code)]
pub enum BridgerError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("Subcommand error: {0}")]
    Subcommand(String),

    #[error("Subcommand error: {0}")]
    UnsupportExternal(String),

    #[error("Failed to call: `{0} {1}` {2}")]
    Process(String, String, String),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Hex error: {0}")]
    Hex(String),

    #[error("Hex error: {0}")]
    Migration(String),

    #[error("Wrap error: {0}")]
    Wrap(Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<serde_yaml::Error> for BridgerError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Config(format!("{:?}", e))
    }
}

impl From<serde_json::Error> for BridgerError {
    fn from(e: Error) -> Self {
        Self::Config(format!("{:?}", e))
    }
}

impl From<toml::ser::Error> for BridgerError {
    fn from(e: toml::ser::Error) -> Self {
        Self::Config(format!("{:?}", e))
    }
}

impl From<config::ConfigError> for BridgerError {
    fn from(e: ConfigError) -> Self {
        Self::Config(format!("{:?}", e))
    }
}
