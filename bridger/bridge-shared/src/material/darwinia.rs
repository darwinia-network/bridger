use bridge_standard::bridge::sand::BridgeSand;
use chain_darwinia::DarwiniaChain;

use crate::bus::SharedBus;
use crate::traits::{SharedChainMaterial, SharedMaterial};

#[derive(Clone, Debug)]
pub struct MaterialDarwinia {}

impl BridgeSand for MaterialDarwinia {
    const NAME: &'static str = "shared-darwinia";
}

impl SharedMaterial for MaterialDarwinia {
    type Bus = SharedBus;
}

impl SharedChainMaterial for MaterialDarwinia {
    type Chain = DarwiniaChain;
}
