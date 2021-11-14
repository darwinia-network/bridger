use std::time::Duration;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

pub use self::config::*;
pub use self::subscan::*;

mod config;
mod subscan;
pub mod types;

#[derive(Clone, Debug, Default)]
pub struct SubscanComponent {
    config: SubscanConfig,
}

impl SubscanComponent {
    pub fn new(config: SubscanConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<SubscanConfig, Subscan> for SubscanComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self>
    where
        Self: Sized,
    {
        let config: SubscanConfig = Config::restore_with_namespace_unwrap(T::NAME, &namespace)?;
        Ok(Self::new(config))
    }

    async fn component(&self) -> anyhow::Result<Subscan> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.config.timeout.unwrap_or(3000)))
            .build()?;
        let subscan = Subscan::new(
            client,
            self.config.endpoint.clone(),
            self.config.token.clone(),
        );
        Ok(subscan)
    }

    fn config(&self) -> &SubscanConfig {
        &self.config
    }
}
