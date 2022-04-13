use relay_substrate_client::{Chain, ChainBase, TransactionSignScheme};
use sp_core::Pair;

use crate::api::FeemarketApi;
use crate::error::{FeemarketError, FeemarketResult};
use crate::fee::UpdateFeeStrategy;

#[derive(Clone)]
pub struct CrazyStrategy<AS: FeemarketApi, AT: FeemarketApi> {
    api_source: AS,
    api_target: AT,
}

impl<AS: FeemarketApi, AT: FeemarketApi> CrazyStrategy<AS, AT> {
    pub fn new(api_source: AS, api_target: AT) -> Self {
        Self {
            api_source,
            api_target,
        }
    }
}

#[async_trait::async_trait]
impl<AS: FeemarketApi, AT: FeemarketApi> UpdateFeeStrategy for CrazyStrategy<AS, AT> {
    async fn handle(&self) -> FeemarketResult<()> {
        self.handle_source().await?;
        self.handle_target().await?;
        Ok(())
    }
}

impl<AS: FeemarketApi, AT: FeemarketApi> CrazyStrategy<AS, AT> {
    async fn handle_source(&self) -> FeemarketResult<()> {
        if !self.api_source.is_relayer().await? {
            tracing::warn!(
                target: "feemarket",
                "[femarket] [crazy] [{}] You are not a relayer, please register first",
                AS::Chain::NAME,
            );
            return Ok(());
        }

        // Query all assigned relayers
        let min_fee = match self.api_source.my_assigned_info().await? {
            Some((0, _)) => {
                return Ok(());
            }
            Some((i, relayer)) => relayer.fee,
            None => 51u32.into(),
        };

        // Nice (
        // RISK: If the cost is not judged, it may be a negative benefit.
        let new_fee = min_fee - 1u32.into();
        let num_balance: u64 = new_fee
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        tracing::info!(
            target: "feemarket",
            "[femarket] [crazy] [{}] Update pangolin fee: {}",
            AS::Chain::NAME,
            num_balance,
        );
        self.api_source.update_relay_fee(new_fee).await?;
        Ok(())
    }

    async fn handle_target(&self) -> FeemarketResult<()> {
        if !self.api_target.is_relayer().await? {
            tracing::warn!(
                target: "feemarket",
                "[femarket] [crazy] [{}] You are not a relayer, please register first",
                AT::Chain::NAME,
            );
            return Ok(());
        }

        // Query all assigned relayers
        let min_fee = match self.api_target.my_assigned_info().await? {
            Some((0, _)) => {
                return Ok(());
            }
            Some((i, relayer)) => relayer.fee,
            None => 51u32.into(),
        };

        // Nice (
        // RISK: If the cost is not judged, it may be a negative benefit.
        let new_fee = min_fee - 1u32.into();
        let num_balance: u64 = new_fee
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        tracing::info!(
            target: "feemarket",
            "[femarket] [crazy] [{}] Update pangolin fee: {}",
            AT::Chain::NAME,
            num_balance,
        );
        self.api_target.update_relay_fee(new_fee).await?;
        Ok(())
    }
}
