use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
#[allow(dead_code)]
pub enum BridgerError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("Subcommand error: {0}")]
    Subcommand(String),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Wrap error: {0}")]
    Wrap(Box<dyn std::error::Error + Send + Sync>),
}
