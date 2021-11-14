use crate::fee::UpdateFeeStrategy;

pub struct NothingStrategy;

#[async_trait::async_trait]
impl UpdateFeeStrategy for NothingStrategy {
    async fn handle(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
