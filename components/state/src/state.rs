use std::fmt::{Debug, Formatter};

use microkv::namespace::NamespaceMicroKV;
use microkv::MicroKV;

use support_common::constants;

use crate::config::MicrokvConfig;

/// Bridger state
#[derive(Clone)]
pub struct BridgeState {
    microkv: MicroKV,
}

lifeline::impl_storage_clone!(BridgeState);

impl BridgeState {
    pub fn new() -> color_eyre::Result<Self> {
        let base_path = constants::bridger_home();
        let config_microkv = MicrokvConfig {
            base_path,
            db_name: Some("database".to_string()),
            auto_commit: true,
        };
        let store_path = &config_microkv.base_path.join("database.kv");
        tracing::debug!(
            target: "component-state",
            "KVDB PATH: {} and the auto_commit is opened",
            store_path.display()
        );
        let microkv = crate::microkv::microkv_instance(&config_microkv)?;
        Ok(Self { microkv })
    }
}

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
