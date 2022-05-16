use std::collections::HashMap;

use crate::subxt_runtime::api::runtime_types;
use crate::subxt_runtime::api::runtime_types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use crate::subxt_runtime::api::runtime_types::darwinia_runtime::pallets::proxy::ProxyType;
use crate::subxt_runtime::api::runtime_types::to_ethereum_backing::pallet::RedeemFor;
use subxt::{BasicError, MetadataError};

use crate::client::DarwiniaClient;
use crate::config::DarwiniaSubxtConfig;
use crate::error::{ClientError, ClientResult};
use crate::helpers;
use crate::types::{
    AffirmationsReturn, BetterRelayAffirmation, EcdsaMessage, EthereumAccount,
    EthereumReceiptProofThing,
};

/// Ethereum api
pub struct EthereumApi<'a> {
    client: &'a DarwiniaClient,
}

impl<'a> EthereumApi<'a> {
    /// Create ethereum api instace
    pub fn new(client: &'a DarwiniaClient) -> Self {
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
        let runtime = self.client.runtime();
        let mut iter = runtime
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
                let call = runtime_types::darwinia_runtime::Call::EthereumRelay(
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
                    target: "client-darwinia",
                    "Proxy redeem ethereum tx 0x{:?} for real account {:?}",
                    ethereum_tx_hash,
                    real
                );
                let call = runtime_types::darwinia_runtime::Call::EthereumBacking(
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
                    target: "client-darwinia",
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
                let call = runtime_types::darwinia_runtime::Call::EthereumBacking(
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
        ethereum_account: EthereumAccount,
        block_number: u32,
    ) -> ClientResult<subxt::sp_core::H256> {
        let darwinia_account = self.client.account();
        let spec_name = self.client.spec_name().await?;

        // get mmr root from darwinia
        let mmr_root = self.client.get_mmr_root(block_number).await?;
        let encoded = helpers::encode_mmr_root_message(spec_name, block_number, mmr_root);
        let hash = web3::signing::keccak256(&encoded);
        let signature = ethereum_account.ecdsa_sign(&hash)?;

        let v = match darwinia_account.real() {
            Some(real) => {
                tracing::trace!(
                    target: "client-darwinia",
                    "Proxied ecdsa sign and submit mmr_root to darwinia, block_number: {}",
                    block_number
                );
                let call = runtime_types::darwinia_runtime::Call::EthereumRelayAuthorities(
                    runtime_types::darwinia_relay_authorities::Call::submit_signed_mmr_root {
                        block_number,
                        signature: signature.0,
                    },
                );
                self.client
                    .runtime()
                    .tx()
                    .proxy()
                    .proxy(real.clone(), Some(ProxyType::EthereumBridge), call)
                    .sign_and_submit(darwinia_account.signer())
                    .await?
            }
            None => {
                tracing::trace!(
                    target: "client-darwinia",
                    "Ecdsa sign and submit mmr_root to darwinia, block_number: {}, signature: {:?}",
                    block_number,
                    signature
                );
                self.client
                    .runtime()
                    .tx()
                    .ethereum_relay_authorities()
                    .submit_signed_mmr_root(block_number, signature.0)
                    .sign_and_submit(darwinia_account.signer())
                    .await?
            }
        };
        Ok(v)
    }

    /// submit signed authorities
    pub async fn ecdsa_sign_and_submit_signed_authorities(
        &self,
        ethereum_account: EthereumAccount,
        message: EcdsaMessage,
    ) -> ClientResult<subxt::sp_core::H256> {
        let signature = ethereum_account.ecdsa_sign(&message)?;
        let darwinia_account = self.client.account();

        let v = match darwinia_account.real() {
            Some(real) => {
                tracing::trace!(target: "client-darwinia", "Proxied ecdsa sign and submit authorities to darwinia");
                let call = runtime_types::darwinia_runtime::Call::EthereumRelayAuthorities(
                    runtime_types::darwinia_relay_authorities::Call::submit_signed_authorities {
                        signature: signature.0,
                    },
                );
                self.client
                    .runtime()
                    .tx()
                    .proxy()
                    .proxy(real.clone(), Some(ProxyType::EthereumBridge), call)
                    .sign_and_submit(darwinia_account.signer())
                    .await?
            }
            None => {
                tracing::trace!(target: "client-darwinia", "Ecdsa sign and submit authorities to darwinia");
                self.client
                    .runtime()
                    .tx()
                    .ethereum_relay_authorities()
                    .submit_signed_authorities(signature.0)
                    .sign_and_submit(darwinia_account.signer())
                    .await?
            }
        };
        Ok(v)
    }

    /// vote pending relay header parcel
    pub async fn vote_pending_relay_header_parcel(
        &self,
        ethereum_block_number: u64,
        aye: bool,
    ) -> ClientResult<subxt::sp_core::H256> {
        let darwinia_account = self.client.account();

        let v = match darwinia_account.real() {
            Some(real) => {
                tracing::trace!(target: "client-darwinia", "Proxied vote pending relay header parcel");
                let call = runtime_types::darwinia_runtime::Call::EthereumRelay(
                    runtime_types::darwinia_bridge_ethereum::Call::vote_pending_relay_header_parcel {
                        ethereum_block_number,
                        aye,
                    },
                );
                self.client
                    .runtime()
                    .tx()
                    .proxy()
                    .proxy(real.clone(), Some(ProxyType::EthereumBridge), call)
                    .sign_and_submit(darwinia_account.signer())
                    .await?
            }
            None => {
                tracing::trace!(target: "client-darwinia", "Vote pending relay header parcel");
                self.client
                    .runtime()
                    .tx()
                    .ethereum_relay()
                    .vote_pending_relay_header_parcel(ethereum_block_number, aye)
                    .sign_and_submit(darwinia_account.signer())
                    .await?
            }
        };
        Ok(v)
    }

    /// The tx is verified
    pub async fn is_verified(&self, block_hash: &[u8], tx_index: u64) -> ClientResult<bool> {
        let hash = subxt::sp_core::H256::from_slice(block_hash);
        let v0: bool = self
            .client
            .runtime()
            .storage()
            .ethereum_backing()
            .verified_proof(&hash, &tx_index, None)
            .await?;
        Ok(v0)
    }

    /// Is authority
    pub async fn is_authority(
        &self,
        block_number: Option<u32>,
        account: &<DarwiniaSubxtConfig as subxt::Config>::AccountId,
    ) -> ClientResult<bool> {
        let hash = self
            .client
            .subxt()
            .rpc()
            .block_hash(block_number.map(|v| v.into()))
            .await?;
        let authorities = self
            .client
            .runtime()
            .storage()
            .ethereum_relay_authorities()
            .authorities(hash)
            .await?;
        Ok(authorities.iter().any(|v| &v.account_id == account))
    }

    /// need to sign authorities
    pub async fn need_to_sign_authorities(
        &self,
        block_number: Option<u32>,
        account: &<DarwiniaSubxtConfig as subxt::Config>::AccountId,
        message: EcdsaMessage,
    ) -> ClientResult<bool> {
        let hash = self
            .client
            .subxt()
            .rpc()
            .block_hash(block_number.map(|v| v.into()))
            .await?;
        let ret = self
            .client
            .runtime()
            .storage()
            .ethereum_relay_authorities()
            .authorities_to_sign(hash)
            .await?;
        match ret {
            None => Ok(false),
            Some(r) => {
                if r.0 == message {
                    let includes = r.1.iter().any(|a| &a.0 == account);
                    Ok(!includes)
                } else {
                    Ok(false)
                }
            }
        }
    }

    /// need to sign mmr root of block
    pub async fn need_to_sign_mmr_root_of(
        &self,
        block_number: u32,
        exec_block_number: Option<u32>,
        account: &<DarwiniaSubxtConfig as subxt::Config>::AccountId,
    ) -> ClientResult<bool> {
        let exec_block_hash = self
            .client
            .subxt()
            .rpc()
            .block_hash(exec_block_number.map(|v| v.into()))
            .await?;
        let mmr_roots_to_sign = self
            .client
            .runtime()
            .storage()
            .ethereum_relay_authorities()
            .mmr_roots_to_sign(&block_number, exec_block_hash)
            .await?;
        match mmr_roots_to_sign {
            None => {
                tracing::debug!(
                    target: "client-darwinia",
                    "No mmr root found in block {} and exec block {}",
                    block_number,
                    exec_block_number.unwrap_or(0)
                );
                Ok(false)
            }
            Some(m) => {
                let need = !m.signatures.iter().any(|a| &a.0 == account);
                if !need {
                    tracing::debug!(
                        target: "client-darwinia",
                        "The account({:?}) is signed. don't need sign again",
                        &account
                    );
                }
                Ok(need)
            }
        }
    }

    /// hack query ethereum relay authorities next term.
    /// todo: fix storage prefix name same with pallet name is better way
    pub async fn ethereum_relay_authorities_next_term(&self) -> ClientResult<u32> {
        let current_term = match self
            .client
            .runtime()
            .storage()
            .ethereum_relay_authorities()
            .next_term(None)
            .await
        {
            Ok(v) => v,
            Err(e) => match e {
                BasicError::Metadata(MetadataError::PalletNotFound(pallet_name)) => {
                    match &pallet_name[..] {
                        "Instance1DarwiniaRelayAuthorities" => 0u32,
                        _ => {
                            return Err(BasicError::Metadata(MetadataError::PalletNotFound(
                                pallet_name,
                            ))
                            .into());
                        }
                    }
                }
                _ => return Err(e.into()),
            },
        };
        Ok(current_term)
    }
}
