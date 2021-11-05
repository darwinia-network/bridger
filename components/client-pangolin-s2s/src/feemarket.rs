use messages_relay::message_lane::MessageLane;
use messages_relay::message_lane_loop::{
    SourceClient as MessageLaneSourceClient, TargetClient as MessageLaneTargetClient,
};
use messages_relay::relay_strategy::{RelayReference, RelayStrategy};

#[derive(Clone)]
pub struct PangolinRelayStrategy {}

impl PangolinRelayStrategy {
    pub fn new() -> Self {
        Self {}
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
        true
    }
}
