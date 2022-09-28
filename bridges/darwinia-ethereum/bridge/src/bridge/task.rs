use lifeline::dyn_bus::DynBus;

use component_state::state::{BridgeState, StateOptions};
use support_common::config::{Config, Names};
use support_lifeline::task::TaskStack;

use crate::bridge::BridgeConfig;
use crate::service::ecdsa_relay::ECDSARelayService;
use crate::service::message_relay::darwinia_to_goerli::DarwiniaEthereumMessageRelay;
use crate::{
    bridge::BridgeBus,
    service::{
        header_relay::{
            execution_layer_update::ExecutionLayerRelay,
            goerli_to_darwinia::EthereumToDarwiniaHeaderRelayService,
            sync_committee_update::SyncCommitteeUpdateService,
        },
        message_relay::goerli_to_darwinia::EthereumDarwiniaMessageRelay,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "darwinia-goerli"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let state = BridgeState::new(StateOptions {
            db_name: Self::name().to_string(),
        })?;
        // check config
        let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;

        let bus = BridgeBus::default();
        bus.store_resource::<BridgeState>(state);

        let mut stack = TaskStack::new(bus);
        if bridge_config.general.header_goerli_to_darwinia {
            stack.spawn_service::<EthereumToDarwiniaHeaderRelayService>()?;
        }
        if bridge_config.general.sync_commit_goerli_to_darwinia {
            stack.spawn_service::<SyncCommitteeUpdateService>()?;
        }
        if bridge_config.general.execution_layer_goerli_to_darwinia {
            stack.spawn_service::<ExecutionLayerRelay>()?;
        }
        if bridge_config.general.ecdsa_service {
            stack.spawn_service::<ECDSARelayService>()?;
        }
        if bridge_config.general.msg_goerli_to_darwinia {
            stack.spawn_service::<EthereumDarwiniaMessageRelay>()?;
        }
        if bridge_config.general.msg_darwinia_to_goerli {
            stack.spawn_service::<DarwiniaEthereumMessageRelay>()?;
        }
        Ok(Self { stack })
    }
}
