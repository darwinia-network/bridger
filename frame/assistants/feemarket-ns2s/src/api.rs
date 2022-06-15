use crate::error::FeemarketResult;
use bp_messages::{LaneId, MessageNonce};
use bp_runtime::Chain;
use pallet_fee_market::types::{Order, Relayer};

/// Fee market api
#[async_trait::async_trait]
pub trait FeemarketApi: 'static + Send + Sync + Clone {
    type Chain: Chain;

    /// Lane id
    fn lane_id(&self) -> LaneId;

    /// Return number of the best finalized block.
    async fn best_finalized_header_number(
        &self,
    ) -> FeemarketResult<<Self::Chain as Chain>::BlockNumber>;

    /// Query assigned relayers
    async fn assigned_relayers(
        &self,
    ) -> FeemarketResult<
        Vec<Relayer<<Self::Chain as Chain>::AccountId, <Self::Chain as Chain>::Balance>>,
    >;
}
