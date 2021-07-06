use serde::{Deserialize, Serialize};

use bridge_component::config::BeeConfig;
use bridge_traits::bridge::config::Config;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaLinkedConfig {
    pub bee: BeeConfig,
}

impl DarwiniaLinkedConfig {
    pub fn store<S: AsRef<str>>(&self, sand_name: S) -> anyhow::Result<()> {
        let sand_name = sand_name.as_ref();
        Config::store(sand_name, self.bee.clone())?;
        Ok(())
    }
}
