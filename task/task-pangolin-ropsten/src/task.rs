use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Sender};

use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};
use component_state::state::BridgeState;

use crate::bus::PangolinRopstenBus;
use crate::config::DarwiniaEthereumConfig;
use crate::message::{DarwiniaEthereumMessage, EthereumScanMessage};
use crate::service::darwinia::DarwiniaService;
use crate::service::ethereum::LikeDarwiniaWithLikeEthereumEthereumScanService;
use crate::service::extrinsics::ExtrinsicsService;
use crate::service::guard::GuardService;
use crate::service::redeem::RedeemService;
use crate::service::relay::LikeDarwiniaWithLikeEthereumRelayService;

#[derive(Debug)]
pub struct PangolinRopstenTask {
    stack: TaskStack<PangolinRopstenBus>,
}

impl BridgeSand for PangolinRopstenTask {
    const NAME: &'static str = "task-pangolin-ropsten";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for PangolinRopstenTask {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    async fn route(&self, uri: String, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
        crate::route::dispatch_route(&self.stack.bus(), uri, param).await
    }
}

impl BridgeTask<PangolinRopstenBus> for PangolinRopstenTask {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(DarwiniaEthereumConfig::template())?)
    }

    fn stack(&mut self) -> &mut TaskStack<PangolinRopstenBus> {
        &mut self.stack
    }
}

impl PangolinRopstenTask {
    pub async fn new(config: DarwiniaEthereumConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = PangolinRopstenBus::default();
        bus.store_resource::<BridgeState>(state);

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<LikeDarwiniaWithLikeEthereumEthereumScanService>()?;
        stack.spawn_service::<LikeDarwiniaWithLikeEthereumRelayService>()?;
        stack.spawn_service::<RedeemService>()?;
        stack.spawn_service::<GuardService>()?;
        stack.spawn_service::<DarwiniaService>()?;
        stack.spawn_service::<ExtrinsicsService>()?;

        let mut tx_scan = stack.bus().tx::<DarwiniaEthereumMessage>()?;
        tx_scan
            .send(DarwiniaEthereumMessage::Scan(EthereumScanMessage::Start))
            .await?;

        Ok(Self { stack })
    }
}
