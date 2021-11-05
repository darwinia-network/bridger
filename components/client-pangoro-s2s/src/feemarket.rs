use messages_relay::message_lane::MessageLane;
use messages_relay::message_lane_loop::{
    SourceClient as MessageLaneSourceClient, TargetClient as MessageLaneTargetClient,
};
use messages_relay::relay_strategy::{RelayReference, RelayStrategy};
use relay_substrate_client::Client;

use crate::PangoroChain;

#[derive(Clone)]
pub struct PangoroRelayStrategy {
    client: Client<PangoroChain>,
}

impl PangoroRelayStrategy {
    pub fn new(client: Client<PangoroChain>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl RelayStrategy for PangoroRelayStrategy {
    async fn decide<
        P: MessageLane,
        SourceClient: MessageLaneSourceClient<P>,
        TargetClient: MessageLaneTargetClient<P>,
    >(
        &self,
        reference: &mut RelayReference<P, SourceClient, TargetClient>,
    ) -> bool {
        true
    }
}
