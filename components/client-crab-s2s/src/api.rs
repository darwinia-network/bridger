use bp_messages::{LaneId, MessageNonce};
use codec::Encode;
use darwinia_common_primitives::AccountId;
use darwinia_common_primitives::Balance;
use darwinia_common_primitives::BlockNumber;
use dp_fee::{Order, Relayer};
use relay_substrate_client::{ChainBase, Client, TransactionSignScheme, UnsignedTransaction};
use relay_utils::relay_loop::Client as RelayLoopClient;
use relay_utils::MaybeConnectionError;
use sp_core::storage::StorageKey;
use sp_core::{Bytes, Pair};

use crate::{patch, CrabChain};

#[derive(Clone)]
pub struct CrabApi {
    client: Client<CrabChain>,
}

impl CrabApi {
    pub fn new(client: Client<CrabChain>) -> Self {
        Self { client }
    }
}

impl CrabApi {
    /// Query assigned relayers
    pub async fn assigned_relayers(&mut self) -> anyhow::Result<Vec<Relayer<AccountId, Balance>>> {
        let storage_key = StorageKey(
            patch::storage_prefix("FeeMarket".as_bytes(), "AssignedRelayers".as_bytes()).to_vec(),
        );
        match self.client.storage_value(storage_key, None).await {
            Ok(v) => Ok(v.unwrap_or_default()),
            Err(e) => {
                if e.is_connection_error() {
                    self.client.reconnect().await?;
                }
                Err(e)?
            }
        }
    }

    /// Query order
    pub async fn order(
        &mut self,
        laned_id: LaneId,
        message_nonce: MessageNonce,
    ) -> anyhow::Result<Option<Order<AccountId, BlockNumber, Balance>>> {
        let storage_key = bp_runtime::storage_map_final_key_blake2_128concat(
            "FeeMarket",
            "Orders",
            (laned_id, message_nonce).encode().as_slice(),
        );
        match self.client.storage_value(storage_key.clone(), None).await {
            Ok(v) => Ok(v),
            Err(e) => {
                if e.is_connection_error() {
                    self.client.reconnect().await?;
                }
                Err(e)?
            }
        }
    }

    /// Query all relayers
    pub async fn relayers(&mut self) -> anyhow::Result<Vec<AccountId>> {
        let storage_key = StorageKey(
            patch::storage_prefix("FeeMarket".as_bytes(), "Relayers".as_bytes()).to_vec(),
        );
        match self.client.storage_value(storage_key, None).await {
            Ok(v) => Ok(v.unwrap_or_default()),
            Err(e) => {
                if e.is_connection_error() {
                    self.client.reconnect().await?;
                }
                Err(e)?
            }
        }
    }

    /// Query relayer info by account id
    pub async fn relayer(
        &mut self,
        account: AccountId,
    ) -> anyhow::Result<Option<Relayer<AccountId, Balance>>> {
        let storage_key = bp_runtime::storage_map_final_key_blake2_128concat(
            "FeeMarket",
            "RelayersMap",
            account.encode().as_slice(),
        );
        match self.client.storage_value(storage_key.clone(), None).await {
            Ok(v) => Ok(v),
            Err(e) => {
                if e.is_connection_error() {
                    self.client.reconnect().await?;
                }
                Err(e)?
            }
        }
    }

    pub async fn is_relayer(&mut self, account: AccountId) -> anyhow::Result<bool> {
        self.relayer(account).await.map(|item| item.is_some())
    }

    /// Return number of the best finalized block.
    pub async fn best_finalized_header_number(
        &mut self,
    ) -> anyhow::Result<darwinia_common_primitives::BlockNumber> {
        match self.client.best_finalized_header_number().await {
            Ok(v) => Ok(v),
            Err(e) => {
                if e.is_connection_error() {
                    self.client.reconnect().await?;
                }
                Err(e)?
            }
        }
    }

    /// Update relay fee
    pub async fn update_relay_fee(
        &mut self,
        signer: <CrabChain as TransactionSignScheme>::AccountKeyPair,
        amount: <CrabChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *self.client.genesis_hash();
        match self
            .client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    CrabChain::sign_transaction(
                        genesis_hash,
                        &signer,
                        relay_substrate_client::TransactionEra::immortal(),
                        UnsignedTransaction::new(
                            crab_runtime::FeeMarketCall::update_relay_fee(amount).into(),
                            transaction_nonce,
                        ),
                    )
                    .encode(),
                )
            })
            .await
        {
            Ok(_v) => Ok(()),
            Err(e) => {
                if e.is_connection_error() {
                    self.client.reconnect().await?;
                }
                Err(e)?
            }
        }
    }

    /// Update locked collateral
    pub async fn update_locked_collateral(
        &mut self,
        signer: <CrabChain as TransactionSignScheme>::AccountKeyPair,
        amount: <CrabChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *self.client.genesis_hash();
        match self
            .client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    CrabChain::sign_transaction(
                        genesis_hash,
                        &signer,
                        relay_substrate_client::TransactionEra::immortal(),
                        UnsignedTransaction::new(
                            crab_runtime::FeeMarketCall::update_locked_collateral(amount).into(),
                            transaction_nonce,
                        ),
                    )
                    .encode(),
                )
            })
            .await
        {
            Ok(_v) => Ok(()),
            Err(e) => {
                if e.is_connection_error() {
                    self.client.reconnect().await?;
                }
                Err(e)?
            }
        }
    }
}
