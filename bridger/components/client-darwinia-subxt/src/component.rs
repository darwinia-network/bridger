use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

use crate::config::DarwiniaSubxtConfig;
use crate::darwinia::client::Darwinia;

pub struct DarwiniaSubxtComponent {
    config: DarwiniaSubxtConfig,
}

impl DarwiniaSubxtComponent {
    pub fn new(config: DarwiniaSubxtConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<DarwiniaSubxtConfig, Darwinia> for DarwiniaSubxtComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self> {
        let config: DarwiniaSubxtConfig = Config::restore_with_namespace(T::NAME, namespace)?;
        Ok(Self::new(config))
    }

    async fn component(&self) -> anyhow::Result<Darwinia> {
        Ok(Darwinia::new(self.config.endpoint.clone()).await?)
    }

    fn config(&self) -> &DarwiniaSubxtConfig {
        &self.config
    }
}
