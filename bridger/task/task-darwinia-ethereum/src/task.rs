use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Sender};

use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::config::DarwiniaEthereumConfig;
use crate::message::{DarwiniaEthereumMessage, EthereumScanMessage};
use crate::service::darwinia::DarwiniaService;
use crate::service::ethereum::LikeDarwiniaWithLikeEthereumEthereumScanService;
use crate::service::extrinsics::ExtrinsicsService;
use crate::service::guard::GuardService;
use crate::service::redeem::RedeemService;
use crate::service::relay::LikeDarwiniaWithLikeEthereumRelayService;

#[derive(Debug)]
pub struct DarwiniaEthereumTask {
    bus: DarwiniaEthereumBus,
    stack: TaskStack<DarwiniaEthereumBus>,
}

impl BridgeSand for DarwiniaEthereumTask {
    const NAME: &'static str = "task-darwinia-ethereum";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for DarwiniaEthereumTask {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    async fn route(&self, uri: String, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
        crate::route::dispatch_route(&self.bus, uri, param).await
    }
}

impl BridgeTask<DarwiniaEthereumBus> for DarwiniaEthereumTask {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(DarwiniaEthereumConfig::template())?)
    }
    fn bus(&self) -> &DarwiniaEthereumBus {
        &self.bus
    }

    fn stack(&mut self) -> &mut TaskStack<DarwiniaEthereumBus> {
        &mut self.stack
    }
}

impl DarwiniaEthereumTask {
    pub async fn new(config: DarwiniaEthereumConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = DarwiniaEthereumBus::default();
        bus.store_resource::<BridgeState>(state);

        let mut stack = TaskStack::new();
        stack.spawn_service::<LikeDarwiniaWithLikeEthereumEthereumScanService>(&bus)?;
        stack.spawn_service::<LikeDarwiniaWithLikeEthereumRelayService>(&bus)?;
        stack.spawn_service::<RedeemService>(&bus)?;
        stack.spawn_service::<GuardService>(&bus)?;
        stack.spawn_service::<DarwiniaService>(&bus)?;
        stack.spawn_service::<ExtrinsicsService>(&bus)?;

        let mut tx_scan = bus.tx::<DarwiniaEthereumMessage>()?;
        tx_scan
            .send(DarwiniaEthereumMessage::Scan(EthereumScanMessage::Start))
            .await?;

        Ok(Self { bus, stack })
    }
}
