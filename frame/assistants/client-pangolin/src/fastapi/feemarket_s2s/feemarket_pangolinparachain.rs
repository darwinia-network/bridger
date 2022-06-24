use abstract_feemarket_s2s::api::FeemarketApiRelay;
use abstract_feemarket_s2s::error::AbstractFeemarketResult;
use abstract_feemarket_s2s::types::{Chain, Order};
use support_toolkit::convert::SmartCodecMapper;

use crate::client::PangolinClient;

#[async_trait::async_trait]
impl FeemarketApiRelay for PangolinClient {
    async fn order(
        &self,
        laned_id: abstract_feemarket_s2s::types::LaneId,
        message_nonce: abstract_feemarket_s2s::types::MessageNonce,
    ) -> AbstractFeemarketResult<
        Option<
            Order<
                <Self::Chain as Chain>::AccountId,
                <Self::Chain as Chain>::BlockNumber,
                <Self::Chain as Chain>::Balance,
            >,
        >,
    > {
        match self
            .runtime()
            .storage()
            .pangolin_parachain_fee_market()
            .orders(laned_id, message_nonce, None)
            .await?
        {
            Some(v) => Ok(Some(SmartCodecMapper::map_to(&v)?)),
            None => Ok(None),
        }
    }
}
