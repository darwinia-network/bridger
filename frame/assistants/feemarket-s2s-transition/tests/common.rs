use bp_messages::{LaneId, MessageNonce};
use darwinia_fee_market::{Order, Relayer};
use relay_substrate_client::{ChainBase, Client};

use feemarket_s2s_transition::api::FeemarketApi;
use feemarket_s2s_transition::error::FeemarketResult;

#[derive(Clone)]
pub struct TestFeemarketApi;

#[async_trait::async_trait]
impl FeemarketApi for TestFeemarketApi {
    type Chain = relay_pangolin_client::PangolinChain;

    fn lane_id(&self) -> LaneId {
        [0, 1, 2, 3]
    }

    async fn best_finalized_header_number(&self) -> FeemarketResult<BlockNumber> {
        Ok(1u32.into())
    }

    async fn assigned_relayers(
        &self,
    ) -> FeemarketResult<
        Vec<Relayer<<Self::Chain as ChainBase>::AccountId, <Self::Chain as ChainBase>::Balance>>,
    > {
        Ok(Vec::new())
    }

    async fn order(
        &self,
        _laned_id: LaneId,
        _message_nonce: MessageNonce,
    ) -> FeemarketResult<
        Option<
            Order<
                <Self::Chain as ChainBase>::AccountId,
                <Self::Chain as ChainBase>::BlockNumber,
                <Self::Chain as ChainBase>::Balance,
            >,
        >,
    > {
        Ok(None)
    }

    async fn relayers(&self) -> FeemarketResult<Vec<<Self::Chain as ChainBase>::AccountId>> {
        Ok(Vec::new())
    }

    async fn relayer(
        &self,
        _account: <Self::Chain as ChainBase>::AccountId,
    ) -> FeemarketResult<
        Option<Relayer<<Self::Chain as ChainBase>::AccountId, <Self::Chain as ChainBase>::Balance>>,
    > {
        Ok(None)
    }

    async fn update_relay_fee(
        &self,
        _amount: <Self::Chain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        Ok(())
    }

    async fn update_locked_collateral(
        &self,
        _amount: <Self::Chain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        Ok(())
    }
}
