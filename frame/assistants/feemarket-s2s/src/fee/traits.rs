use crate::error::FeemarketResult;

/// Update fee strategy
#[async_trait::async_trait]
pub trait UpdateFeeStrategy {
    async fn handle(&self) -> FeemarketResult<()>;
}
