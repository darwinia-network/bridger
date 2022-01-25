use std::collections::HashMap;

use crate::client::PangolinClient;
use crate::codegen::api::runtime_types;
use crate::error::{ClientError, ClientResult};
use crate::types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use crate::types::pangolin_runtime::pallets::proxy::ProxyType;
use crate::types::to_ethereum_backing::pallet::RedeemFor;
use crate::types::{AffirmationsReturn, BetterRelayAffirmation, EthereumReceiptProofThing};

/// Ethereum api
pub struct EthereumApi<'a> {
    client: &'a PangolinClient,
}

impl<'a> EthereumApi<'a> {
    /// Create ethereum api instace
    pub fn new(client: &'a PangolinClient) -> Self {
        Self { client }
    }
}

impl<'a> EthereumApi<'a> {
    /// Get the last confirmed block
    pub async fn last_confirmed(&self) -> ClientResult<u64> {
        let blocks = self
            .client
            .runtime()
            .storage()
            .ethereum_relay()
            .confirmed_block_numbers(None)
            .await?;
        Ok(blocks.iter().max().cloned().unwrap_or(0))
    }

    /// Affirmations
    pub async fn affirmations(&self) -> ClientResult<AffirmationsReturn> {
        let mut result = HashMap::new();
        let mut iter = self
            .client
            .runtime()
            .storage()
            .ethereum_relayer_game()
            .affirmations_iter(None)
            .await?;
        while let Some((mut storage_key, affirmations)) = iter.next().await? {
            // get game id
            let game_id: &mut [u8] = &mut storage_key.0[32..40];
            game_id.reverse();
            let game_id =
                u64::from_str_radix(array_bytes::bytes2hex("", game_id).as_str(), 16).unwrap();

            if result.get(&game_id).is_none() {
                result.insert(game_id, HashMap::<u32, Vec<BetterRelayAffirmation>>::new());
            }
            let game = result.get_mut(&game_id).unwrap();

            // get round id
            let round_id: &mut [u8] = &mut storage_key.0[40..44];
            round_id.reverse();
            let round_id =
                u32::from_str_radix(array_bytes::bytes2hex("", round_id).as_str(), 16).unwrap();

            game.insert(round_id, affirmations);
        }
        Ok(result)
    }

    /// Submit affirmation
    pub async fn affirm(
        &self,
        parcel: EthereumRelayHeaderParcel,
    ) -> ClientResult<subxt::sp_core::H256> {
        let account = self.client.account();
        let v = match account.real() {
            Some(real) => {
                let call = runtime_types::pangolin_runtime::Call::EthereumRelay(
                    runtime_types::darwinia_bridge_ethereum::Call::affirm {
                        ethereum_relay_header_parcel: parcel,
                        optional_ethereum_relay_proofs: None,
                    },
                );
                self.client
                    .runtime()
                    .tx()
                    .proxy()
                    .proxy(real.clone(), Some(ProxyType::EthereumBridge), call)
                    .sign_and_submit(account.signer())
                    .await?
            }
            None => {
                self.client
                    .runtime()
                    .tx()
                    .ethereum_relay()
                    .affirm(parcel, None)
                    .sign_and_submit(account.signer())
                    .await?
            }
        };
        Ok(v)
    }

    /// Register erc20 token
    pub async fn register_erc20(
        &self,
        proof: EthereumReceiptProofThing,
    ) -> ClientResult<subxt::sp_core::H256> {
        let account = self.client.account();
        let v = match account.real() {
            Some(real) => {
                let call = runtime_types::pangolin_runtime::Call::EthereumIssuing(
                    runtime_types::from_ethereum_issuing::pallet::Call::register_erc20 {
                        proof: (proof.header, proof.receipt_proof, proof.mmr_proof),
                    },
                );
                self.client
                    .runtime()
                    .tx()
                    .proxy()
                    .proxy(real.clone(), Some(ProxyType::EthereumBridge), call)
                    .sign_and_submit(account.signer())
                    .await?
            }
            None => {
                self.client
                    .runtime()
                    .tx()
                    .ethereum_issuing()
                    .register_erc20((proof.header, proof.receipt_proof, proof.mmr_proof))
                    .sign_and_submit(account.signer())
                    .await?
            }
        };
        Ok(v)
    }

    /// redeem erc20
    pub async fn redeem_erc20(
        &self,
        proof: EthereumReceiptProofThing,
    ) -> ClientResult<subxt::sp_core::H256> {
        let account = self.client.account();
        let v = match account.real() {
            Some(real) => {
                let call = runtime_types::pangolin_runtime::Call::EthereumIssuing(
                    runtime_types::from_ethereum_issuing::pallet::Call::redeem_erc20 {
                        proof: (proof.header, proof.receipt_proof, proof.mmr_proof),
                    },
                );
                self.client
                    .runtime()
                    .tx()
                    .proxy()
                    .proxy(real.clone(), Some(ProxyType::EthereumBridge), call)
                    .sign_and_submit(account.signer())
                    .await?
            }
            None => {
                self.client
                    .runtime()
                    .tx()
                    .ethereum_issuing()
                    .redeem_erc20((proof.header, proof.receipt_proof, proof.mmr_proof))
                    .sign_and_submit(account.signer())
                    .await?
            }
        };
        Ok(v)
    }

    /// Redeem
    pub async fn redeem(
        &self,
        act: RedeemFor,
        proof: EthereumReceiptProofThing,
    ) -> ClientResult<subxt::sp_core::H256> {
        let ethereum_tx_hash = proof
            .header
            .hash
            .map(|hash| array_bytes::bytes2hex("", &hash))
            .ok_or(ClientError::NoHeaderHashInEthereumReceiptProofOfThing)?;
        let account = self.client.account();
        let v = match account.real() {
            Some(real) => {
                tracing::trace!(
                    target: "client-pangolin",
                    "Proxy redeem ethereum tx 0x{:?} for real account {:?}",
                    ethereum_tx_hash,
                    real
                );
                let call = runtime_types::pangolin_runtime::Call::EthereumBacking(
                    runtime_types::to_ethereum_backing::pallet::Call::redeem {
                        act,
                        proof: (proof.header, proof.receipt_proof, proof.mmr_proof),
                    },
                );
                self.client
                    .runtime()
                    .tx()
                    .proxy()
                    .proxy(real.clone(), Some(ProxyType::EthereumBridge), call)
                    .sign_and_submit(account.signer())
                    .await?
            }
            None => {
                tracing::trace!(
                    target: "client-pangolin",
                    "Redeem ethereum tx {:?} with account {:?}",
                    ethereum_tx_hash,
                    &account.account_id()
                );
                self.client
                    .runtime()
                    .tx()
                    .ethereum_backing()
                    .redeem(act, (proof.header, proof.receipt_proof, proof.mmr_proof))
                    .sign_and_submit(account.signer())
                    .await?
            }
        };
        Ok(v)
    }

    /// Sync authorities change
    pub async fn sync_authorities_change(
        &self,
        proof: EthereumReceiptProofThing,
    ) -> ClientResult<subxt::sp_core::H256> {
        let account = self.client.account();
        let v = match account.real() {
            Some(real) => {
                let call = runtime_types::pangolin_runtime::Call::EthereumBacking(
                    runtime_types::to_ethereum_backing::pallet::Call::sync_authorities_change {
                        proof: (proof.header, proof.receipt_proof, proof.mmr_proof),
                    },
                );
                self.client
                    .runtime()
                    .tx()
                    .proxy()
                    .proxy(real.clone(), Some(ProxyType::EthereumBridge), call)
                    .sign_and_submit(account.signer())
                    .await?
            }
            None => {
                self.client
                    .runtime()
                    .tx()
                    .ethereum_backing()
                    .sync_authorities_change((proof.header, proof.receipt_proof, proof.mmr_proof))
                    .sign_and_submit(account.signer())
                    .await?
            }
        };
        Ok(v)
    }

    /// submit_signed_mmr_root
    pub async fn ecdsa_sign_and_submit_signed_mmr_root(
        &self,
    ) -> ClientResult<subxt::sp_core::H256> {
        let account = self.client.account();
        let runtime_version = self.client.subxt().rpc().runtime_version(None).await?;
        // let spec_name = runtime_version.spec_name.to_string();
        Ok(1.into())
    }
}
