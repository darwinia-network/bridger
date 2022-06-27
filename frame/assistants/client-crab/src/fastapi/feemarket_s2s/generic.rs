use abstract_feemarket_s2s::api::FeemarketApiBase;
use abstract_feemarket_s2s::error::{AbstractFeemarketError, AbstractFeemarketResult};
use abstract_feemarket_s2s::types::Chain;

use crate::client::CrabClient;

#[async_trait::async_trait]
impl FeemarketApiBase for CrabClient {
    const CHAIN: &'static str = "crab";

    type Chain = bp_crab::Crab;

    async fn best_finalized_header_number(
        &self,
    ) -> AbstractFeemarketResult<<Self::Chain as Chain>::BlockNumber> {
        let head_hash = self.subxt().rpc().finalized_head().await?;
        let header = self
            .subxt()
            .rpc()
            .header(Some(head_hash))
            .await?
            .ok_or_else(|| {
                AbstractFeemarketError::Custom("Can not query best finalized header".to_string())
            })?;
        Ok(header.number)
    }
}
