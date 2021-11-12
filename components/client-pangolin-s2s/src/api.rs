use bp_messages::{LaneId, MessageNonce};
use codec::Encode;
use common_primitives::AccountId;
use common_primitives::Balance;
use common_primitives::BlockNumber;
use dp_fee::{Order, Relayer};
use relay_substrate_client::{ChainBase, Client, TransactionSignScheme, UnsignedTransaction};
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
    pub async fn client(&self) -> &Client<PangolinChain> {
        &self.client
    }

    /// Query assigned relayers
    pub async fn assigned_relayers(
        &self,
    ) -> anyhow::Result<Option<Vec<Relayer<AccountId, Balance>>>> {
        Ok(self
            .client
            .storage_value(
                StorageKey(
                    patch::storage_prefix(
                        "FeeMarket".as_bytes(),
                        "AssignedRelayersStorage".as_bytes(),
                    )
                    .to_vec(),
                ),
                None,
            )
            .await?)
    }

    /// Query order
    pub async fn order(
        &self,
        laned_id: LaneId,
        message_nonce: MessageNonce,
    ) -> anyhow::Result<Option<Order<AccountId, BlockNumber, Balance>>> {
        Ok(self
            .client
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
        Ok(self.client.best_finalized_header_number().await?)
    }

    /// Update relay fee
    pub async fn update_relay_fee(
        &self,
        signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
        amount: <PangolinChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = self.client.genesis_hash().clone();
        self.client
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
        let genesis_hash = self.client.genesis_hash().clone();
        self.client
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
