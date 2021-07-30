#![allow(missing_docs)]
//! Bridger Result
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    // #[error("The last redeemed block number is not set")]
    // LastRedeemedFileNotExists,
    //
    // #[error("No ethereum start, run 'bridger set-start --block <redeem_scan_start> [--data-dir <data_dir>]' to set one")]
    // NoEthereumStart,
    //
    // #[error("No darwinia scan start, run 'bridger set-darwinia-start --block <scan_start> [--data-dir <data_dir>]' to set one")]
    // NoDarwiniaStart,
    //
    // #[error("No signer seed set for authority, please check your config.toml")]
    // NoAuthoritySignerSeed,
    #[error("RuntimeUpdated")]
    RuntimeUpdated,

    // #[error("ShadowInternalServerError: {0}")]
    // ShadowInternalServerError(String),

    // #[error("`bytes2hex` - FAILED: {0}")]
    // Bytes2Hex(String),
    #[error("`hex2bytes` - FAILED: {0}")]
    Hex2Bytes(String),

    // #[error("New http with URI {0} error: {1}")]
    // NewHttpError(String, String),
    #[error("Restart from jsonrpsee")]
    RestartFromJsonrpsee,
}

pub type Result<T> = anyhow::Result<T>;
