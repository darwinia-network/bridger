#![allow(missing_docs)]

use bridge_traits::error::StandardError;
use thiserror::Error as ThisError;

pub type ComponentEthereumResult<T> = core::result::Result<T, ComponentEthereumError>;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum ComponentEthereumError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Standard(#[from] StandardError),

    #[error(transparent)]
    ExternalAnyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Biz(#[from] BizError),

    #[error("Fail to get ethereum header of block {1}: {0}")]
    FailToGetEthereumHeader(String, u64),
}

#[derive(ThisError, Debug)]
pub enum BizError {
    #[error("{0}")]
    Bridger(String),

    #[error("Heartbeat>>> Scanning ethereum too fast from {0}, the latest block number is {1}")]
    ScanningEthereumTooFast(u64, u64),

    #[error("The affirming target block {0} is less than the last_confirmed {1}")]
    AffirmingBlockLessThanLastConfirmed(u64, u64),

    #[error("The affirming target block {0} is in pending")]
    AffirmingBlockInPending(u64),

    #[error("The affirming target block {0} is in the relayer game")]
    AffirmingBlockInGame(u64),

    #[error("Shadow service failed to provide parcel for block {0}")]
    ParcelFromShadowIsEmpty(u64),

    // todo: temporarily closed
    // #[error("{0:?}'s block {1} is large than last confirmed block {2}")]
    // RedeemingBlockLargeThanLastConfirmed(EthereumTransactionHash, u64, u64),
    //
    // #[error("{0:?} has already been redeemed")]
    // TxRedeemed(EthereumTransactionHash),
    #[error("Mmr root for ethereum block {0} may be not filled yet, the error from shadow: {1}")]
    BlankEthereumMmrRoot(usize, String),
}
