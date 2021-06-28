use bridge_config::config::component::BeeConfig;
use bridge_config::Config;
use bridge_standard::bridge::config::BridgeConfig;
use bridge_standard::bridge::sand::BridgeSand;

#[derive(Clone, Debug)]
pub struct SharedConfig {
    pub service_darwinia: DarwiniaServiceConfig,
}

#[derive(Clone, Debug)]
pub struct DarwiniaServiceConfig {
    pub bee: BeeConfig,
}

impl DarwiniaServiceConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        Config::store(cell_name.as_ref(), self.bee.clone())?;
        Ok(())
    }
}

impl SharedConfig {
    pub fn store_darwinia<S: BridgeSand>(&self) -> anyhow::Result<()> {
        self.service_darwinia.store(S::NAME)
    }
}
