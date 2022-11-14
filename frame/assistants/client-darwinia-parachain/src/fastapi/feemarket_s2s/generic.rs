use feemarket_s2s_traits::api::FeemarketApiBase;
use feemarket_s2s_traits::error::{AbstractFeemarketError, AbstractFeemarketResult};
use feemarket_s2s_traits::types::Chain;

use crate::client::DarwiniaParachainClient;

#[async_trait::async_trait]
impl FeemarketApiBase for DarwiniaParachainClient {
    const CHAIN: &'static str = "darwiniaparachain";

    type Chain = bp_darwinia_parachain::DarwiniaParachain;

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
