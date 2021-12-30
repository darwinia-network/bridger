#[async_trait::async_trait]
pub trait UpdateFeeStrategy {
    async fn handle(&mut self) -> color_eyre::Result<()>;
}
