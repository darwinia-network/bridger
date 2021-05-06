use crate::{Darwinia, HeaderMMR, EcdsaMessage, EcdsaSignature};

use super::Account;

use crate::error::Result;

use codec::Encode;

use substrate_subxt::sp_core::H256;

use primitives::{
	chain::{ethereum::EthereumReceiptProofThing, proxy_type::ProxyType},
	frame::{
		bridge::relay_authorities::{
			AuthoritiesStoreExt, AuthoritiesToSignStoreExt, MmrRootsToSignStoreExt,
			NextTermStoreExt, SubmitSignedAuthorities, SubmitSignedAuthoritiesCallExt,
			SubmitSignedMmrRoot, SubmitSignedMmrRootCallExt,
		},
		ethereum::backing::{SyncAuthoritiesChange, SyncAuthoritiesChangeCallExt},
		proxy::ProxyCallExt,
	},
};

use core::marker::PhantomData;
use substrate_subxt::{
	Runtime, SignedExtra, SignedExtension, system::System
};
use crate::types::EcdsaAddress;
use primitives::frame::{
	proxy::Proxy,
	sudo::Sudo,
	ethereum::{
		backing::EthereumBacking,
		issuing::EthereumIssuing,
	},
	bridge::relay_authorities::EthereumRelayAuthorities
};
use substrate_subxt::sp_runtime::traits::{UniqueSaturatedInto, Verify};


#[derive(Encode)]
struct _S<_1, _2, _3, _4>
where
	_1: Encode,
	_2: Encode,
	_3: Encode,
	_4: Encode,
{
	_1: _1, // spec name
	_2: _2, // op code, mmr root: 0x479fbdf9, next authorities: 0xb4bcf497
	#[codec(compact)]
	_3: _3, // block_number or term
	_4: _4, // mmr_root or next authorities
}

/// Dawrinia API
#[derive(Clone)]
pub struct Darwinia2Ethereum<R: Runtime + EthereumRelayAuthorities> {
	/// darwinia client
	pub darwinia: Darwinia<R>,
}

impl<R> Darwinia2Ethereum<R>
where R: Runtime + Proxy + Sudo + EthereumBacking + EthereumIssuing + EthereumRelayAuthorities,
	<<R::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: std::marker::Send,
	<<R::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Sync,
	<R as System>::Address: From<<R as System>::AccountId>,
	<R as Runtime>::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	  <<R as substrate_subxt::Runtime>::Signature as Verify>::Signer: From<sp_keyring::sr25519::sr25519::Public>
{
	pub fn new(darwinia: Darwinia<R>) -> Self {
		Self { darwinia }
	}

	/// header mmr proof
	pub async fn get_headermmr_genproof(
		&self,
		member_leaf: u64,
		last_leaf: u64,
		hash: H256,
	) -> Result<Option<HeaderMMR>> {
		return self.darwinia.header_mmr(member_leaf, last_leaf, hash).await;
	}

	/// construct mmr root message
	pub fn construct_mmr_root_message(
		spec_name: String,
		block_number: u32,
		mmr_root: H256,
	) -> Vec<u8> {
		let op_code: [u8; 4] = [71, 159, 189, 249];
		debug!(
			"Infos to construct mmr_root message: {}, {}, {}, {:?}",
			spec_name,
			array_bytes::bytes2hex("", &op_code),
			block_number,
			mmr_root
		);
		// scale encode & sign
		let message = _S {
			_1: spec_name,
			_2: op_code,
			_3: block_number,
			_4: mmr_root,
		};
		let encoded: &[u8] = &message.encode();
		encoded.to_vec()
	}

	/// construct_message
	pub fn construct_authorities_message(
		spec_name: String,
		term: u32,
		next_authorities: Vec<EcdsaAddress>,
	) -> Vec<u8> {
		let op_code: [u8; 4] = [180, 188, 244, 151];
		debug!(
			"Infos to construct eth authorities message: {}, {}, {}, {:?}",
			spec_name,
			array_bytes::bytes2hex("", &op_code),
			term,
			next_authorities
				.iter()
				.map(|a| array_bytes::bytes2hex("", &a))
				.collect::<Vec<_>>()
				.join(", ")
		);
		// scale encode & sign
		let message = _S {
			_1: spec_name,
			_2: op_code,
			_3: term,
			_4: next_authorities,
		};
		let encoded: &[u8] = &message.encode();
		encoded.to_vec()
	}

	/// get_current_term
	pub async fn get_current_authority_term(&self) -> Result<u32> {
		Ok(self.darwinia.subxt.next_term(None).await?)
	}

	// use account
	/// sync authorities change from ethereum to darwinia
	pub async fn sync_authorities_change(
		&self,
		account: &Account<R>,
		proof: EthereumReceiptProofThing,
		proxy_type: <R as Proxy>::ProxyType,
	) -> Result<<R as System>::Hash>
	{
		match &account.0.real {
			Some(real) => {
				let call = SyncAuthoritiesChange {
					_runtime: PhantomData::default(),
					proof,
				};

				let ex = self.darwinia.subxt.encode(call).unwrap();
				Ok(self
					.darwinia
					.subxt
					.proxy(
						&account.0.signer,
						real.clone(),
						Some(proxy_type),
						&ex,
					)
					.await?)
			}
			None => Ok(self
				.darwinia
				.subxt
				.sync_authorities_change(&account.0.signer, proof)
				.await?),
		}
	}

	/// submit_signed_authorities
	pub async fn ecdsa_sign_and_submit_signed_authorities(
		&self,
		account: &Account<R>,
		message: EcdsaMessage,
		proxy_type: <R as Proxy>::ProxyType,
	) -> Result<<R as System>::Hash>
	where R: EthereumRelayAuthorities<RelayAuthoritySignature = EcdsaSignature>
	{
		// TODO: check
		// 	.sender
		// 	.need_to_sign_authorities(decoded.message, block)
		// 	.await?
		let signature = account.ecdsa_sign(&message)?;
		match &account.0.real {
			// proxy
			Some(real) => {
				trace!("Proxyed ecdsa sign and submit authorities to darwinia");
				let submit_signed_authorities = SubmitSignedAuthorities { signature };

				let ex = self
					.darwinia
					.subxt
					.encode(submit_signed_authorities)
					.unwrap();
				let tx_hash = self
					.darwinia
					.subxt
					.proxy(
						&account.0.signer,
						real.clone(),
						Some(proxy_type),
						&ex,
					)
					.await?;
				Ok(tx_hash)
			}
			None => {
				trace!("Ecdsa sign and submit authorities to darwinia");
				let tx_hash = self
					.darwinia
					.subxt
					.submit_signed_authorities(&account.0.signer, signature)
					.await?;
				Ok(tx_hash)
			}
		}
	}

	/// submit_signed_mmr_root
	pub async fn ecdsa_sign_and_submit_signed_mmr_root(
		&self,
		account: &Account<R>,
		spec_name: String,
		block_number: <R as System>::BlockNumber,
		proxy_type: <R as Proxy>::ProxyType,
	) -> Result<<R as System>::Hash>
	where <<R::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned: Sync,
	R: EthereumRelayAuthorities<RelayAuthoritySignature = EcdsaSignature>
	{
		// get mmr root from darwinia
		let leaf_index = block_number;
		let mmr_root = self.darwinia.get_mmr_root(leaf_index).await?;

		let bn = UniqueSaturatedInto::<u32>::unique_saturated_into(block_number);
		let encoded =
			Darwinia2Ethereum::<R>::construct_mmr_root_message(spec_name, bn, mmr_root);
		let hash = web3::signing::keccak256(&encoded);
		let signature = account.ecdsa_sign(&hash)?;

		match &account.0.real {
			// proxy
			Some(real) => {
				trace!(
					"Proxyed ecdsa sign and submit mmr_root to darwinia, block_number: {}",
					block_number
				);
				let submit_signed_mmr_root = SubmitSignedMmrRoot {
					block_number,
					signature,
				};

				let ex = self.darwinia.subxt.encode(submit_signed_mmr_root).unwrap();
				let tx_hash = self
					.darwinia
					.subxt
					.proxy(
						&account.0.signer,
						real.clone(),
						Some(proxy_type),
						&ex,
					)
					.await?;
				Ok(tx_hash)
			}
			None => {
				trace!(
					"Ecdsa sign and submit mmr_root to darwinia, block_number: {}",
					block_number
				);
				let tx_hash = self
					.darwinia
					.subxt
					.submit_signed_mmr_root(&account.0.signer, block_number, signature)
					.await?;
				Ok(tx_hash)
			}
		}
	}

	/// is authority
	pub async fn is_authority(&self, block_number: Option<u32>, account: &Account<R>) -> Result<bool> {
		// #![allow(clippy::needless_collect)]
		// let block_hash = self.darwinia.block_number2hash(block_number).await?;
		// let authorities = self
		// 	.darwinia
		// 	.subxt
		// 	.authorities(block_hash)
		// 	.await?
		// 	.iter()
		// 	.map(|a| a.account_id.clone())
		// 	.collect::<Vec<_>>();
		// Ok(authorities.contains(account.0.real()))
		Ok(true)
	}

	/// need_to_sign_authorities
	pub async fn need_to_sign_authorities(
		&self,
		block_number: Option<u32>,
		account: &Account<R>,
		message: <R as EthereumRelayAuthorities>::RelayAuthorityMessage,
	) -> Result<bool> {
		let block_hash = self.darwinia.block_number2hash(block_number).await?;
		let ret = self.darwinia.subxt.authorities_to_sign(block_hash).await?;
		match ret {
			None => Ok(false),
			Some(r) => {
				if r.0 == message {
					let account_id = account
						.0
						.real
						.clone()
						.unwrap_or_else(|| account.0.account_id.clone());
					let includes = r.1.iter().any(|a| a.0 == account_id);
					Ok(!includes)
				} else {
					Ok(false)
				}
			}
		}
	}

	/// need_to_mmr_root_of
	pub async fn need_to_sign_mmr_root_of(
		&self,
		account: &Account<R>,
		block_number: <R as System>::BlockNumber,
		exec_block_number: Option<u32>,
	) -> Result<bool> {
		let exec_block_hash = self.darwinia.block_number2hash(exec_block_number).await?;
		match self
			.darwinia
			.subxt
			.mmr_roots_to_sign(block_number, exec_block_hash)
			.await?
		{
			None => Ok(false),
			Some(items) => {
				let account_id = account
					.0
					.real
					.clone()
					.unwrap_or_else(|| account.0.account_id.clone());
				let includes = items.iter().any(|a| a.0 == account_id);
				Ok(!includes)
			}
		}
	}

	/// Print Detail
	pub async fn account_detail(&self, block_number: Option<u32>, account: &Account<R>) -> Result<()> {
		info!("🧔 darwinia => ethereum account");
		let mut roles = self.darwinia.account_role(&account.0).await?;
		if self.is_authority(block_number, &account).await? {
			roles.push("Authority".to_string());
		}
		match &account.0.real {
			None => {
				info!("🧔 Relayer({:?}): 0x{:?}", roles, account.0.account_id);
			}
			Some(real_account_id) => {
				info!("🧔 Proxy Relayer: 0x{:?}", account.0.account_id);
				info!("👴 Real Account({:?}): 0x{:?}", roles, real_account_id);
			}
		}
		Ok(())
	}
}
