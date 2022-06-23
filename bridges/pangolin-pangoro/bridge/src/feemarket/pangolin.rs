use abstract_bridge_s2s::client::S2SClientGeneric;
use abstract_bridge_s2s::config::Config;
use abstract_bridge_s2s::types::bp_messages::{LaneId, MessageNonce};
use feemarket_ns2s::api::FeemarketApi;
use feemarket_ns2s::error::FeemarketResult;
use feemarket_ns2s::types::{Order, Relayer};

#[derive(Clone)]
pub struct PangolinFeemarketApi {
    pub lane_id: LaneId,
}

#[async_trait::async_trait]
impl<C: S2SClientGeneric> FeemarketApi<C> for PangolinFeemarketApi {
    fn lane_id(&self) -> LaneId {
        self.lane_id
    }

    async fn assigned_relayers(
        &self,
        client: &C,
    ) -> FeemarketResult<
        Vec<Relayer<<C::Config as Config>::AccountId, <C::Config as Config>::Balance>>,
    > {
        todo!()
    }

    async fn order(
        &self,
        client: &C,
        laned_id: LaneId,
        message_nonce: MessageNonce,
    ) -> FeemarketResult<
        Option<
            Order<
                <C::Config as Config>::AccountId,
                <C::Config as Config>::BlockNumber,
                <C::Config as Config>::Balance,
            >,
        >,
    > {
        todo!()
    }

    async fn is_relayer(&self, client: &C) -> FeemarketResult<bool> {
        todo!()
    }

    async fn relayers(&self, client: &C) -> FeemarketResult<Vec<<C::Config as Config>::AccountId>> {
        todo!()
    }

    async fn relayer(
        &self,
        client: &C,
        account: <C::Config as Config>::AccountId,
    ) -> FeemarketResult<
        Option<Relayer<<C::Config as Config>::AccountId, <C::Config as Config>::Balance>>,
    > {
        todo!()
    }

    async fn update_relay_fee(
        &self,
        client: &C,
        amount: <C::Config as Config>::Balance,
    ) -> FeemarketResult<()> {
        todo!()
    }

    async fn update_locked_collateral(
        &self,
        client: &C,
        amount: <C::Config as Config>::Balance,
    ) -> FeemarketResult<()> {
        todo!()
    }
}
