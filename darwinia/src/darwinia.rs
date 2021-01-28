use substrate_subxt::{
	Client as Subxt,
    ClientBuilder,
    sp_core::{twox_128, H256, Bytes},
    sp_core::storage::{StorageData, StorageKey},
    BlockNumber,
    events::Raw,
    sp_runtime::traits::{
        Header as TraitHeader,
        BlakeTwo256,
    },
    sp_runtime::generic::Header,
};

use primitives::{
	runtime::{DarwiniaRuntime},
};

use crate::{
    Rpc,
    DarwiniaEvents,
    EventInfo,
    account::DarwiniaAccount,
};

use crate::error::{
    Result,
    DarwiniaError,
    Error,
};

use primitives::{
	frame::{
		sudo::KeyStoreExt,
    },
};

pub struct Darwinia {
    /// jsonrpc client
    pub rpc: Rpc,
	/// client
	pub subxt: Subxt<DarwiniaRuntime>,
    /// Event Parser
    pub event: DarwiniaEvents,
}

impl Clone for Darwinia {
    fn clone(&self) -> Self {
        Self {
            rpc: self.rpc.clone(),
            subxt: self.subxt.clone(),
            event: self.event.clone(),
        }
    }
}

impl Darwinia {
	pub async fn new(url: &str) -> Result<Darwinia> {
        let client = jsonrpsee::ws_client(url).await?;
        let rpc = Rpc::new(client.clone());
		let client = ClientBuilder::<DarwiniaRuntime>::new()
			.set_client(client.clone())
			.build()
			.await?;
        let event = DarwiniaEvents::new(client.clone());
		//let signer_seed = config.darwinia_to_ethereum.seed.clone();
		//let sender = DarwiniaSender::new(
			//config.seed.clone(),
			//config
				//.proxy
				//.clone()
				//.map(|proxy| proxy.real[2..].to_string()),
			//client.clone(),
			//signer_seed,
			//config.eth.rpc.to_string(),
		//);

		Ok(Self{
            rpc, 
            subxt: client,
            event,
        })
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

		Err(anyhow::anyhow!("StorageData not found"))
	}

    /// get events from a special block
    pub async fn get_events_from_block_number(&self, block: u32) -> Result<Vec<EventInfo<DarwiniaRuntime>>> {
        let blockno = BlockNumber::from(block);
        match self.subxt.block_hash(Some(blockno)).await? {
            Some(hash) => {
                let storage_data = self
                    .get_storage_data("System", "Events", hash)
                    .await?;

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
                                info!("cannot decode event {}", info);
                            } else {
                                result.push(event);
                            }
                        }
                        Raw::Error(err) => {
                            error!("Error found in raw events: {:#?}", err);
                        }
                    }
                }
                return Ok(result)
            },
            None => {
                info!("error");
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
					return Err(DarwiniaError::Bridger(format!(
						"Wrong parent_mmr_root len: {}",
						parent_mmr_root.len()
					))
					.into());
				}
				let mut mmr_root: [u8; 32] = [0; 32];
				mmr_root.copy_from_slice(&parent_mmr_root);
				H256(mmr_root)
			} else {
				return Err(
					DarwiniaError::Bridger("Wrong header with no parent_mmr_root".to_string()).into(),
				);
			}
		} else {
			return Err(DarwiniaError::Bridger("No header fetched".to_string()).into());
		};
		Ok(mmr_root)
	}

    /// events proof
    pub async fn get_event_proof(&self, storage_key: Vec<u8>, block_hash: H256) -> Result<Vec<Bytes>> {
		let keys = vec![StorageKey(storage_key)];
        Ok(self.subxt.read_proof(keys, Some(block_hash)).await?.proof)
    }

    /// get block header by number
    pub async fn get_block_by_number(&self, number: u32) -> Result<Header<u32, BlakeTwo256>> {
        match self.subxt.block_hash(Some(BlockNumber::from(number))).await? {
            Some(block_hash) => {
                match self.subxt.header(Some(block_hash)).await? {
                    Some(header) => {
                        Ok(header)
                    }
                    None => Err(Error::Other("get header return nil".to_string()).into()),
                }
            }
            None => Err(Error::Other("get block hash failed".to_string()).into())
        }
    }

    /// is_sudo_key
    pub async fn is_sudo_key(&self, account: &DarwiniaAccount) -> Result<bool> {
        let sudo = self.subxt.key(None).await?;
        Ok(&sudo == account.real())
    }

    /// role
    pub async fn account_role(&self, account: &DarwiniaAccount) -> Result<Vec<String>> {
        let mut roles = vec!["Normal".to_string()];
        if self.is_sudo_key(account).await? {
            roles.push("Sudo".to_string());
        }
        Ok(roles)
    }
}

