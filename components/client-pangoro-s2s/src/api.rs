use bp_messages::{LaneId, MessageNonce};
use codec::Encode;
use common_primitives::AccountId;
use common_primitives::Balance;
use common_primitives::BlockNumber;
use dp_fee::{Order, Relayer};
use relay_substrate_client::Client;
use sp_core::storage::StorageKey;

use crate::{patch, PangoroChain};

#[derive(Clone)]
pub struct PangoroApi {
    client: Client<PangoroChain>,
}

impl PangoroApi {
    pub fn new(client: Client<PangoroChain>) -> Self {
        Self { client }
    }
}

impl PangoroApi {
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
}
