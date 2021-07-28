use jsonrpsee_http_client::to_json_value;
use substrate_subxt::{
    events::Raw,
    sp_core::storage::{StorageData, StorageKey},
    sp_core::{twox_128, Bytes, H256},
    sp_runtime::generic::Header,
    sp_runtime::traits::{BlakeTwo256, Header as TraitHeader},
    BlockNumber, Client, ClientBuilder,
};

use crate::account::DarwiniaAccount;
use crate::darwinia::runtime::DarwiniaRuntime;
use crate::error::{Error, Result};
use crate::events::{DarwiniaEvents, EventInfo};
use crate::frame::ethereum::backing::VerifiedProofStoreExt;
use crate::frame::ethereum::issuing::VerifiedIssuingProofStoreExt;
use crate::frame::sudo::KeyStoreExt;
use crate::rpc::*;

pub struct Darwinia {
    /// client
    pub subxt: Client<DarwiniaRuntime>,
    /// Event Parser
    pub event: DarwiniaEvents,
}

impl Clone for Darwinia {
    fn clone(&self) -> Self {
        Self {
            subxt: self.subxt.clone(),
            event: self.event.clone(),
        }
    }
}

impl Darwinia {
    pub async fn new(url: impl AsRef<str>) -> Result<Darwinia> {
        let client = ClientBuilder::<DarwiniaRuntime>::new()
            .set_url(url.as_ref())
            .skip_type_sizes_check()
            .build()
            .await?;
        let event = DarwiniaEvents::new(client.clone());

        Ok(Self {
            subxt: client,
            event,
        })
    }

    /// get mmr root of darwinia
    pub async fn header_mmr(
        &self,
        block_number_of_member_leaf: u64,
        block_number_of_last_leaf: u64,
        hash: H256,
    ) -> Result<Option<HeaderMMR>> {
        let params = &[
            to_json_value(block_number_of_member_leaf)?,
            to_json_value(block_number_of_last_leaf)?,
        ];
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
        header_hash: H256,
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
    pub async fn get_events_from_block_hash(
        &self,
        hash: H256,
    ) -> Result<Vec<EventInfo<DarwiniaRuntime>>> {
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
                            log::trace!(">> Event - {}", info);
                        }
                    } else {
                        result.push(event);
                    }
                }
                Raw::Error(err) => {
                    log::error!("Error found in raw events: {:#?}", err);
                }
            }
        }
        Ok(result)
    }

    /// get events from a special block
    pub async fn get_events_from_block_number(
        &self,
        block: u32,
    ) -> Result<Vec<EventInfo<DarwiniaRuntime>>> {
        let blockno = BlockNumber::from(block);
        match self.subxt.block_hash(Some(blockno)).await? {
            Some(hash) => return self.get_events_from_block_hash(hash).await,
            None => {
                log::info!("error");
            }
        }
        Ok(vec![])
    }

    /// get mmr root
    pub async fn get_mmr_root(&self, leaf_index: u32) -> Result<H256> {
        let block_number = leaf_index + 1;

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
                let mut mmr_root: [u8; 32] = [0; 32];
                mmr_root.copy_from_slice(parent_mmr_root);
                H256(mmr_root)
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
    ) -> Result<Vec<Bytes>> {
        let keys = vec![StorageKey(storage_key)];
        Ok(self.subxt.read_proof(keys, Some(block_hash)).await?.proof)
    }

    /// get block header by number
    pub async fn get_block_by_number(&self, number: u32) -> Result<Header<u32, BlakeTwo256>> {
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

    /// block number to hash
    pub async fn block_number2hash(&self, block_number: Option<u32>) -> Result<Option<H256>> {
        let block_number = block_number.map(|n| n.into());
        Ok(self.subxt.block_hash(block_number).await?)
    }

    /// is_sudo_key
    pub async fn is_sudo_key(
        &self,
        block_number: Option<u32>,
        account: &DarwiniaAccount,
    ) -> Result<bool> {
        let block_hash = self.block_number2hash(block_number).await?;
        let sudo = self.subxt.key(block_hash).await?;
        Ok(&sudo == account.real())
    }

    /// role
    pub async fn account_role(&self, account: &DarwiniaAccount) -> Result<Vec<String>> {
        let mut roles = vec!["Normal".to_string()];
        if self.is_sudo_key(None, account).await? {
            roles.push("Sudo".to_string());
        }
        Ok(roles)
    }

    /// finalized_head
    pub async fn finalized_head(&self) -> Result<H256> {
        let hash = self.subxt.finalized_head().await?;
        Ok(hash)
    }

    /// get block by hash
    pub async fn get_block_number_by_hash(&self, block_hash: H256) -> Result<Option<u32>> {
        let block = self.subxt.block(Some(block_hash)).await?;
        if let Some(block) = block {
            return Ok(Some(block.block.header.number));
        }
        Ok(None)
    }

    /// Check if should redeem
    pub async fn verified(&self, block_hash: web3::types::H256, tx_index: u64) -> Result<bool> {
        Ok(self
            .subxt
            .verified_proof((block_hash.to_fixed_bytes(), tx_index), None)
            .await?
            .unwrap_or(false))
    }

    /// Check if should issuing sync
    pub async fn verified_issuing(
        &self,
        block_hash: web3::types::H256,
        tx_index: u64,
    ) -> Result<bool> {
        Ok(self
            .subxt
            .verified_issuing_proof((block_hash.to_fixed_bytes(), tx_index), None)
            .await?
            .unwrap_or(false))
    }
}
