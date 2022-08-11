use crate::service::ecdsa_relay::ECDSARelayService;
use crate::{
    bridge::PangoroKilnBus,
    service::{
        header_relay::{
            execution_layer_update::ExecutionLayerRelay,
            kiln_to_pangoro::KilnToPangoroHeaderRelayService,
            sync_committee_update::SyncCommitteeUpdateService,
        },
        message_relay::kiln_to_pangoro::KilnPangoroMessageRelay,
    },
};
use support_lifeline::task::TaskStack;

#[allow(dead_code)]
#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<PangoroKilnBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "pangoro-kiln"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = PangoroKilnBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<KilnToPangoroHeaderRelayService>()?;
        stack.spawn_service::<SyncCommitteeUpdateService>()?;
        stack.spawn_service::<ExecutionLayerRelay>()?;
        stack.spawn_service::<KilnPangoroMessageRelay>()?;
        stack.spawn_service::<ECDSARelayService>()?;
        Ok(Self { stack })
    }
}
