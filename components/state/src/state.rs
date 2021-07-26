use std::fmt::{Debug, Formatter};

use microkv::MicroKV;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::BridgeConfig;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

use crate::config::{BridgeStateConfig, MicrokvConfig};

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
    pub fn put_task_config_password(
        &self,
        task: impl AsRef<str>,
        password: impl AsRef<str>,
        store: bool,
    ) -> anyhow::Result<()> {
        let task = task.as_ref();
        let password = password.as_ref();
        crate::keep::put_task_config_password(task, password)?;
        if store {
            let key = format!("{}@password", task);
            self.microkv().put(key, &password.to_string())?;
        }
        Ok(())
    }
    pub fn get_task_config_password(
        &self,
        task: impl AsRef<str>,
    ) -> anyhow::Result<Option<String>> {
        let task = task.as_ref();
        let key = format!("{}@password", task);
        match self.microkv().get(key)? {
            Some(v) => Ok(Some(v)),
            None => crate::keep::get_task_config_password(task),
        }
    }
    pub fn get_task_config_password_unwrap_or_default(
        &self,
        task: impl AsRef<str>,
    ) -> anyhow::Result<String> {
        Ok(self.get_task_config_password(task)?.unwrap_or_default())
    }
}

impl Debug for BridgeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("BridgeState { microkv: <...> }")
    }
}
