use thiserror::Error as ThisError;

pub type SubscanComponentResult<T> = Result<T, SubscanComponentError>;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum SubscanComponentError {
    #[error("Wrong response error: [{0}]: {1}")]
    WrongResponse(i32, String),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
