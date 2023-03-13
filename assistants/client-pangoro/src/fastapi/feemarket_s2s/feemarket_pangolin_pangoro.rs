use feemarket_s2s_traits::api::FeemarketApiRelay;
use feemarket_s2s_traits::error::AbstractFeemarketResult;
use feemarket_s2s_traits::types::{Chain, Order};
use support_toolkit::convert::SmartCodecMapper;

use crate::client::PangoroClient;

#[async_trait::async_trait]
impl FeemarketApiRelay for PangoroClient {
    async fn order(
        &self,
        lane_id: feemarket_s2s_traits::types::LaneId,
        message_nonce: feemarket_s2s_traits::types::MessageNonce,
    ) -> AbstractFeemarketResult<
        Option<
            Order<
                <Self::Chain as Chain>::AccountId,
                <Self::Chain as Chain>::BlockNumber,
                <Self::Chain as Chain>::Balance,
            >,
        >,
    > {
        let address = crate::subxt_runtime::api::storage()
            .pangolin_fee_market()
            .orders(lane_id, message_nonce);

        match self.subxt().storage().fetch(&address, None).await? {
            Some(v) => Ok(Some(SmartCodecMapper::map_to(&v)?)),
            None => Ok(None),
        }
    }
}
