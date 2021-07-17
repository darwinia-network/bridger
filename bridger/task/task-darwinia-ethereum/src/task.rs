use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Sender};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep, TaskTerminal};
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::config::DarwiniaEthereumConfig;
use crate::message::{DarwiniaEthereumMessage, EthereumScanMessage};
use crate::service::ethereum::LikeDarwiniaWithLikeEthereumEthereumScanService;
use crate::service::relay::LikeDarwiniaWithLikeEthereumRelayService;
use crate::service::redeem::RedeemService;
use crate::service::extrinsics::ExtrinsicsService;
use crate::service::guard::GuardService;
use crate::service::darwinia::DarwiniaService;

#[derive(Debug)]
pub struct DarwiniaEthereumTask {
    bus: DarwiniaEthereumBus,
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for DarwiniaEthereumTask {
    const NAME: &'static str = "task-darwinia-ethereum";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for DarwiniaEthereumTask {
    fn as_any(&self) -> &dyn std::any::Any {
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

    fn keep_carry(&mut self, other_bus: Lifeline) {
        self.carries.push(other_bus);
    }
}

impl DarwiniaEthereumTask {
    pub async fn new(config: DarwiniaEthereumConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = DarwiniaEthereumBus::default();
        bus.store_resource::<BridgeState>(state);

        let services = vec![
            Self::spawn_service::<LikeDarwiniaWithLikeEthereumRelayService>(&bus)?,
            // Self::spawn_service::<LikeDarwiniaWithLikeEthereumEthereumScanService>(&bus)?,
            // Self::spawn_service::<RedeemService>(&bus)?,
            // Self::spawn_service::<ExtrinsicsService>(&bus)?,
            // Self::spawn_service::<GuardService>(&bus)?,
            // Self::spawn_service::<DarwiniaService>(&bus)?,
        ];

        let mut tx_scan = bus.tx::<DarwiniaEthereumMessage>()?;
        tx_scan
            .send(DarwiniaEthereumMessage::Scan(EthereumScanMessage::Start))
            .await?;

        let carries = vec![];
        Ok(Self {
            bus,
            services,
            carries,
        })
    }
}
