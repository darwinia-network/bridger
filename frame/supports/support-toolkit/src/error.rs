#![allow(missing_docs)]

use thiserror::Error as ThisError;

pub type TkResult<T> = Result<T, TkError>;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum TkError {
    #[error("[custom] {0}")]
    Custom(String),
}
