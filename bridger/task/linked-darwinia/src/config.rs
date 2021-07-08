use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::Config;
use component_darwinia::config::DarwiniaConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaLinkedConfig {
    pub darwinia: DarwiniaConfig,
}

impl DarwiniaLinkedConfig {
    pub fn store<S: AsRef<str>>(&self, sand_name: S) -> anyhow::Result<()> {
        let sand_name = sand_name.as_ref();
        Config::store(sand_name, self.darwinia.clone())?;
        Ok(())
    }
}
