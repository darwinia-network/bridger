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
                SubmitSignedMmrRootCallExt,
                VerifiedProofStoreExt,
                EthereumBackingEventsDecoder
            },
            game::{AffirmationsStoreExt, EthereumRelayerGame, EthereumRelayerGameEventsDecoder},
            relay::{
                Affirm, AffirmCallExt, ConfirmedBlockNumbersStoreExt, EthereumRelay,
                PendingRelayHeaderParcelsStoreExt, SetConfirmedParcel,
                VotePendingRelayHeaderParcelCallExt,
                VotePendingRelayHeaderParcel,
                EthereumRelayEventsDecoder
            },
        },
        proxy::ProxyCallExt,
        sudo::SudoCallExt,
        bridge::relay_authorities::{
            EthereumRelayAuthoritiesEventsDecoder,
            SubmitSignedAuthoritiesCallExt
        }
    },
    runtime::DarwiniaRuntime,
};
use std::collections::HashMap;
use substrate_subxt::{system::System, Client, ClientBuilder, EventSubscription, EventsDecoder};
use web3::types::H256;
use crate::error::BizError;
use crate::api::darwinia_sender::DarwiniaSender;
use parity_scale_codec::{Compact, Encode};

// Types
type PendingRelayHeaderParcel = <DarwiniaRuntime as EthereumRelay>::PendingRelayHeaderParcel;
type RelayAffirmation = <DarwiniaRuntime as EthereumRelayerGame>::RelayAffirmation;
type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<RelayAffirmation>>>;
/// AccountId
pub type AccountId = <DarwiniaRuntime as System>::AccountId;
type BlockNumber = <DarwiniaRuntime as System>::BlockNumber;

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
                client.clone()
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
    pub async fn ecdsa_sign_and_submit_signed_authorities(&self, message: &[u8]) -> Result<H256> {
        let signature = self.sender.ecdsa_sign(message)?;
        Ok(self.client.submit_signed_authorities(&self.sender.signer, signature).await?)
    }

    /// submit_signed_mmr_root
    pub async fn ecdsa_sign_and_submit_signed_mmr_root(&self, spec_name: &str, block_number: BlockNumber, mmr_root: H256) -> Result<H256> {
        let mut encoded: Vec<u8> = vec![];
        encoded.append(&mut spec_name.encode());
        encoded.append(&mut Compact(block_number).encode());
        encoded.append(&mut mmr_root.encode());
        let signature = self.sender.ecdsa_sign(&encoded)?;
        Ok(self.client.submit_signed_mmr_root(&self.sender.signer, block_number, mmr_root, signature).await?)
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

    /// Build event subscription
    pub async fn build_event_subscription(&self) -> Result<EventSubscription<DarwiniaRuntime>> {
        let scratch = self.client.subscribe_events().await?;
        let mut decoder = EventsDecoder::<DarwiniaRuntime>::new(self.client.metadata().clone());

        // Register decoders
        decoder.with_ethereum_backing();
        decoder.with_ethereum_relayer_game();
        decoder.with_ethereum_relay();
        decoder.with_ethereum_relay_authorities();

        // Build subscriber
        let sub = EventSubscription::<DarwiniaRuntime>::new(scratch, decoder);
        Ok(sub)
    }
}
