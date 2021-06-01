#![allow(missing_docs)]
//! Bridger Result
use crate::service::redeem::EthereumTransactionHash;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("The last redeemed block number is not set")]
	LastRedeemedFileNotExists,

	#[error("No ethereum start, run 'bridger set-start --block <redeem_scan_start> [--data-dir <data_dir>]' to set one")]
	NoEthereumStart,

	#[error("No darwinia scan start, run 'bridger set-darwinia-start --block <scan_start> [--data-dir <data_dir>]' to set one")]
	NoDarwiniaStart,

	#[error("No signer seed set for authority, please check your config.toml")]
	NoAuthoritySignerSeed,

	#[error("RuntimeUpdated")]
	RuntimeUpdated,

	#[error("ShadowInternalServerError: {0}")]
	ShadowInternalServerError(String),

	#[error("`bytes2hex` - FAILED: {0}")]
	Bytes2Hex(String),

	#[error("`hex2bytes` - FAILED: {0}")]
	Hex2Bytes(String),

	#[error("New http with URI {0} error: {1}")]
	NewHttpError(String, String),

	#[error("Restart from jsonrpsee")]
	RestartFromJsonrpsee,
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

	#[error("{0:?}'s block {1} is large than last confirmed block {2}")]
	RedeemingBlockLargeThanLastConfirmed(EthereumTransactionHash, u64, u64),

	#[error("{0:?} has already been redeemed")]
	TxRedeemed(EthereumTransactionHash),

	#[error("Mmr root for ethereum block {0} may be not filled yet, the error from shadow: {1}")]
	BlankEthereumMmrRoot(usize, String),
}

pub type Result<T> = anyhow::Result<T>;
