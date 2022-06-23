use abstract_feemarket_s2s::api::FeemarketApiBase;
use abstract_feemarket_s2s::error::{AbstractFeemarketError, AbstractFeemarketResult};
use abstract_feemarket_s2s::types::Chain;

use crate::client::PangolinClient;

#[async_trait::async_trait]
impl FeemarketApiBase for PangolinClient {
    const CHAIN: &'static str = "pangolin";

    type Chain = bp_pangolin::Pangolin;

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
