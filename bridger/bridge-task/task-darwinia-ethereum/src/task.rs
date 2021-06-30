use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Sender};

use bridge_config::config::component::{
    BeeConfig, EthereumRpcConfig, MicrokvConfig, ShadowConfig, Web3Config,
};
use bridge_config::config::service::SubstrateEthereumConfig;
use bridge_config::Config;
use bridge_shared::shared::SharedChannel;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask};

use crate::bus::DarwiniaEthereumBus;
use crate::message::s2e::EthereumScanMessage;
use crate::service::ethereum::LikeDarwiniaWithLikeEthereumEthereumScanService;
use crate::service::relay::LikeDarwiniaWithLikeEthereumRelayService;

#[derive(Debug)]
pub struct DarwiniaEthereumTask {
    services: Vec<Box<dyn BridgeService>>,
}

impl BridgeTask for DarwiniaEthereumTask {}

impl BridgeSand for DarwiniaEthereumTask {
    const NAME: &'static str = "task-darwinia-ethereum";
}

impl DarwiniaEthereumTask {
    pub async fn new(
        config: DarwiniaEthereumConfig,
        channel: SharedChannel,
    ) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = DarwiniaEthereumBus::default();
        bus.store_resource::<SharedChannel>(channel.clone());

        let services = vec![
            Self::spawn_service::<LikeDarwiniaWithLikeEthereumRelayService>(&bus)?,
            Self::spawn_service::<LikeDarwiniaWithLikeEthereumEthereumScanService>(&bus)?,
        ];

        // todo: only test code
        let mut tx_scan = bus.tx::<EthereumScanMessage>()?;
        tx_scan.send(EthereumScanMessage::Start).await?;

        Ok(Self { services })
    }
}

impl DarwiniaEthereumTask {
    fn spawn_service<
        S: lifeline::Service<Bus = DarwiniaEthereumBus, Lifeline = anyhow::Result<S>>
            + BridgeService
            + 'static,
    >(
        bus: &DarwiniaEthereumBus,
    ) -> anyhow::Result<Box<dyn BridgeService>> {
        Ok(Box::new(S::spawn(bus)?))
    }
}

#[derive(Clone, Debug)]
pub struct DarwiniaEthereumConfig {
    pub bee: BeeConfig,
    pub web3: Web3Config,
    pub ethereum_rpc: EthereumRpcConfig,
    pub shadow: ShadowConfig,
    pub microkv: MicrokvConfig,
    pub service: SubstrateEthereumConfig,
}

impl DarwiniaEthereumConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        let name = cell_name.as_ref();
        Config::store(name, self.bee.clone())?;
        Config::store(name, self.web3.clone())?;
        Config::store(name, self.ethereum_rpc.clone())?;
        Config::store(name, self.shadow.clone())?;
        Config::store(name, self.service.clone())?;
        Config::store(name, self.microkv.clone())?;
        Ok(())
    }
}
