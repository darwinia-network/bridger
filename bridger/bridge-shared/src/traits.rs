use std::fmt::Debug;

use lifeline::dyn_bus::DynBus;

use bridge_standard::bridge::chain::BridgeChain;
use bridge_standard::bridge::sand::BridgeSand;

pub trait SharedKeep: Debug {}

pub trait SharedKeepService<M: SharedMaterial>: Debug {}

pub trait SharedMaterial: SharedKeep + BridgeSand + Clone + Debug {
    type Chain: BridgeChain;
    type Bus: DynBus + Default;

    fn bus() -> Self::Bus {
        Self::Bus::default()
    }
}
