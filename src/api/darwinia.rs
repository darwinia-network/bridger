//! Darwinia API
use crate::{service::redeem::EthereumTransaction, error::{Result, Error}, Config};
use core::marker::PhantomData;
use primitives::{
    chain::{
        ethereum::{EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor},
        proxy_type::ProxyType,
    },
    frame::{
        ethereum::{
            backing::{
                Redeem,
                RedeemCallExt,
                VerifiedProofStoreExt,
            },
            game::{AffirmationsStoreExt, EthereumRelayerGame,},
            relay::{
                Affirm, AffirmCallExt, ConfirmedBlockNumbersStoreExt, EthereumRelay,
                PendingRelayHeaderParcelsStoreExt, SetConfirmedParcel,
                VotePendingRelayHeaderParcelCallExt,
                VotePendingRelayHeaderParcel,
            },
        },
        proxy::ProxyCallExt,
        sudo::SudoCallExt,
        bridge::relay_authorities::{
            SubmitSignedAuthorities,
            SubmitSignedAuthoritiesCallExt,
            SubmitSignedMmrRoot,
            SubmitSignedMmrRootCallExt,
            AuthorityTermStoreExt,
        }
    },
    runtime::{DarwiniaRuntime, EcdsaMessage, EcdsaSignature, EcdsaAddress},
};
use std::collections::HashMap;
use substrate_subxt::{system::System, BlockNumber, Client, ClientBuilder, EventsDecoder, RawEvent};
use crate::error::BizError;
use crate::api::darwinia_sender::DarwiniaSender;
use parity_scale_codec::Encode;
use substrate_subxt::sp_runtime::traits::Header;
use substrate_subxt::sp_core::{twox_128, H256};
use substrate_subxt::sp_core::storage::{StorageKey, StorageData};
use substrate_subxt::events::Raw;

// Types
type PendingRelayHeaderParcel = <DarwiniaRuntime as EthereumRelay>::PendingRelayHeaderParcel;
type RelayAffirmation = <DarwiniaRuntime as EthereumRelayerGame>::RelayAffirmation;
type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<RelayAffirmation>>>;
/// AccountId
pub type AccountId = <DarwiniaRuntime as System>::AccountId;

/// Dawrinia API
pub struct Darwinia {
    /// client
    pub client: Client<DarwiniaRuntime>,
    /// account
    pub sender: DarwiniaSender,
}

impl Darwinia {
    /// New darwinia API
    pub async fn new(config: &Config) -> Result<Darwinia> {
        let client =
            jsonrpsee::ws_client(&config.node).await
                .map_err(|e| {
                    Error::FailToConnectDarwinia {
                        url: config.node.clone(),
                        source: e
                    }
                })?;
        let client = ClientBuilder::<DarwiniaRuntime>::new()
            .set_client(client)
            .build()
            .await?;

        let sender = DarwiniaSender::new(
            config.seed.clone(),
            config.proxy.clone().map(|proxy| proxy.real[2..].to_string()),
            client.clone(),
            config.darwinia_to_ethereum.seed.clone()[2..].to_string(),
            config.eth.rpc.to_string(),
		);

        Ok(Darwinia {
            client,
            sender,
        })
    }

    /// set confirmed with sudo privilege
    pub async fn set_confirmed_parcel(&self, parcel: EthereumRelayHeaderParcel) -> Result<H256> {
        let ex = self.client.encode(SetConfirmedParcel {
            ethereum_relay_header_parcel: parcel,
            _runtime: PhantomData::default(),
        })?;
        Ok(self.client.sudo(&self.sender.signer, &ex).await?)
    }

    /// Vote pending relay header parcel
    pub async fn vote_pending_relay_header_parcel(&self, pending: u64, aye: bool) -> Result<H256> {
        if self.sender.is_tech_comm_member().await? {
            match &self.sender.real {
                Some(real) => { // proxy
                    trace!("Proxy vote for {:?}", real);
                    let vote = VotePendingRelayHeaderParcel {
                        block_number: pending,
                        aye,
                        _runtime: PhantomData::default(),
                    };

                    let ex = self.client.encode(vote).unwrap();
                    let ex_hash = self.client.proxy(&self.sender.signer, real.clone(), Some(ProxyType::EthereumBridge), &ex).await?;
                    Ok(ex_hash)
                },
                None => { // no proxy
                    let ex_hash = self.client
                        .vote_pending_relay_header_parcel(&self.sender.signer, pending, aye)
                        .await?;
                    Ok(ex_hash)
                }
            }
        } else {
            Err(BizError::Bridger("Not technical committee member".to_string()).into())
        }
    }

    /// Get all active games' affirmations
    /// games = {
    ///   game_id: {
    ///     round_id: [...]
    ///   }
    /// }
    pub async fn affirmations(&self) -> Result<AffirmationsReturn> {
        let mut result = HashMap::new();
        let mut iter = self.client.affirmations_iter(None).await?;
        while let Some((mut storage_key, affirmations)) = iter.next().await? {
            // get game id
            let game_id: &mut [u8] = &mut storage_key.0[32..40];
            game_id.reverse();
            let game_id = u64::from_str_radix(hex::encode(game_id).as_str(), 16).unwrap();

            //
            if result.get(&game_id).is_none() {
                result.insert(game_id, HashMap::<u32, Vec<RelayAffirmation>>::new());
            }
            let game = result.get_mut(&game_id).unwrap();

            // get round id
            let round_id: &mut [u8] = &mut storage_key.0[40..44];
            round_id.reverse();
            let round_id = u32::from_str_radix(hex::encode(round_id).as_str(), 16).unwrap();

            game.insert(round_id, affirmations);
        }
        Ok(result)
    }

    /// Get confirmed block numbers
    pub async fn confirmed_block_numbers(&self) -> Result<Vec<u64>> {
        Ok(self.client.confirmed_block_numbers(None).await?)
    }

    /// Get the last confirmed block
    pub async fn last_confirmed(&self) -> Result<u64> {
        Ok(
            if let Some(confirmed) = self.confirmed_block_numbers().await?.iter().max() {
                *confirmed
            } else {
                0
            },
        )
    }

    /// Get pending headers
    pub async fn pending_headers(&self) -> Result<Vec<PendingRelayHeaderParcel>> {
        Ok(self.client.pending_relay_header_parcels(None).await?)
    }

    /// Submit affirmation
    pub async fn affirm(&self, parcel: EthereumRelayHeaderParcel) -> Result<H256> {
        match &self.sender.real {
            Some(real) => {
                trace!("Proxy call `affirm` for {:?}", real);
                let affirm = Affirm {
                    _runtime: PhantomData::default(),
                    ethereum_relay_header_parcel: parcel,
                    ethereum_relay_proofs: None,
                };

                let ex = self.client.encode(affirm).unwrap();
                Ok(self.client.proxy(&self.sender.signer, real.clone(), Some(ProxyType::EthereumBridge), &ex).await?)
            },
            None => {
                Ok(self.client.affirm(&self.sender.signer, parcel, None).await?)
            }
        }
    }

    /// Redeem
    pub async fn redeem(
        &self,
        redeem_for: RedeemFor,
        proof: EthereumReceiptProofThing,
    ) -> Result<H256> {
        let ethereum_tx_hash = proof.header.hash
            .map(|hash| hex::encode(&hash))
            .ok_or_else(|| BizError::Bridger("No hash in header".to_string()))?;
        match &self.sender.real {
            Some(real) => {
                trace!("Proxy redeem ethereum tx 0x{:?} for real account {:?}", ethereum_tx_hash, real);
                let redeem = Redeem {
                    _runtime: PhantomData::default(),
                    act: redeem_for,
                    proof,
                };

                let ex = self.client.encode(redeem).unwrap();
                Ok(self.client.proxy(&self.sender.signer, real.clone(), Some(ProxyType::EthereumBridge), &ex).await?)
            },
            None => {
                trace!("Redeem ethereum tx 0x{:?} with account {:?}", ethereum_tx_hash, &self.sender.account_id);
                Ok(self.client.redeem(&self.sender.signer, redeem_for, proof).await?)
            }
        }
    }

    /// submit_signed_authorities
    pub async fn ecdsa_sign_and_submit_signed_authorities(&self, message: EcdsaMessage) -> Result<H256> {
        if self.sender.is_authority().await? {
            let signature = self.sender.ecdsa_sign(&message)?;
            match &self.sender.real { // proxy
                Some(real) => {
                    trace!("Proxyed ecdsa sign and submit authorities to darwinia");
                    let submit_signed_authorities = SubmitSignedAuthorities {
                        signature,
                    };

                    let ex = self.client.encode(submit_signed_authorities).unwrap();
                    let tx_hash = self.client.proxy(&self.sender.signer, real.clone(), Some(ProxyType::EthereumBridge), &ex).await?;
                    Ok(tx_hash)
                },
                None => {
                    trace!("Ecdsa sign and submit authorities to darwinia");
                    let tx_hash = self
                        .client
                        .submit_signed_authorities(&self.sender.signer, signature)
                        .await?;
                    Ok(tx_hash)
                }
            }
        } else {
            Err(BizError::Bridger("Not authority".to_string()).into())
        }
    }

    /// construct_message
    pub fn construct_authorities_message(first: String, second: u32, third: Vec<EcdsaAddress>) -> Vec<u8> {
        debug!("Infos to construct eth authorities message: {:?}, {:?}, {:?}", first, second, third.iter().map(|a| hex::encode(&a)).collect::<Vec<_>>().join(", "));
        // scale encode & sign
        let message = _S {
            _1: first,
            _2: second,
            _3: third
        };
        let encoded: &[u8] = &message.encode();
        encoded.to_vec()
    }

    /// submit_signed_mmr_root
    pub async fn ecdsa_sign_and_submit_signed_mmr_root(
        &self,
        spec_name: String,
        block_number: u32,
    ) -> Result<H256> {
        if self.sender.is_authority().await? {
            // get mmr root from darwinia
            let leaf_index = block_number;
            let mmr_root = self.get_mmr_root(leaf_index).await?;

            debug!("Infos to construct mmr_root message: {}, {}, {:?}", spec_name.clone(), block_number, mmr_root);
            // scale encode & sign
            let message = _S {
					_1: spec_name,
					_2: block_number,
					_3: mmr_root
				};
			let encoded = message.encode();

            let hash = web3::signing::keccak256(&encoded);
            let signature = self.sender.ecdsa_sign(&hash)?;

            match &self.sender.real { // proxy
                Some(real) => {
                    trace!("Proxyed ecdsa sign and submit mmr_root to darwinia, block_number: {}", block_number);
                    let submit_signed_mmr_root = SubmitSignedMmrRoot {
                        block_number,
                        mmr_root,
                        signature,
                    };

                    let ex = self.client.encode(submit_signed_mmr_root).unwrap();
                    let tx_hash = self.client.proxy(&self.sender.signer, real.clone(), Some(ProxyType::EthereumBridge), &ex).await?;
                    Ok(tx_hash)
                },
                None => {
                    trace!("Ecdsa sign and submit mmr_root to darwinia, block_number: {}", block_number);
                    let tx_hash = self
                        .client
                        .submit_signed_mmr_root(&self.sender.signer, block_number, mmr_root, signature)
                        .await?;
                    Ok(tx_hash)
                }
            }
        } else {
            Err(BizError::Bridger("Not authority".to_string()).into())
        }
    }

    async fn get_mmr_root(&self, leaf_index: u32) -> Result<H256> {
        // Get mmr_root from block number == leaf_index + 1
        let block_number = leaf_index + 1;

        // TODO: 是否需要考虑finalized
        let block_hash = self
            .client
            .block_hash(Some(BlockNumber::from(block_number)))
            .await?;
        let header = self.client.header(block_hash).await?;

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
                    return Err(BizError::Bridger(format!(
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
                    BizError::Bridger("Wrong header with no parent_mmr_root".to_string()).into(),
                );
            }
        } else {
            return Err(BizError::Bridger("No header fetched".to_string()).into());
        };

        Ok(mmr_root)
    }


    /// Check if should redeem
    pub async fn verified(&self, tx: &EthereumTransaction) -> Result<bool> {
        Ok(self
            .client
            .verified_proof((tx.block_hash.to_fixed_bytes(), tx.index), None)
            .await?
            .unwrap_or(false))
    }

    /// affirmations contains block?
    pub fn contains(affirmations: &[RelayAffirmation], block: u64) -> bool {
        for affirmation in affirmations {
            let blocks: &Vec<u64> = &affirmation
                .relay_header_parcels
                .iter()
                .map(|bp| bp.header.number)
                .collect();
            if blocks.contains(&block) {
                return true;
            }
        }

        // TODO: Checking the equality of the affirmations

        // TODO: If there is an affirmation with larger block number, then agree and join in the game.

        // TODO: How to play and join the game
        false
    }

    /// get_raw_events
    pub async fn get_raw_events(&self, header_hash: H256) -> Result<Vec<RawEvent>> {
        let mut events = vec![];

        let storage_data = self.get_storage_data("System", "Events", header_hash).await?;

        let mut decoder = EventsDecoder::<DarwiniaRuntime>::new(self.client.metadata().clone());
        decoder.register_type_size::<u128>("Balance");
        decoder.register_type_size::<u128>("RingBalance");
        decoder.register_type_size::<u128>("KtonBalance");
        decoder.register_type_size::<[u8; 20]>("EthereumAddress");
        decoder.register_type_size::<[u8; 20]>("EcdsaAddress");
        decoder.register_type_size::<H256>("MMRRoot");
        decoder.register_type_size::<[u8; 32]>("RelayAuthorityMessage");
        decoder.register_type_size::<[u8; 20]>("RelayAuthoritySigner");
        decoder.register_type_size::<EcdsaSignature>("RelayAuthoritySignature");
        decoder.register_type_size::<u8>("ElectionCompute"); // just a hack
        decoder.register_type_size::<u32>("Term");

        let raw_events = decoder.decode_events(&mut &storage_data.0[..])?;
        for (_, raw) in raw_events {
            match raw {
                Raw::Event(event) => {
                    events.push(event);
                },
                Raw::Error(err) => {
                    error!("Error found in raw events: {:#?}", err);
                }
            }
        }

        Ok(events)
    }

    /// get_storage_data
    pub async fn get_storage_data(&self, module_name: &str, storage_name: &str, header_hash: H256) -> Result<StorageData> {
        let mut storage_key = twox_128(module_name.as_bytes()).to_vec();
        storage_key.extend(twox_128(storage_name.as_bytes()).to_vec());

        let keys = vec![StorageKey(storage_key)];

        let change_sets = self.client.query_storage(keys, header_hash, Some(header_hash)).await?;
        for change_set in change_sets {
            for (_key, data) in change_set.changes {
                if let Some(data) = data {
                    return Ok(data)
                }
            }
        }

        Err(anyhow::anyhow!("StorageData not found"))
    }

    /// get_current_term
    pub async fn get_current_authority_term(&self) -> Result<u32>{
        Ok(self.client.authority_term(None).await?)
    }
}
#[derive(Encode)]
struct _S<_1, _2, _3>
where
	_1: Encode,
	_2: Encode,
	_3: Encode,
{
	_1: _1,
	#[codec(compact)]
	_2: _2,
	_3: _3,
}

#[test]
fn test_encode() {
	let s = _S {
		_1: "Pangolin",
		_2: 50u32,
		_3: [38u8, 199, 154, 103, 135, 242, 210, 106, 168, 120, 216, 232, 234, 114, 194, 69, 189, 238, 196, 220, 4, 5, 74, 15, 181, 223, 155, 200, 224, 204, 189, 1],
	};
	println!("{:?}", s.encode());
}
