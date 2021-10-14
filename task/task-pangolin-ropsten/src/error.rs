#![allow(missing_docs)]
//! Bridger Result
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("RuntimeUpdated")]
    RuntimeUpdated,

    #[error("Restart from jsonrpsee")]
    RestartFromJsonrpsee,
}

pub type Result<T> = anyhow::Result<T>;
