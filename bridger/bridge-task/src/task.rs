use bridge_standard::bridge::chain::BridgeChain;
use bridge_standard::bridge::task::BridgeTask;
use bridge_standard::external::lifeline::LifelineBus;
use chain_darwinia::chain::DarwiniaChain;
use chain_ethereum::chain::EthereumChain;
use std::fmt::Debug;

use crate::bus::DarwiniaEthereumBus;

pub struct TaskDarwiniaEthereum {}

impl BridgeTask for TaskDarwiniaEthereum {
    const NAME: String = "darwinia-ethereum".to_string();
    type Source = DarwiniaChain;
    type Target = EthereumChain;
    type Bus = DarwiniaEthereumBus;
}
