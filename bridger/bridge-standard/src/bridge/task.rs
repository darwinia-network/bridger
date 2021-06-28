use std::fmt::Debug;

use lifeline::dyn_bus::DynBus;

use crate::bridge::chain::BridgeChain;
use crate::bridge::sand::BridgeSand;

pub trait BridgeTask: BridgeSand + Debug + Clone {
    type Source: BridgeChain;
    type Target: BridgeChain;
    type Bus: DynBus + Default;

    fn bus() -> Self::Bus {
        Self::Bus::default()
    }
}
