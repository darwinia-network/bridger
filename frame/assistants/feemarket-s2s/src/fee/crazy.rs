use relay_substrate_client::{Chain, ChainBase, TransactionSignScheme};
use sp_core::Pair;

use crate::api::FeemarketApi;
use crate::error::{FeemarketError, FeemarketResult};
use crate::fee::UpdateFeeStrategy;

#[derive(Clone)]
pub struct CrazyStrategy<A: FeemarketApi, T: TransactionSignScheme>
where
    <T::AccountKeyPair as Pair>::Public: Into<<A::Chain as ChainBase>::AccountId>,
{
    api: A,
    signer: T::AccountKeyPair,
}

impl<A: FeemarketApi, T: TransactionSignScheme> CrazyStrategy<A, T>
where
    <T::AccountKeyPair as Pair>::Public: Into<<A::Chain as ChainBase>::AccountId>,
{
    pub fn new(api: A, signer: T::AccountKeyPair) -> Self {
        Self { api, signer }
    }
}

#[async_trait::async_trait]
impl<A: FeemarketApi, T: TransactionSignScheme> UpdateFeeStrategy for CrazyStrategy<A, T>
where
    <T::AccountKeyPair as Pair>::Public: Into<<A::Chain as ChainBase>::AccountId>,
{
    async fn handle(&self) -> FeemarketResult<()> {
        // todo: may don't need this
        let my_id = self.signer.public().into();
        if self.api.is_relayer(my_id.clone()).await? {
            tracing::warn!(
                target: "feemarket",
                "[femarket] [crazy] [{}] You are not a relayer, please register first",
                A::Chain::NAME,
            );
            return Ok(());
        }

        // Query all assigned relayers
        let assigned_relayers = self.api.assigned_relayers().await?;
        let min_fee = match assigned_relayers.get(0) {
            Some(relayer) => {
                if relayer.id == my_id {
                    // If you are the first assigned relayer, no change will be made
                    return Ok(());
                }
                relayer.fee
            }
            None => 51u32.into(), // This is default value when not have any assigned relayers
        };

        // Nice (
        // RISK: If the cost is not judged, it may be a negative benefit.
        let new_fee = min_fee - 1u32.into();
        let num_balance: u64 = new_fee
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        tracing::info!(
            target: "pangolin-pangoro",
            "[femarket] [crazy] [{}] Update pangolin fee: {}",
            A::Chain::NAME,
            num_balance,
        );
        self.api.update_relay_fee(new_fee).await?;
        Ok(())
    }
}
