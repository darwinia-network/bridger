use messages_relay::message_lane::MessageLane;
use messages_relay::message_lane_loop::{
    SourceClient as MessageLaneSourceClient, TargetClient as MessageLaneTargetClient,
};
use messages_relay::relay_strategy::{RelayReference, RelayStrategy};
use relay_substrate_client::Client;
use sp_core::storage::StorageKey;

use crate::PangolinChain;

#[derive(Clone)]
pub struct PangolinRelayStrategy {
    client: Client<PangolinChain>,
}

impl PangolinRelayStrategy {
    pub fn new(client: Client<PangolinChain>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl RelayStrategy for PangolinRelayStrategy {
    async fn decide<
        P: MessageLane,
        SourceClient: MessageLaneSourceClient<P>,
        TargetClient: MessageLaneTargetClient<P>,
    >(
        &self,
        reference: &mut RelayReference<P, SourceClient, TargetClient>,
    ) -> bool {
        let nonce = &reference.nonce;
        // self.client.storage_value(StorageKey())
        true
    }
}
