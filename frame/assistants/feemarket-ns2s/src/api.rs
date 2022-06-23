use abstract_bridge_s2s::client::S2SClientGeneric;
use abstract_bridge_s2s::config::Config;
use abstract_bridge_s2s::types::bp_messages::{LaneId, MessageNonce};

use crate::error::FeemarketResult;
use crate::types::{Order, Relayer};

/// Fee market api
#[async_trait::async_trait]
pub trait FeemarketApi<C: S2SClientGeneric>: 'static + Send + Sync + Clone {
    /// Query assigned relayers
    async fn assigned_relayers(
        &self,
        client: &C,
    ) -> FeemarketResult<
        Vec<Relayer<<C::Config as Config>::AccountId, <C::Config as Config>::Balance>>,
    >;

    /// order
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
    >;

    async fn is_relayer(&self, client: &C) -> FeemarketResult<bool>;

    /// all relayers
    async fn relayers(&self, client: &C) -> FeemarketResult<Vec<<C::Config as Config>::AccountId>>;

    /// Query relayer info by account id
    async fn relayer(
        &self,
        client: &C,
        account: <C::Config as Config>::AccountId,
    ) -> FeemarketResult<
        Option<Relayer<<C::Config as Config>::AccountId, <C::Config as Config>::Balance>>,
    >;

    /// Update relay fee
    async fn update_relay_fee(
        &self,
        client: &C,
        amount: <C::Config as Config>::Balance,
    ) -> FeemarketResult<()>;

    /// Update locked collateral
    async fn update_locked_collateral(
        &self,
        client: &C,
        amount: <C::Config as Config>::Balance,
    ) -> FeemarketResult<()>;
}
