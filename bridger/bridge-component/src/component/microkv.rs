use bridge_config::config::component::MicrokvConfig;
use bridge_standard::bridge::component::BridgeComponent;
use bridge_standard::bridge::config::BridgeConfig;
use microkv::MicroKV;

#[derive(Clone, Debug, Default)]
pub struct MicrokvComponent {
    config: MicrokvConfig,
}

impl MicrokvComponent {
    pub fn new(config: MicrokvConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl BridgeComponent<MicrokvConfig, MicroKV> for MicrokvComponent {
    async fn component(&self) -> anyhow::Result<MicroKV> {
        let dbname = self
            .config
            .db_name
            .clone()
            .unwrap_or_else(|| MicrokvConfig::marker().to_string());
        let kv = MicroKV::open_with_base_path(dbname, self.config.base_path.clone())?
            .set_auto_commit(self.config.auto_commit);
        Ok(kv)
    }

    fn config(&self) -> &MicrokvConfig {
        &self.config
    }
}
