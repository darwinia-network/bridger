use std::fmt::Debug;

use lifeline::dyn_bus::DynBus;

use bridge_standard::bridge::chain::BridgeChain;
use bridge_standard::bridge::sand::BridgeSand;

pub trait SharedKeepService: Debug {}

pub trait SharedMaterial: BridgeSand + Clone + Debug {
    type Chain: BridgeChain;
    type Bus: DynBus + Default;

    fn bus() -> Self::Bus {
        Self::Bus::default()
    }
}
