use std::marker::PhantomData;
use std::time::{SystemTime, UNIX_EPOCH};

use relay_substrate_client::{Chain, ChainBase, TransactionSignScheme};
use sp_core::Pair;
use sp_runtime::{FixedPointNumber, FixedU128};

use component_subscan::Subscan;

use crate::api::FeemarketApi;
use crate::error::{FeemarketError, FeemarketResult};
use crate::fee::UpdateFeeStrategy;

#[derive(Clone)]
pub struct ReasonStrategy<
    AS: FeemarketApi,
    AT: FeemarketApi,
    TS: TransactionSignScheme,
    TT: TransactionSignScheme,
> where
    <TS::AccountKeyPair as Pair>::Public: Into<<AS::Chain as ChainBase>::AccountId>,
    <TT::AccountKeyPair as Pair>::Public: Into<<AT::Chain as ChainBase>::AccountId>,
{
    api_source: AS,
    api_target: AT,
    signer_source: TS::AccountKeyPair,
    signer_target: TT::AccountKeyPair,
    subscan_source: Subscan,
    subscan_target: Subscan,
}

#[async_trait::async_trait]
impl<AS: FeemarketApi, AT: FeemarketApi, TS: TransactionSignScheme, TT: TransactionSignScheme>
    UpdateFeeStrategy for ReasonStrategy<AS, AT, TS, TT>
where
    <TS::AccountKeyPair as Pair>::Public: Into<<AS::Chain as ChainBase>::AccountId>,
    <TT::AccountKeyPair as Pair>::Public: Into<<AT::Chain as ChainBase>::AccountId>,
{
    async fn handle(&self) -> FeemarketResult<()> {
        let top100_source = self.subscan_source.extrinsics(1, 100).await?;
        let top100_target = self.subscan_target.extrinsics(1, 100).await?;
        let top100_source = top100_source.data()?.ok_or_else(|| {
            FeemarketError::Custom("Can not query pangolin extrinsics data".to_string())
        })?;
        let top100_target = top100_target.data()?.ok_or_else(|| {
            FeemarketError::Custom("Can not query pangoro extrinsics data".to_string())
        })?;

        let max_fee_source = top100_source
            .extrinsics
            .unwrap_or_default()
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0);
        let max_fee_target = top100_target
            .extrinsics
            .unwrap_or_default()
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0);
        Ok(())
    }
}

struct ConversionBalance<'a, SC: Chain, TC: Chain> {
    _marker0: PhantomData<SC>,
    _marker1: PhantomData<TC>,
    subscan_source: &'a Subscan,
    subscan_target: &'a Subscan,
}

impl<'a, SC: Chain, TC: Chain> ConversionBalance<'a, SC, TC> {
    pub fn new(subscan_source: &'a Subscan, subscan_target: &'a Subscan) -> Self {
        Self {
            _marker0: Default::default(),
            _marker1: Default::default(),
            subscan_source,
            subscan_target,
        }
    }

    /// conversion source chain balance to target chain balance
    pub async fn conversion_source_to_target(
        &self,
        source_currency: SC::Balance,
    ) -> FeemarketResult<TC::Balance> {
        let price_source = self.source_open_price().await?;
        let price_target = self.target_open_price().await?;
        let rate = price_source / price_target;
        let rate = FixedU128::from_float(rate);
        let ret = rate.saturating_mul_int(source_currency);
        Ok(ret.try_into())
    }

    /// conversion target chain balance to source chain balance
    pub async fn conversion_target_to_source(
        &self,
        target_currency: TC::Balance,
    ) -> FeemarketResult<SC::Balance> {
        let price_source = self.source_open_price().await?;
        let price_target = self.target_open_price().await?;
        let rate = price_target / price_source;
        let rate = FixedU128::from_float(rate);
        let ret = rate.saturating_mul_int(target_currency);
        Ok(ret)
    }

    async fn source_open_price(&self) -> FeemarketResult<f64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| FeemarketError::Custom(format!("{:?}", e)))?
            .as_secs();
        match self.subscan_source.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan = self
                    .subscan_source
                    .clone()
                    .endpoint("https://darwinia.api.subscan.io");
                let open_price = subscan.price(now).await?;
                let data = open_price.data()?;
                match data {
                    Some(v) => Ok(v.price),
                    None => Err(FeemarketError::Custom(
                        "Can not query pangolin price".to_string(),
                    )),
                }
            }
        }
    }

    async fn target_open_price(&self) -> FeemarketResult<f64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| FeemarketError::Custom(format!("{:?}", e)))?
            .as_secs();
        match self.subscan_target.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan = self
                    .subscan_target
                    .clone()
                    .endpoint("https://dock.api.subscan.io");
                let open_price = subscan.price(now).await?;
                let data = open_price.data()?;
                match data {
                    Some(v) => Ok(v.price),
                    None => Err(FeemarketError::Custom(
                        "Can not query pangolin price".to_string(),
                    )),
                }
            }
        }
    }
}
