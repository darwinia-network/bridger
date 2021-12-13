use bp_messages::{LaneId, MessageNonce};
use codec::Encode;
use darwinia_common_primitives::AccountId;
use darwinia_common_primitives::Balance;
use darwinia_common_primitives::BlockNumber;
use dp_fee::{Order, Relayer};
use relay_substrate_client::{
    ChainBase, Client, SignParam, TransactionSignScheme, UnsignedTransaction,
};
use relay_utils::relay_loop::Client as RelayLoopClient;
use sp_core::storage::StorageKey;
use sp_core::{Bytes, Pair};

use crate::{patch, DarwiniaChain};

#[derive(Clone)]
pub struct DarwiniaApi {
    client: Client<DarwiniaChain>,
}

impl DarwiniaApi {
    pub fn new(client: Client<DarwiniaChain>) -> Self {
        Self { client }
    }
}

impl DarwiniaApi {
    pub async fn reconnect(&mut self) -> anyhow::Result<()> {
        Ok(self.client.reconnect().await?)
    }

    /// Query assigned relayers
    pub async fn assigned_relayers(&self) -> anyhow::Result<Vec<Relayer<AccountId, Balance>>> {
        let storage_key = StorageKey(
            patch::storage_prefix("FeeMarket".as_bytes(), "AssignedRelayers".as_bytes()).to_vec(),
        );
        Ok(self
            .client
            .storage_value(storage_key, None)
            .await?
            .unwrap_or_default())
    }

    /// Query order
    pub async fn order(
        &self,
        laned_id: LaneId,
        message_nonce: MessageNonce,
    ) -> anyhow::Result<Option<Order<AccountId, BlockNumber, Balance>>> {
        let storage_key = bp_runtime::storage_map_final_key_blake2_128concat(
            "FeeMarket",
            "Orders",
            (laned_id, message_nonce).encode().as_slice(),
        );
        Ok(self.client.storage_value(storage_key.clone(), None).await?)
    }

    /// Query all relayers
    pub async fn relayers(&self) -> anyhow::Result<Vec<AccountId>> {
        let storage_key = StorageKey(
            patch::storage_prefix("FeeMarket".as_bytes(), "Relayers".as_bytes()).to_vec(),
        );
        Ok(self
            .client
            .storage_value(storage_key, None)
            .await?
            .unwrap_or_default())
    }

    /// Query relayer info by account id
    pub async fn relayer(
        &self,
        account: AccountId,
    ) -> anyhow::Result<Option<Relayer<AccountId, Balance>>> {
        let storage_key = bp_runtime::storage_map_final_key_blake2_128concat(
            "FeeMarket",
            "RelayersMap",
            account.encode().as_slice(),
        );
        Ok(self.client.storage_value(storage_key.clone(), None).await?)
    }

    pub async fn is_relayer(&self, account: AccountId) -> anyhow::Result<bool> {
        self.relayer(account).await.map(|item| item.is_some())
    }

    /// Return number of the best finalized block.
    pub async fn best_finalized_header_number(&self) -> anyhow::Result<BlockNumber> {
        Ok(self.client.best_finalized_header_number().await?)
    }

    /// Update relay fee
    pub async fn update_relay_fee(
        &self,
        signer: <DarwiniaChain as TransactionSignScheme>::AccountKeyPair,
        amount: <DarwiniaChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *self.client.genesis_hash();
        let runtime_version = self.client.runtime_version().await?;
        self.client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Ok(Bytes(
                    DarwiniaChain::sign_transaction(SignParam {
                        spec_version: runtime_version.spec_version,
                        transaction_version: runtime_version.transaction_version,
                        genesis_hash,
                        signer,
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            darwinia_runtime::FeeMarketCall::update_relay_fee(amount).into(),
                            transaction_nonce,
                        ),
                    })
                    .encode(),
                ))
            })
            .await?;
        Ok(())
    }

    /// Update locked collateral
    pub async fn update_locked_collateral(
        &self,
        signer: <DarwiniaChain as TransactionSignScheme>::AccountKeyPair,
        amount: <DarwiniaChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *self.client.genesis_hash();
        let runtime_version = self.client.runtime_version().await?;
        self.client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Ok(Bytes(
                    DarwiniaChain::sign_transaction(SignParam {
                        spec_version: runtime_version.spec_version,
                        transaction_version: runtime_version.transaction_version,
                        genesis_hash,
                        signer,
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            darwinia_runtime::FeeMarketCall::update_locked_collateral(amount)
                                .into(),
                            transaction_nonce,
                        ),
                    })
                    .encode(),
                ))
            })
            .await?;
        Ok(())
    }
}
