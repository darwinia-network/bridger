//! Verifier

use core::marker::PhantomData;
use substrate_subxt::{Client, Runtime};

use async_trait::async_trait;

use crate::frame::ethereum::{
    backing::{
        EthereumBacking,
        VerifiedProofStoreExt,
    },
    issuing::{
        EthereumIssuing,
        VerifiedIssuingProofStoreExt
    },
};

use crate::result::Result;

/// verifiy backing or issuing storage proof existence
#[async_trait]
pub trait Verifier: Runtime {
	/// the verify handler
	type VerifierHandler: DarwiniaVerifier<Self>;

	/// verify
	async fn verify(client: &Client<Self>, block_hash: [u8; 32], tx_index: u64) -> Result<bool> {
		Self::VerifierHandler::verify(client, block_hash, tx_index).await
	}
}

/// darwinia verifier
#[async_trait]
pub trait DarwiniaVerifier<R: Runtime + Verifier> {
	/// verify
	async fn verify(client: &Client<R>, block_hash: [u8; 32], tx_index: u64) -> Result<bool>;
}

/// verify for darwinia mainnet
// backing
pub struct DarwiniaMainnetVerifier<R: Runtime> {
	pub _runtime: PhantomData<R>,
}

#[async_trait]
impl<R: Runtime + EthereumBacking + Verifier> DarwiniaVerifier<R> for DarwiniaMainnetVerifier<R> {
	/// verify
	async fn verify(client: &Client<R>, block_hash: [u8; 32], tx_index: u64) -> Result<bool> {
		println!("mainnet verify");
		Ok(client
			.verified_proof((block_hash, tx_index), None)
			.await?
			.unwrap_or(false))
	}
}

/// verifier for darwinia pangolin
// issuing & backing
pub struct DarwiniaPangolinVerifier<R: Runtime> {
	pub _runtime: PhantomData<R>,
}

#[async_trait]
impl<R: Runtime + EthereumBacking + EthereumIssuing + Verifier> DarwiniaVerifier<R> for DarwiniaPangolinVerifier<R> {
	/// verify
	async fn verify(client: &Client<R>, block_hash: [u8; 32], tx_index: u64) -> Result<bool> {
		Ok(client
			.verified_issuing_proof((block_hash, tx_index), None)
			.await?
			.unwrap_or(false) ||
			client
			.verified_proof((block_hash, tx_index), None)
			.await?
			.unwrap_or(false))
	}
}
