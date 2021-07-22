use crate::bridge::config::{BridgeConfig, Config};
use crate::bridge::task::BridgeSand;
use crate::error::BridgeResult;

#[async_trait::async_trait]
pub trait BridgeComponent<C: BridgeConfig, R> {
    fn restore<T: BridgeSand>() -> BridgeResult<Self>
    where
        Self: Sized,
    {
        Self::restore_with_namespace::<T>(Config::default_namespace().to_string())
    }
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self>
    where
        Self: Sized;

    async fn component(&self) -> anyhow::Result<R>;
    fn config(&self) -> &C;
}
