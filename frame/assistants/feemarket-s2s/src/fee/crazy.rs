use relay_substrate_client::Chain;

use crate::api::FeemarketApi;
use crate::error::{FeemarketError, FeemarketResult};
use crate::fee::UpdateFeeStrategy;

#[derive(Clone)]
pub struct CrazyStrategy<AS: FeemarketApi, AT: FeemarketApi> {
    api_left: AS,
    api_right: AT,
}

impl<AS: FeemarketApi, AT: FeemarketApi> CrazyStrategy<AS, AT> {
    pub fn new(api_left: AS, api_right: AT) -> Self {
        Self {
            api_left,
            api_right,
        }
    }
}

#[async_trait::async_trait]
impl<AS: FeemarketApi, AT: FeemarketApi> UpdateFeeStrategy for CrazyStrategy<AS, AT> {
    async fn handle(&self) -> FeemarketResult<()> {
        tracing::trace!(
            target: "feemarket",
            "[femarket] [crazy] Start update fee",
        );
        self.handle_source().await?;
        self.handle_target().await?;
        tracing::trace!(
            target: "feemarket",
            "[femarket] [crazy] Fee updated",
        );
        Ok(())
    }
}

impl<AS: FeemarketApi, AT: FeemarketApi> CrazyStrategy<AS, AT> {
    async fn handle_source(&self) -> FeemarketResult<()> {
        if !self.api_left.is_relayer().await? {
            tracing::warn!(
                target: "feemarket",
                "[femarket] [crazy] [{}] You are not a relayer, please register first",
                AS::Chain::NAME,
            );
            return Ok(());
        }

        // Query all assigned relayers
        let min_fee = match self.api_left.my_assigned_info().await? {
            Some((0, _)) => {
                tracing::info!(
                    target: "feemarket",
                    "[femarket] [crazy] [{}] You are first assigned relayer, nothing to do",
                    AS::Chain::NAME,
                );
                return Ok(());
            }
            Some((_i, relayer)) => relayer.fee,
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
        self.api_left.update_relay_fee(new_fee).await?;
        Ok(())
    }

    async fn handle_target(&self) -> FeemarketResult<()> {
        if !self.api_right.is_relayer().await? {
            tracing::warn!(
                target: "feemarket",
                "[femarket] [crazy] [{}] You are not a relayer, please register first",
                AT::Chain::NAME,
            );
            return Ok(());
        }

        // Query all assigned relayers
        let min_fee = match self.api_right.my_assigned_info().await? {
            Some((0, _)) => {
                tracing::info!(
                    target: "feemarket",
                    "[femarket] [crazy] [{}] You are first assigned relayer, nothing to do",
                    AS::Chain::NAME,
                );
                return Ok(());
            }
            Some((_i, relayer)) => relayer.fee,
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
        self.api_right.update_relay_fee(new_fee).await?;
        Ok(())
    }
}
