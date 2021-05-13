//! RuntimeExt

use core::marker::PhantomData;
use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::{
    Client, Runtime, system::System, PairSigner, SignedExtension, SignedExtra
};

use async_trait::async_trait;

use crate::frame::{
    ethereum::{
        backing::{
            EthereumBacking,
            VerifiedProofStoreExt,
        },
        issuing::{
            EthereumIssuing,
            RedeemErc20,
            RedeemErc20CallExt,
            RegisterErc20,
            RegisterErc20CallExt,
            VerifiedIssuingProofStoreExt
        },
    },
    proxy::{ Proxy, ProxyCallExt },
};

use crate::{
	chain::{
		proxy_type::ProxyType,
		ethereum::EthereumReceiptProofThing,
    }
};

use crate::result::{
    Result,
    Error,
};

/// some extra option for special runtime
#[async_trait]
pub trait RuntimeExt: Runtime {
	/// the verify handler
	type VerifierHandler: DarwiniaVerifier<Self>;
    
    /// Issuing Handler
    /// redeem erc20
    type RedeemErc20Handler: DarwiniaErc20Redeemer<Self>;
    /// register erc20
    type RegisterErc20Handler: DarwiniaErc20Register<Self>;

	/// verify
	async fn verify(client: &Client<Self>, block_hash: [u8; 32], tx_index: u64) -> Result<bool> {
		Self::VerifierHandler::verify(client, block_hash, tx_index).await
	}

    /// redeem erc20
    async fn redeem_erc20(
        client: &Client<Self>, 
        account: Option<<Self as System>::AccountId>, 
        signer: PairSigner<Self, Pair>, 
        proof: EthereumReceiptProofThing) -> Result<<Self as System>::Hash> 
        where
        <<<Self as Runtime>::Extra as SignedExtra<Self>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync,
		Self::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
        <Self as System>::Address: From<<Self as System>::AccountId>
    {
            Self::RedeemErc20Handler::redeem_erc20(client, account, signer, proof).await
    }

    /// register erc20
    async fn register_erc20(
        client: &Client<Self>, 
        account: Option<<Self as System>::AccountId>, 
        signer: PairSigner<Self, Pair>, 
        proof: EthereumReceiptProofThing) -> Result<<Self as System>::Hash> 
        where
        <<<Self as Runtime>::Extra as SignedExtra<Self>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync,
		Self::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
        <Self as System>::Address: From<<Self as System>::AccountId>
    {
            Self::RegisterErc20Handler::register_erc20(client, account, signer, proof).await
    }

}

/// darwinia verifier
#[async_trait]
pub trait DarwiniaVerifier<R: Runtime + RuntimeExt> {
	/// verify
	async fn verify(client: &Client<R>, block_hash: [u8; 32], tx_index: u64) -> Result<bool>;
}

/// verifier of darwinia mainnet
// backing
pub struct DarwiniaMainnetVerifier<R: Runtime> {
	pub _runtime: PhantomData<R>,
}

#[async_trait]
impl<R: Runtime + EthereumBacking + RuntimeExt> DarwiniaVerifier<R> for DarwiniaMainnetVerifier<R> {
	/// verify
	async fn verify(client: &Client<R>, block_hash: [u8; 32], tx_index: u64) -> Result<bool> {
		println!("mainnet verify");
		Ok(client
			.verified_proof((block_hash, tx_index), None)
			.await?
			.unwrap_or(false))
	}
}

/// verifier of darwinia pangolin
// issuing & backing
pub struct DarwiniaPangolinVerifier<R: Runtime> {
	pub _runtime: PhantomData<R>,
}

#[async_trait]
impl<R: Runtime + EthereumBacking + EthereumIssuing + RuntimeExt> DarwiniaVerifier<R> for DarwiniaPangolinVerifier<R> {
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

/// only supported for the runtime include issuing module
/// darwinia redeem erc20 trait
#[async_trait]
pub trait DarwiniaErc20Redeemer<R: Runtime + RuntimeExt> {
	/// verify
	async fn redeem_erc20(
        client: &Client<R>, 
        account: Option<<R as System>::AccountId>, 
        signer: PairSigner<R, Pair>, 
        proof: EthereumReceiptProofThing) -> Result<<R as System>::Hash>
        where 
        <<<R as Runtime>::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
        <R as System>::Address: From<<R as System>::AccountId>;

}

/// redeemErc20 for darwinia mainnet
// issuing & backing
pub struct DarwiniaMainnetErc20Redeemer<R: Runtime> {
	pub _runtime: PhantomData<R>,
}

#[async_trait]
impl<R: Runtime + RuntimeExt> DarwiniaErc20Redeemer<R> for DarwiniaMainnetErc20Redeemer<R> {
    /// redeem erc20
    async fn redeem_erc20 (
        _client: &Client<R>, 
        _account: Option<<R as System>::AccountId>, 
        _signer: PairSigner<R, Pair>, 
        _proof: EthereumReceiptProofThing) -> Result<<R as System>::Hash> {
        return Err(Error::DonotSupport("redeemErc20".into()));
    }
}

/// redeemErc20 for darwinia pangolin
// issuing & backing
pub struct DarwiniaPangolinErc20Redeemer<R: Runtime> {
	pub _runtime: PhantomData<R>,
}

#[async_trait]
impl<R: Runtime + EthereumIssuing + RuntimeExt + Proxy<ProxyType = ProxyType>> DarwiniaErc20Redeemer<R> for DarwiniaPangolinErc20Redeemer<R> {
    /// redeem erc20
    async fn redeem_erc20 (
        client: &Client<R>, 
        account: Option<<R as System>::AccountId>, 
        signer: PairSigner<R, Pair>, 
        proof: EthereumReceiptProofThing) -> Result<<R as System>::Hash> 
        where 
        <<<R as Runtime>::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
        <R as System>::Address: From<<R as System>::AccountId>
    {
        match &account {
			Some(real) => {
				let call = RedeemErc20 {
					_runtime: PhantomData::default(),
					proof,
				};

				let ex = client.encode(call).unwrap();
				Ok(client
					.proxy(
						&signer,
						real.clone(),
						Some(ProxyType::EthereumBridge),
						&ex,
					)
					.await?)
			}
			None => Ok(client
                       .redeem_erc20(&signer, proof)
                       .await?),
		}
    }
}

/// darwinia register erc20 trait
#[async_trait]
pub trait DarwiniaErc20Register<R: Runtime + RuntimeExt> {
	/// register
	async fn register_erc20(
        client: &Client<R>, 
        account: Option<<R as System>::AccountId>, 
        signer: PairSigner<R, Pair>, 
        proof: EthereumReceiptProofThing) -> Result<<R as System>::Hash>
        where 
        <<<R as Runtime>::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
        <R as System>::Address: From<<R as System>::AccountId>;
}

/// registerErc20 for darwinia mainnet
pub struct DarwiniaMainnetErc20Register<R: Runtime> {
	pub _runtime: PhantomData<R>,
}

#[async_trait]
impl<R: Runtime + RuntimeExt> DarwiniaErc20Register<R> for DarwiniaMainnetErc20Register<R> {
    /// register erc20
    async fn register_erc20 (
        _client: &Client<R>, 
        _account: Option<<R as System>::AccountId>, 
        _signer: PairSigner<R, Pair>, 
        _proof: EthereumReceiptProofThing) -> Result<<R as System>::Hash> {
        return Err(Error::DonotSupport("registerErc20".into()));
    }
}

/// registerErc20 for darwinia pangolin
pub struct DarwiniaPangolinErc20Register<R: Runtime> {
	pub _runtime: PhantomData<R>,
}

#[async_trait]
impl<R: Runtime + EthereumIssuing + RuntimeExt + Proxy<ProxyType = ProxyType>> DarwiniaErc20Register<R> for DarwiniaPangolinErc20Register<R> {
    /// redeem erc20
    async fn register_erc20 (
        client: &Client<R>, 
        account: Option<<R as System>::AccountId>, 
        signer: PairSigner<R, Pair>, 
        proof: EthereumReceiptProofThing) -> Result<<R as System>::Hash> 
        where 
        <<<R as Runtime>::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Send + Sync,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
        <R as System>::Address: From<<R as System>::AccountId>
    {
        match &account {
			Some(real) => {
				let call = RegisterErc20 {
					_runtime: PhantomData::default(),
					proof,
				};

				let ex = client.encode(call).unwrap();
				Ok(client
					.proxy(
						&signer,
						real.clone(),
						Some(ProxyType::EthereumBridge),
						&ex,
					)
					.await?)
			}
			None => Ok(client
                       .register_erc20(&signer, proof)
                       .await?),
		}
    }
}

