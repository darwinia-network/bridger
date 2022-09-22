use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::result::Result as StdResult;
use thiserror::Error as ThisError;
use web3::Error as Web3Error;

pub type Result<T> = StdResult<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Etherscan api error: {0}")]
    Etherscan(String),
    #[error(transparent)]
    Web3(#[from] Web3Error),
    #[error(transparent)]
    Reqwest(#[from] ReqwestError),
    #[error(transparent)]
    SerdeJson(#[from] SerdeJsonError),
}
