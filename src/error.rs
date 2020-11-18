#![allow(missing_docs)]
//! Bridger Result
use thiserror::Error as ThisError;
use jsonrpsee::transport::ws::WsNewDnsError;
use crate::service::redeem::EthereumTransactionHash;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    DeToml(#[from] toml::de::Error),

    #[error(transparent)]
    SerToml(#[from] toml::ser::Error),

    #[error(transparent)]
    Web3(#[from] web3::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Subxt(#[from] substrate_subxt::Error),

    #[error(transparent)]
    Primitives(#[from] primitives::result::Error),

    #[error(transparent)]
    Etc(#[from] etc::Error),

    #[error("{0}")]
    Bridger(String),

    #[error("Failed to connect to darwinia node {url}")]
    FailToConnectDarwinia {
        url: String,
        source: WsNewDnsError,
    },

    #[error("The last redeemed block number is not set")]
    LastRedeemedFileNotExists,

    #[error("No ethereum start, run 'bridger set-start --data-dir <data_dir> --block <redeem_scan_start>' to set one")]
    NoEthereumStart,

    #[error("Heartbeat>>> Scanning ethereum too fast from {0}, but the latest block number is {1}")]
    ScanningEthereumTooFast(u64, u64),

    #[error("The affirming target block {0} is less than the last_confirmed {0}")]
    AffirmingBlockLessThanLastConfirmed(u64, u64),

    #[error("The affirming target block {0} is in pending")]
    AffirmingBlockInPending(u64),

    #[error("The affirming target block {0} is in the relayer game")]
    AffirmingBlockInGame(u64),

    #[error("Shadow service failed to provide parcel for block {0}")]
    ParcelFromShadowIsEmpty(u64),

    #[error("{0:?}'s block {1} is large than last confirmed block {2}")]
    RedeemingBlockLargeThanLastConfirmed(EthereumTransactionHash, u64, u64),

    #[error("{0:?} has already been redeemed")]
    TxRedeemed(EthereumTransactionHash),
}

pub type Result<T> = std::result::Result<T, Error>;
