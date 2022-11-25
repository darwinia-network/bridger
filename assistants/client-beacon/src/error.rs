use std::string::FromUtf8Error;

use hex::FromHexError;
use reqwest::Error as ReqwestError;
use thiserror::Error as ThisError;

pub type BeaconApiResult<T> = Result<T, BeaconApiError>;

#[derive(ThisError, Debug)]
pub enum BeaconApiError {
    #[error("Custom: {0}")]
    Custom(String),
    #[error("Failed to decode from {0} into {1}")]
    DecodeError(String, String),
    #[error(transparent)]
    FromHexError(#[from] FromHexError),
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error(transparent)]
    ApiError(#[from] ReqwestError),
    #[error(
        "Beacon api error: Status code: {status_code:?}, Error: {error:?}, Message: {message:?}"
    )]
    BeaconApiError {
        status_code: u64,
        error: String,
        message: String,
    },
}
