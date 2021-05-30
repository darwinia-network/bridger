use substrate_subxt::{
	events::Raw,
	sp_core::storage::{StorageData, StorageKey},
	sp_core::{twox_128, Bytes, H256},
	sp_runtime::traits::Header as TraitHeader,
	BlockNumber, Client, ClientBuilder, Runtime,
};

use crate::{account::DarwiniaAccount, DarwiniaEvents, EventInfo};

use crate::error::{Error, Result};
use jsonrpsee_types::jsonrpc::{to_value as to_json_value, Params};

use crate::rpc::*;
use primitives::frame::bridge::relay_authorities::EthereumRelayAuthorities;
use primitives::frame::ethereum::runtime_ext::RuntimeExt;
use primitives::frame::sudo::KeyStoreExt;
use primitives::frame::sudo::Sudo;
use substrate_subxt::sp_runtime::traits::Verify;
use substrate_subxt::system::System;

pub struct Darwinia<R: Runtime> {
	/// client
	pub subxt: Client<R>,
	/// Event Parser
	pub event: DarwiniaEvents<R>,
}

impl<R: Runtime> Clone for Darwinia<R> {
	fn clone(&self) -> Self {
		Self {
			subxt: self.subxt.clone(),
			event: self.event.clone(),
		}
	}
}

impl<R: Runtime> Darwinia<R> {
	pub async fn new(url: &str) -> Result<Darwinia<R>> {
		let client = ClientBuilder::<R>::new()
			.set_url(url)
			.skip_type_sizes_check()
			.build()
			.await?;
		let event = DarwiniaEvents::new(client.clone());

		Ok(Self {
			subxt: client,
			event,
		})
	}

	/// block number to hash
	pub async fn block_number2hash(&self, block_number: Option<u32>) -> Result<Option<<R as System>::Hash>> {
		let block_number = block_number.map(|n| n.into());
		Ok(self.subxt.block_hash(block_number).await?)
	}

	/// is_sudo_key
	pub async fn is_sudo_key(
		&self,
		block_number: Option<u32>,
		account: &DarwiniaAccount<R>,
	) -> Result<bool>
	where
		R: System<Hash = H256> + Sudo,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
		<R::Signature as Verify>::Signer: From<sp_keyring::sr25519::sr25519::Public>,
	{
		let block_hash = self.block_number2hash(block_number).await?;
		let sudo = self.subxt.key(block_hash).await?;
		Ok(&sudo == account.real())
	}

	/// role
	pub async fn account_role(&self, account: &DarwiniaAccount<R>) -> Result<Vec<String>>
	where
		R: System<Hash = H256> + Sudo,
		R::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
		<R::Signature as Verify>::Signer: From<sp_keyring::sr25519::sr25519::Public>,
	{
		let mut roles = vec!["Normal".to_string()];
		if self.is_sudo_key(None, account).await? {
			roles.push("Sudo".to_string());
		}
		Ok(roles)
	}

	/// Check if should redeem
	pub async fn verified(&self, block_hash: web3::types::H256, tx_index: u64) -> Result<bool>
	where
		R: RuntimeExt,
	{
		Ok(R::verify(&self.subxt, block_hash.to_fixed_bytes(), tx_index).await?)
	}

	/// get mmr root of darwinia
	pub async fn header_mmr(
		&self,
		block_number_of_member_leaf: u64,
		block_number_of_last_leaf: u64,
		hash: H256,
	) -> Result<Option<HeaderMMR>> {
		let params = Params::Array(vec![
			to_json_value(block_number_of_member_leaf)?,
			to_json_value(block_number_of_last_leaf)?,
		]);
		let result: HeaderMMRRpc = self
			.subxt
			.rpc
			.client
			.request("headerMMR_genProof", params)
			.await?;
		let header_mmr: Option<HeaderMMR> = result.into();
		if let Some(mut header_proof) = header_mmr {
			header_proof.block = block_number_of_member_leaf;
			header_proof.hash = hash;
			return Ok(Some(header_proof));
		}
		Ok(None)
	}

	/// get_storage_data
	pub async fn get_storage_data(
		&self,
		module_name: &str,
		storage_name: &str,
		header_hash: <R as System>::Hash,
	) -> Result<StorageData> {
		let mut storage_key = twox_128(module_name.as_bytes()).to_vec();
		storage_key.extend(twox_128(storage_name.as_bytes()).to_vec());

		let keys = vec![StorageKey(storage_key)];

		let change_sets = self
			.subxt
			.query_storage(keys, header_hash, Some(header_hash))
			.await?;
		for change_set in change_sets {
			for (_key, data) in change_set.changes {
				if let Some(data) = data {
					return Ok(data);
				}
			}
		}

		Err(Error::NoStorageDataFound(
			module_name.to_string(),
			storage_name.to_string(),
		))
	}

	/// get runtime version
	pub async fn runtime_version(&self) -> Result<String> {
		let version = self.subxt.rpc.runtime_version(None).await?;
		Ok(version.spec_name.to_string())
	}

	/// get events from a special block
	pub async fn get_events_from_block_hash(&self, hash: <R as System>::Hash) -> Result<Vec<EventInfo<R>>>
	where
		R: EthereumRelayAuthorities,
	{
		let storage_data = self.get_storage_data("System", "Events", hash).await?;

		let raw_events = self.event.decoder.decode_events(&mut &storage_data.0[..])?;
		let mut result = Vec::new();
		for (_, raw) in raw_events {
			match raw {
				Raw::Event(event) => {
					let module = event.module.as_str();
					let variant = event.variant.as_str();
					let event_data = event.data;
					let event = self.event.parse_event(module, variant, event_data);
					if let EventInfo::Invalid(info) = event {
						if module != "System" {
							trace!(">> Event - {}", info);
						}
					} else {
						result.push(event);
					}
				}
				Raw::Error(err) => {
					error!("Error found in raw events: {:#?}", err);
				}
			}
		}
		Ok(result)
	}

	/// get events from a special block
	pub async fn get_events_from_block_number(&self, block: u32) -> Result<Vec<EventInfo<R>>>
	where
		R: EthereumRelayAuthorities + System<Hash = H256>,
	{
		let blockno = BlockNumber::from(block);
		match self.subxt.block_hash(Some(blockno)).await? {
			Some(hash) => return self.get_events_from_block_hash(hash).await,
			None => {
				info!("error");
			}
		}
		Ok(vec![])
	}

	/// get mmr root
	pub async fn get_mmr_root(&self, leaf_index: u32) -> Result<H256>
	where
		R: System<BlockNumber = u32>,
	{
		let block_number = leaf_index + 1u32;

		let block_hash = self
			.subxt
			.block_hash(Some(BlockNumber::from(block_number)))
			.await?;
		let header = self.subxt.header(block_hash).await?;

		let mmr_root = if let Some(header) = header {
			// get digest_item from header
			let log = header
				.digest()
				.logs()
				.iter()
				.find(|&x| x.as_other().is_some());
			if let Some(digest_item) = log {
				// get mmr_root from log
				let parent_mmr_root = digest_item.as_other().unwrap().to_vec();
				let parent_mmr_root = &parent_mmr_root[4..];
				if parent_mmr_root.len() != 32 {
					return Err(Error::WrongMmrRootInDarwiniaHeader(
						array_bytes::bytes2hex("", &parent_mmr_root),
						block_number,
					));
				}

				H256::from_slice(parent_mmr_root)
			} else {
				return Err(Error::NoMmrRootInDarwiniaHeader(block_number));
			}
		} else {
			return Err(Error::FailedToFetchDarwiniaHeader(block_number));
		};
		Ok(mmr_root)
	}

	/// events proof
	pub async fn get_event_proof(
		&self,
		storage_key: Vec<u8>,
		block_hash: H256,
	) -> Result<Vec<Bytes>>
	where
		R: System<Hash = H256>,
	{
		let keys = vec![StorageKey(storage_key)];
		Ok(self.subxt.read_proof(keys, Some(block_hash)).await?.proof)
	}

	/// get block header by number
	pub async fn get_block_by_number(&self, number: u32) -> Result<<R as System>::Header>
	{
		match self
			.subxt
			.block_hash(Some(BlockNumber::from(number)))
			.await?
		{
			Some(block_hash) => match self.subxt.header(Some(block_hash)).await? {
				Some(header) => Ok(header),
				None => Err(Error::Other("get header return nil".to_string())),
			},
			None => Err(Error::Other("get block hash failed".to_string())),
		}
	}

	/// finalized_head
	pub async fn finalized_head(&self) -> Result<<R as System>::Hash>
	{
		let hash = self.subxt.finalized_head().await?;
		Ok(hash)
	}

	/// get block by hash
	pub async fn get_block_number_by_hash(
		&self,
		block_hash: <R as System>::Hash,
	) -> Result<Option<<R as System>::BlockNumber>>
	{
		let block = self.subxt.block(Some(block_hash)).await?;
		if let Some(block) = block {
			return Ok(Some(*block.block.header.number()));
		}
		Ok(None)
	}
}
