use bee_client::ws::BeeWebsocket;

use crate::config::DarwiniaConfig;
use crate::types::DarwiniaChainTypes;

#[derive(Clone, Debug)]
pub struct DarwiniaClient {
    bee: BeeWebsocket<DarwiniaChainTypes>,
}

impl DarwiniaClient {
    pub async fn new(config: DarwiniaConfig) -> anyhow::Result<Self> {
        let bee = bee_client::Bee::websocket(config.endpoint, config.strict).await?;
        Ok(Self { bee })
    }
}
