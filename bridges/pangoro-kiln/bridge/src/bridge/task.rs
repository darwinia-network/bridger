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
pub struct PangoroKilnServiceManager {
    stack: TaskStack<PangoroKilnBus>,
}

impl PangoroKilnServiceManager {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = PangoroKilnBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<KilnToPangoroHeaderRelayService>()?;
        stack.spawn_service::<SyncCommitteeUpdateService>()?;
        stack.spawn_service::<ExecutionLayerRelay>()?;
        stack.spawn_service::<KilnPangoroMessageRelay>()?;
        Ok(Self { stack })
    }
}
