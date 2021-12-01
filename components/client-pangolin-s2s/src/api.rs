use std::future::Future;
use std::pin::Pin;

use bp_messages::{LaneId, MessageNonce};
use bridge_traits::error::StandardError;
use codec::{Decode, Encode};
use common_primitives::AccountId;
use common_primitives::Balance;
use common_primitives::BlockNumber;
use dp_fee::{Order, Relayer};
use relay_substrate_client::{ChainBase, Client, TransactionSignScheme, UnsignedTransaction};
use relay_utils::relay_loop::Client as RelayLoopClient;
use relay_utils::MaybeConnectionError;
use sp_core::storage::StorageKey;
use sp_core::{Bytes, Pair};

use crate::{patch, PangolinChain};

#[derive(Clone)]
pub struct PangolinApi {
    client: Client<PangolinChain>,
}

impl PangolinApi {
    pub fn new(client: Client<PangolinChain>) -> Self {
        Self { client }
    }
}

impl PangolinApi {
    async fn _storage_value<T: Send + Decode + 'static>(
        &mut self,
        storage_key: StorageKey,
        block_hash: Option<<PangolinChain as relay_substrate_client::ChainBase>::Hash>,
        times: u32,
    ) -> anyhow::Result<Option<T>> {
        loop {
            if times > 10 {
                return Err(StandardError::Api("Failed to query storage".to_string()).into());
            }
            match self
                .client
                .storage_value(storage_key.clone(), block_hash.clone())
                .await
            {
                Ok(v) => return Ok(v),
                Err(e) => {
                    if e.is_connection_error() {
                        self.client.reconnect().await?;
                    }
                }
            }
        }
    }
}

impl PangolinApi {
    pub async fn client(&self) -> anyhow::Result<Client<PangolinChain>> {
        let mut client = self.client.clone();
        client.reconnect().await?;
        Ok(client)
    }

    /// Query assigned relayers
    pub async fn assigned_relayers(&mut self) -> anyhow::Result<Vec<Relayer<AccountId, Balance>>> {
        let storage_key = StorageKey(
            patch::storage_prefix("FeeMarket".as_bytes(), "AssignedRelayers".as_bytes()).to_vec(),
        );
        Ok(self
            ._storage_value(storage_key, None, 0)
            .await?
            .unwrap_or_default())
    }

    /// Query all relayers
    pub async fn relayers(&self) -> anyhow::Result<Vec<AccountId>> {
        Ok(self
            .client()
            .await?
            .storage_value(
                StorageKey(
                    patch::storage_prefix("FeeMarket".as_bytes(), "Relayers".as_bytes()).to_vec(),
                ),
                None,
            )
            .await?
            .unwrap_or_else(Vec::new))
    }

    /// Query relayer info by account id
    pub async fn relayer(
        &self,
        account: AccountId,
    ) -> anyhow::Result<Option<Relayer<AccountId, Balance>>> {
        Ok(self
            .client()
            .await?
            .storage_value(
                bp_runtime::storage_map_final_key_blake2_128concat(
                    "FeeMarket",
                    "RelayersMap",
                    account.encode().as_slice(),
                ),
                None,
            )
            .await?)
    }

    pub async fn is_relayer(&self, account: AccountId) -> anyhow::Result<bool> {
        self.relayer(account).await.map(|item| item.is_some())
    }

    /// Query order
    pub async fn order(
        &self,
        laned_id: LaneId,
        message_nonce: MessageNonce,
    ) -> anyhow::Result<Option<Order<AccountId, BlockNumber, Balance>>> {
        Ok(self
            .client()
            .await?
            .storage_value(
                bp_runtime::storage_map_final_key_blake2_128concat(
                    "FeeMarket",
                    "Orders",
                    (laned_id, message_nonce).encode().as_slice(),
                ),
                None,
            )
            .await?)
    }

    /// Return number of the best finalized block.
    pub async fn best_finalized_header_number(
        &self,
    ) -> anyhow::Result<common_primitives::BlockNumber> {
        Ok(self.client().await?.best_finalized_header_number().await?)
    }

    /// Update relay fee
    pub async fn update_relay_fee(
        &self,
        signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
        amount: <PangolinChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let client = self.client().await?;
        let genesis_hash = *client.genesis_hash();
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    PangolinChain::sign_transaction(
                        genesis_hash,
                        &signer,
                        relay_substrate_client::TransactionEra::immortal(),
                        UnsignedTransaction::new(
                            pangolin_runtime::FeeMarketCall::update_relay_fee(amount).into(),
                            transaction_nonce,
                        ),
                    )
                    .encode(),
                )
            })
            .await?;
        Ok(())
    }

    /// Update locked collateral
    pub async fn update_locked_collateral(
        &self,
        signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
        amount: <PangolinChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let client = self.client().await?;
        let genesis_hash = *client.genesis_hash();
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    PangolinChain::sign_transaction(
                        genesis_hash,
                        &signer,
                        relay_substrate_client::TransactionEra::immortal(),
                        UnsignedTransaction::new(
                            pangolin_runtime::FeeMarketCall::update_locked_collateral(amount)
                                .into(),
                            transaction_nonce,
                        ),
                    )
                    .encode(),
                )
            })
            .await?;
        Ok(())
    }
}
