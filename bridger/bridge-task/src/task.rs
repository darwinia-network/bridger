use bridge_standard::bridge::task::BridgeTask;
use chain_darwinia::chain::DarwiniaChain;
use chain_ethereum::chain::EthereumChain;

use crate::bus::DarwiniaEthereumBus;

pub struct TaskDarwiniaEthereum {
    bus: <Self as BridgeTask>::Bus,
}

impl BridgeTask for TaskDarwiniaEthereum {
    const NAME: &'static str = "task-darwinia-ethereum";
    type Source = DarwiniaChain;
    type Target = EthereumChain;
    type Bus = DarwiniaEthereumBus;
}

impl TaskDarwiniaEthereum {
    pub fn new() -> Self {
        Self {
            bus: <Self as BridgeTask>::Bus::default(),
        }
    }
    pub fn bus(&self) -> &<Self as BridgeTask>::Bus {
        &self.bus
    }

    pub fn spawn_service<S: lifeline::Service<Bus = <Self as BridgeTask>::Bus>>(
        &self,
    ) -> S::Lifeline {
        S::spawn(&self.bus)
    }
}
