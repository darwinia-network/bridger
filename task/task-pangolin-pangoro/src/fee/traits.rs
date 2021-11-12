#[async_trait::async_trait]
pub trait UpdateFeeStrategy {
    async fn handle(&self) -> anyhow::Result<()>;
}
