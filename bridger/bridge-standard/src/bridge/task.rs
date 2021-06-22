use crate::bridge::chain::BridgeChain;
use crate::external::lifeline::LifelineBus;

pub trait BridgeTask {
    const NAME: String;
    type Source: BridgeChain;
    type Target: BridgeChain;
    type Bus: LifelineBus;

    fn spawn_service<S: lifeline::Service>(&self, bus: &Self::Bus) -> anyhow::Result<()> {
        S::spawn(bus)
    }
}
