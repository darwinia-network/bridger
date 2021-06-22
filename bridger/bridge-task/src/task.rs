use bridge_standard::bridge::chain::BridgeChain;
use bridge_standard::bridge::task::BridgeTask;
use chain_darwinia::chain::DarwiniaChain;
use chain_ethereum::chain::EthereumChain;
use std::fmt::Debug;

use crate::bus::DarwiniaEthereumBus;

pub struct TaskDarwiniaEthereum {}

impl BridgeTask for TaskDarwiniaEthereum {
    const NAME: &'static str = "task-darwinia-ethereum";
    type Source = DarwiniaChain;
    type Target = EthereumChain;
    type Bus = DarwiniaEthereumBus;
}

impl TaskDarwiniaEthereum {
    pub fn spawn_service<S: lifeline::Service<Bus = <Self as BridgeTask>::Bus>>(
        bus: &<Self as BridgeTask>::Bus,
    ) -> S::Lifeline {
        S::spawn(bus)
    }
}
