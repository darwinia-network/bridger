use thiserror::Error as ThisError;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum EthereumComponentError {
    #[error("You have no ethereum private key configured.")]
    NoPrivateKey,
    #[error("You have no beneficiary configured.")]
    NoBeneficiaryAccount,
    #[error("Wrong {0} key ({1}), failed parse to bytes.")]
    WrongKey(String, String),
    #[error("The block [{0}] not found")]
    BlockNotFound(u64),
}

#[derive(ThisError, Debug)]
pub enum BizError {
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

    #[error("Mmr root for ethereum block {0} may be not filled yet, the error from shadow: {1}")]
    BlankEthereumMmrRoot(usize, String),
}
