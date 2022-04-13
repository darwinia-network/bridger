use std::marker::PhantomData;
use std::time::{SystemTime, UNIX_EPOCH};

use relay_substrate_client::{Chain, ChainBase};
use relay_utils::MaybeConnectionError;
use sp_runtime::traits::Saturating;
use sp_runtime::{FixedPointNumber, FixedU128};

use component_subscan::Subscan;

use crate::api::FeemarketApi;
use crate::error::{FeemarketError, FeemarketResult};
use crate::fee::UpdateFeeStrategy;

#[derive(Clone)]
pub struct ReasonableStrategy<AS: FeemarketApi, AT: FeemarketApi> {
    api_source: AS,
    api_target: AT,
    subscan_source: Subscan,
    subscan_target: Subscan,
    min_relay_fee_source: <AS::Chain as ChainBase>::Balance,
    min_relay_fee_target: <AT::Chain as ChainBase>::Balance,
}

impl<AS: FeemarketApi, AT: FeemarketApi> ReasonableStrategy<AS, AT> {
    pub fn new(
        api_source: AS,
        api_target: AT,
        subscan_source: Subscan,
        subscan_target: Subscan,
        min_relay_fee_source: u128,
        min_relay_fee_target: u128,
    ) -> FeemarketResult<Self> {
        let min_source = min_relay_fee_source
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        let min_target = min_relay_fee_target
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        Ok(Self {
            api_source,
            api_target,
            subscan_source,
            subscan_target,
            min_relay_fee_source: min_source,
            min_relay_fee_target: min_target,
        })
    }
}

#[async_trait::async_trait]
impl<AS: FeemarketApi, AT: FeemarketApi> UpdateFeeStrategy for ReasonableStrategy<AS, AT> {
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
            .unwrap_or(0)
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        let max_fee_target = top100_target
            .extrinsics
            .unwrap_or_default()
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0)
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;

        let conversion_balance = ConversionBalance::<AS::Chain, AT::Chain>::new(
            &self.subscan_source,
            &self.subscan_target,
        );

        let top100_max_cost_source = conversion_balance
            .conversion_target_to_source(max_fee_target)
            .await?
            .saturating_add(max_fee_source);
        let top100_max_cost_target = conversion_balance
            .conversion_source_to_target(max_fee_source)
            .await?
            .saturating_add(max_fee_target);

        // Nice (
        let mul_source: <AS::Chain as ChainBase>::Balance = 15u64
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        let expected_fee_source = top100_max_cost_source
            .saturating_mul(mul_source)
            .saturating_add(self.min_relay_fee_source);
        let mul_target: <AT::Chain as ChainBase>::Balance = 15u64
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        let expected_fee_target = top100_max_cost_target
            .saturating_mul(mul_target)
            .saturating_add(self.min_relay_fee_target);

        match self.update_source_fee(expected_fee_source).await {
            Err(FeemarketError::RelayClient(e)) => {
                if e.is_connection_error() {
                    tracing::debug!(
                        target: "feemarket",
                        "[feemarket] [reasonable] [{}] Lost rpc connection",
                        AS::Chain::NAME,
                    );
                    // if let Err(re) = self.helper.reconnect_pangoro().await {
                    //     tracing::error!(
                    //         target: "feemarket",
                    //         "[pangoro] Failed to reconnect substrate client: {:?} (update fee strategy)",
                    //         re
                    //     );
                    // }
                }
                return Err(e.into());
            }
            Err(e) => return Err(e),
            _ => {}
        }
        match self.update_target_fee(expected_fee_target).await {
            Err(FeemarketError::RelayClient(e)) => {
                if e.is_connection_error() {
                    tracing::debug!(
                        target: "feemarket",
                        "[feemarket] [reasonable] [{}] Lost rpc connection",
                        AT::Chain::NAME,
                    );
                    // if let Err(re) = self.helper.reconnect_pangoro().await {
                    //     tracing::error!(
                    //         target: "feemarket",
                    //         "[pangoro] Failed to reconnect substrate client: {:?} (update fee strategy)",
                    //         re
                    //     );
                    // }
                }
                return Err(e.into());
            }
            Err(e) => return Err(e),
            _ => {}
        }
        Ok(())
    }
}

impl<AS: FeemarketApi, AT: FeemarketApi> ReasonableStrategy<AS, AT> {
    async fn update_source_fee(
        &self,
        expected_fee_source: <AS::Chain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let efpu: u128 = expected_fee_source
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        tracing::info!(
            target: "ffeemarket",
            "[feemarket] [reasonable] [{}] Update pangolin fee: {}",
            AS::Chain::NAME,
            efpu,
        );
        self.api_source.update_relay_fee(expected_fee_source).await
    }

    async fn update_target_fee(
        &self,
        expected_fee_target: <AT::Chain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let efpu: u128 = expected_fee_target
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        tracing::info!(
            target: "feemarket",
            "[feemarket] [reasonable] [{}] Update pangoro fee: {}",
            AT::Chain::NAME,
            efpu,
        );
        self.api_target.update_relay_fee(expected_fee_target).await
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
        let ret: u128 = rate
            .saturating_mul_int(source_currency)
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        Ok(ret
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?)
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
        let ret: u128 = rate
            .saturating_mul_int(target_currency)
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        Ok(ret
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?)
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
