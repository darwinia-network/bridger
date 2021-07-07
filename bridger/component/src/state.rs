use std::fmt::{Debug, Formatter};

use microkv::MicroKV;

use bridge_traits::bridge::component::BridgeComponent;

use crate::component::microkv::MicrokvComponent;
use crate::config::StateConfig;

#[derive(Clone)]
pub struct BridgeState {
    microkv: MicroKV,
}

lifeline::impl_storage_clone!(BridgeState);

impl Debug for BridgeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("BridgeState { microkv: <microkv> }")
    }
}

impl BridgeState {
    pub async fn new(config: StateConfig) -> anyhow::Result<Self> {
        let component_microkv = MicrokvComponent::new(config.microkv);
        let microkv = component_microkv.component().await?;
        Ok(Self { microkv })
    }
}

impl BridgeState {
    /// Get microkv instance reference
    /// todo: wrapper microkv, add special prefix for key
    pub fn microkv(&self) -> &MicroKV {
        &self.microkv
    }
}
