use crate::error::FeemarketResult;
use crate::fee::UpdateFeeStrategy;

#[derive(Clone)]
pub struct NothingStrategy;

#[async_trait::async_trait]
impl UpdateFeeStrategy for NothingStrategy {
    async fn handle(&self) -> FeemarketResult<()> {
        Ok(())
    }
}
