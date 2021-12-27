use thiserror::Error as ThisError;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum SubscanComponentError {
    #[error("Wrong response error: [{0}]: {1}")]
    WrongResponse(i32, String),
}
