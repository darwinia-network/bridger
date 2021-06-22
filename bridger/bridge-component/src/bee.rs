use bee_client::types::client::ChainTypes;
use bee_client::ws::BeeWebsocket;
use bridge_config::component::BeeConfig;
use bridge_standard::bridge::component::BridgeComponent;
use std::marker::PhantomData;

pub struct BeeComponent<T: ChainTypes> {
    config: BeeConfig,
    _marker: PhantomData<T>,
}

impl<T: ChainTypes> BeeComponent<T> {
    pub fn new(config: BeeConfig) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            _marker: Default::default(),
        })
    }
}

#[async_trait]
impl<T: ChainTypes> BridgeComponent<BeeConfig, BeeWebsocket<T>> for BeeComponent<T> {
    async fn component(&self) -> anyhow::Result<BeeWebsocket<T>> {
        let client =
            bee_client::Bee::websocket(self.config.endpoint.clone(), self.config.strict).await?;
        Ok(client)
    }

    fn config(&self) -> &BeeConfig {
        &self.config
    }
}
