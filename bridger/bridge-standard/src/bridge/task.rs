use lifeline::dyn_bus::DynBus;

use crate::bridge::chain::BridgeChain;

pub trait BridgeTask {
    const NAME: &'static str;
    type Source: BridgeChain;
    type Target: BridgeChain;
    type Bus: DynBus + Default;

    fn bus() -> Self::Bus {
        Self::Bus::default()
    }
}
