use crate::bridge::config::BridgeConfig;

#[async_trait]
pub trait BridgeComponent<C: BridgeConfig, R> {
    async fn component(&self) -> anyhow::Result<R>;
    fn config(&self) -> &C;
}
