use lifeline::dyn_bus::DynBus;

use component_state::state::{BridgeState, StateOptions};
use support_common::config::{Config, Names};
use support_lifeline::task::TaskStack;

use crate::bridge::BridgeConfig;
use crate::service::ecdsa_relay::ECDSARelayService;
use crate::service::message_relay::pangoro_to_goerli::PangoroGoerliMessageRelay;
use crate::{
    bridge::BridgeBus,
    service::{
        header_relay::{
            execution_layer_update::ExecutionLayerRelay,
            goerli_to_pangoro::GoerliToPangoroHeaderRelayService,
            sync_committee_update::SyncCommitteeUpdateService,
        },
        message_relay::goerli_to_pangoro::GoerliPangoroMessageRelay,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "pangoro-goerli"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let state = BridgeState::new(StateOptions {
            db_name: Self::name().to_string(),
        })?;
        // check config
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;

        let bus = BridgeBus::default();
        bus.store_resource::<BridgeState>(state);

        let mut stack = TaskStack::new(bus);
        if bridge_config.general.enable_beacon_header_relay {
            stack.spawn_service::<GoerliToPangoroHeaderRelayService>()?;
        }
        if bridge_config.general.enable_sync_commit_relay {
            stack.spawn_service::<SyncCommitteeUpdateService>()?;
        }
        if bridge_config.general.enable_execution_header_layer {
            stack.spawn_service::<ExecutionLayerRelay>()?;
        }
        if bridge_config.general.enable_ecdsa_relay {
            stack.spawn_service::<ECDSARelayService>()?;
        }
        if bridge_config.general.enable_message_execution_to_evm {
            stack.spawn_service::<GoerliPangoroMessageRelay>()?;
        }
        if bridge_config.general.enable_message_evm_to_execution {
            stack.spawn_service::<PangoroGoerliMessageRelay>()?;
        }
        Ok(Self { stack })
    }
}
