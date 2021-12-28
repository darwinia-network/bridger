use std::fmt::{Debug, Formatter};

use microkv::namespace::NamespaceMicroKV;
use microkv::MicroKV;

use crate::config::BridgeStateConfig;

/// Bridge state component
#[derive(Clone)]
pub struct BridgeStateComponent;

impl BridgeStateComponent {
    pub fn component(config: BridgeStateConfig) -> color_eyre::Result<BridgeState> {
        let microkv = crate::microkv::microkv_instance(&config.microkv)?;
        Ok(BridgeState { microkv })
    }
}

#[derive(Clone)]
pub struct BridgeState {
    microkv: MicroKV,
}

lifeline::impl_storage_clone!(BridgeState);

impl BridgeState {
    pub fn microkv(&self) -> &MicroKV {
        &self.microkv
    }

    pub fn microkv_with_namespace(&self, namespace: impl AsRef<str>) -> NamespaceMicroKV {
        self.microkv.namespace(namespace)
    }
}

impl Debug for BridgeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("BridgeState { microkv: <...> }")
    }
}
