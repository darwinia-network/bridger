use std::collections::HashMap;

use crate::client::PangolinClient;
use crate::codegen::api::{ethereum_relay, ethereum_relayer_game, runtime_types};
use crate::config::PangolinSubxtConfig;
use crate::error::ClientResult;
use crate::types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use crate::types::pangolin_runtime::pallets::proxy::ProxyType;
use crate::types::{AffirmationsReturn, BetterRelayAffirmation, DarwiniaAccount};

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

    /// Sync authorities change
    pub async fn sync_authorities_change(&self) -> ClientResult<subxt::sp_core::H256> {
        Ok(1)
    }
}
