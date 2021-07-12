use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Sender};
use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep};
use component_darwinia::config::DarwiniaConfig;
use component_ethereum::config::{EthereumRpcConfig, Web3Config};
use component_shadow::ShadowConfig;
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::config::SubstrateEthereumConfig;
use crate::message::{DarwiniaEthereumMessage, EthereumScanMessage};
use crate::service::ethereum::LikeDarwiniaWithLikeEthereumEthereumScanService;
use crate::service::relay::LikeDarwiniaWithLikeEthereumRelayService;

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
    async fn route(
        &self,
        uri: String,
        param: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        crate::route::dispatch_route(&self.bus, uri, param).await
    }
}

impl BridgeTask<DarwiniaEthereumBus> for DarwiniaEthereumTask {
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
            Self::spawn_service::<LikeDarwiniaWithLikeEthereumEthereumScanService>(&bus)?,
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

impl DarwiniaEthereumTask {
    fn spawn_service<
        S: lifeline::Service<Bus = DarwiniaEthereumBus, Lifeline = anyhow::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        bus: &DarwiniaEthereumBus,
    ) -> anyhow::Result<Box<dyn BridgeService + Send + Sync>> {
        Ok(Box::new(S::spawn(bus)?))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaEthereumConfig {
    pub darwinia: DarwiniaConfig,
    pub web3: Web3Config,
    pub ethereum_rpc: EthereumRpcConfig,
    pub shadow: ShadowConfig,
    pub service: SubstrateEthereumConfig,
}

impl DarwiniaEthereumConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        let name = cell_name.as_ref();
        Config::store(name, self.darwinia.clone())?;
        Config::store(name, self.web3.clone())?;
        Config::store(name, self.ethereum_rpc.clone())?;
        Config::store(name, self.shadow.clone())?;
        Config::store(name, self.service.clone())?;
        Ok(())
    }
}
