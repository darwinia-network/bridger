use bp_messages::{LaneId, MessageNonce};
use dp_fee::{Order, Relayer};
use relay_substrate_client::{Chain, ChainBase};

use crate::error::FeemarketResult;

#[async_trait::async_trait]
pub trait FeemarketApi: 'static + Send + Sync + Clone {
    type Chain: Chain;

    fn lane_id(&self) -> LaneId;

    /// Return number of the best finalized block.
    async fn best_finalized_header_number(
        &self,
    ) -> FeemarketResult<<Self::Chain as ChainBase>::BlockNumber>;

    /// Query assigned relayers
    async fn assigned_relayers(
        &self,
    ) -> FeemarketResult<
        Vec<Relayer<<Self::Chain as ChainBase>::AccountId, <Self::Chain as ChainBase>::Balance>>,
    >;

    /// Query order
    async fn order(
        &self,
        laned_id: LaneId,
        message_nonce: MessageNonce,
    ) -> FeemarketResult<
        Option<
            Order<
                <Self::Chain as ChainBase>::AccountId,
                <Self::Chain as ChainBase>::BlockNumber,
                <Self::Chain as ChainBase>::Balance,
            >,
        >,
    >;

    /// Query all relayers
    async fn relayers(&self) -> FeemarketResult<Vec<<Self::Chain as ChainBase>::AccountId>>;

    /// Query relayer info by account id
    async fn relayer(
        &self,
        account: <Self::Chain as ChainBase>::AccountId,
    ) -> FeemarketResult<
        Option<Relayer<<Self::Chain as ChainBase>::AccountId, <Self::Chain as ChainBase>::Balance>>,
    >;

    async fn is_relayer(
        &self,
        account: <Self::Chain as ChainBase>::AccountId,
    ) -> FeemarketResult<bool> {
        self.relayer(account).await.map(|item| item.is_some())
    }

    /// Update relay fee
    async fn update_relay_fee(
        &self,
        // signer: <Self::Chain as TransactionSignScheme>::AccountKeyPair,
        amount: <Self::Chain as ChainBase>::Balance,
    ) -> FeemarketResult<()>;

    /// Update locked collateral
    async fn update_locked_collateral(
        &self,
        // signer: <Self::Chain as TransactionSignScheme>::AccountKeyPair,
        amount: <Self::Chain as ChainBase>::Balance,
    ) -> FeemarketResult<()>;
}
