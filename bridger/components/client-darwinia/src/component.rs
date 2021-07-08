use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

use crate::config::DarwiniaConfig;
use crate::darwinia::DarwiniaClient;

pub struct DarwiniaComponent {
    config: DarwiniaConfig,
}

impl DarwiniaComponent {
    pub fn new(config: DarwiniaConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<DarwiniaConfig, DarwiniaClient> for DarwiniaComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self> {
        let config: DarwiniaConfig = Config::restore_with_namespace(T::NAME, namespace)?;
        Ok(Self::new(config))
    }

    async fn component(&self) -> anyhow::Result<DarwiniaClient> {
        DarwiniaClient::new(self.config.clone()).await
    }

    fn config(&self) -> &DarwiniaConfig {
        &self.config
    }
}
