use std::marker::PhantomData;

use bridge_e2e_traits::client::EcdsaClient;
use component_state::state::{BridgeState, StateOptions};
use lifeline::dyn_bus::DynBus;

use support_lifeline::task::TaskStack;

use crate::bridge::BridgeBus;
use crate::config::BridgeConfig;
use crate::service::ecdsa_relay::ECDSARelayService;
use crate::service::header_relay::beacon_header_relay::EthereumToDarwiniaHeaderRelayService;
use crate::service::header_relay::execution_layer_relay::ExecutionLayerRelay;
use crate::service::header_relay::sync_committee_relay::SyncCommitteeUpdateService;
use crate::service::message_relay::darwinia_to_eth::DarwiniaEthereumMessageRelay;
use crate::service::message_relay::eth_to_darwinia::EthereumDarwiniaMessageRelay;

#[derive(Debug)]
pub struct BridgeTask<T: EcdsaClient> {
    stack: TaskStack<BridgeBus>,
    _substrate: PhantomData<T>,
}

impl<T: EcdsaClient> BridgeTask<T> {
    pub fn new(bridge_config: BridgeConfig<T>) -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();
        let state = BridgeState::new(StateOptions {
            db_name: bridge_config.name.clone(),
        })?;

        bus.store_resource::<BridgeState>(state);
        let mut stack = TaskStack::new(bus);
        stack.bus().store_resource(bridge_config);
        stack.spawn_service::<EthereumToDarwiniaHeaderRelayService<T>>()?;
        stack.spawn_service::<ExecutionLayerRelay<T>>()?;
        stack.spawn_service::<SyncCommitteeUpdateService<T>>()?;
        stack.spawn_service::<EthereumDarwiniaMessageRelay<T>>()?;
        stack.spawn_service::<DarwiniaEthereumMessageRelay<T>>()?;
        stack.spawn_service::<ECDSARelayService<T>>()?;

        Ok(Self {
            stack,
            _substrate: Default::default(),
        })
    }
}

impl<T: EcdsaClient> BridgeTask<T> {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
