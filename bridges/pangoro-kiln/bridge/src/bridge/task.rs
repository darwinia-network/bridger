use lifeline::dyn_bus::DynBus;

use component_state::state::{BridgeState, StateOptions};
use support_common::config::{Config, Names};
use support_lifeline::task::TaskStack;

use crate::bridge::BridgeConfig;
use crate::service::ecdsa_relay::ECDSARelayService;
use crate::{
    bridge::BridgeBus,
    service::{
        header_relay::{
            execution_layer_update::ExecutionLayerRelay,
            kiln_to_pangoro::KilnToPangoroHeaderRelayService,
            sync_committee_update::SyncCommitteeUpdateService,
        },
        message_relay::kiln_to_pangoro::KilnPangoroMessageRelay,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "pangoro-kiln"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let state = BridgeState::new(StateOptions {
            db_name: Self::name().to_string(),
        })?;
        // check config
        let _bridge_config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;

        let bus = BridgeBus::default();
        bus.store_resource::<BridgeState>(state);

        let mut stack = TaskStack::new(bus);
        // stack.spawn_service::<KilnToPangoroHeaderRelayService>()?;
        // stack.spawn_service::<SyncCommitteeUpdateService>()?;
        // stack.spawn_service::<ExecutionLayerRelay>()?;
        // stack.spawn_service::<KilnPangoroMessageRelay>()?;
        stack.spawn_service::<ECDSARelayService>()?;
        Ok(Self { stack })
    }
}
