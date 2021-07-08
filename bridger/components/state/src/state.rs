use std::fmt::{Debug, Formatter};

use microkv::MicroKV;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::BridgeConfig;

use crate::config::{BridgeStateConfig, MicrokvConfig};
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

#[derive(Clone)]
pub struct BridgeStateComponent {
    config: BridgeStateConfig,
}

impl BridgeStateComponent {
    pub fn new(config: BridgeStateConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<BridgeStateConfig, BridgeState> for BridgeStateComponent {
    fn restore_with_namespace<T: BridgeSand>(_namespace: String) -> BridgeResult<Self> {
        panic!("PANIC: THE BRIDGER STATE CAN NOT RESTORE FROM CONFIG, PLEASE INIT IT FROM PROGRAM ENTRYPOINT AND SHARE IT")
    }

    async fn component(&self) -> anyhow::Result<BridgeState> {
        let config_microkv = &self.config.microkv;
        let dbname = config_microkv
            .db_name
            .clone()
            .unwrap_or_else(|| MicrokvConfig::marker().to_string());
        let kv = MicroKV::open_with_base_path(dbname, config_microkv.base_path.clone())?
            .set_auto_commit(config_microkv.auto_commit);
        Ok(BridgeState { microkv: kv })
    }

    fn config(&self) -> &BridgeStateConfig {
        &self.config
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
}

impl Debug for BridgeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("BridgeState { microkv: <...> }")
    }
}
