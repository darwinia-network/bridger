use bp_runtime::Chain;

use crate::error::AbstractFeemarketResult;
use crate::types::{LaneId, MessageNonce, Order, Relayer};

#[async_trait::async_trait]
pub trait FeemarketApiBase: 'static + Send + Sync + Clone {
    /// chain name
    const CHAIN: &'static str;

    /// chain types
    type Chain: Chain;

    /// best finalized block number
    async fn best_finalized_header_number(
        &self,
    ) -> AbstractFeemarketResult<<Self::Chain as Chain>::BlockNumber>;
}

/// Fee market api
#[async_trait::async_trait]
pub trait FeemarketApiRelay: FeemarketApiBase {
    /// order
    async fn order(
        &self,
        laned_id: LaneId,
        message_nonce: MessageNonce,
    ) -> AbstractFeemarketResult<
        Option<
            Order<
                <Self::Chain as Chain>::AccountId,
                <Self::Chain as Chain>::BlockNumber,
                <Self::Chain as Chain>::Balance,
            >,
        >,
    >;
}

#[async_trait::async_trait]
pub trait FeemarketApiQuote: FeemarketApiBase {
    /// Query assigned relayers
    async fn assigned_relayers(
        &self,
    ) -> AbstractFeemarketResult<
        Vec<Relayer<<Self::Chain as Chain>::AccountId, <Self::Chain as Chain>::Balance>>,
    >;

    async fn is_relayer(&self) -> AbstractFeemarketResult<bool>;

    /// all relayers
    async fn relayers(&self) -> AbstractFeemarketResult<Vec<<Self::Chain as Chain>::AccountId>>;

    /// Query relayer info by account id
    async fn relayer(
        &self,
        account: <Self::Chain as Chain>::AccountId,
    ) -> AbstractFeemarketResult<
        Option<Relayer<<Self::Chain as Chain>::AccountId, <Self::Chain as Chain>::Balance>>,
    >;

    /// Update relay fee
    async fn update_relay_fee(
        &self,
        amount: <Self::Chain as Chain>::Balance,
    ) -> AbstractFeemarketResult<()>;

    /// Update locked collateral
    async fn update_locked_collateral(
        &self,
        amount: <Self::Chain as Chain>::Balance,
    ) -> AbstractFeemarketResult<()>;
}
