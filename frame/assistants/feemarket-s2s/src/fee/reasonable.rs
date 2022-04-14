use std::marker::PhantomData;
use std::time::{SystemTime, UNIX_EPOCH};

use relay_substrate_client::{Chain, ChainBase};
use relay_utils::MaybeConnectionError;
use sp_runtime::traits::Saturating;
use sp_runtime::{FixedPointNumber, FixedU128};

use component_subscan::{Subscan, SubscanComponent};

use crate::api::FeemarketApi;
use crate::config::FeemarketConfig;
use crate::error::{FeemarketError, FeemarketResult};
use crate::fee::UpdateFeeStrategy;

#[derive(Clone)]
pub struct ReasonableStrategy<AS: FeemarketApi, AT: FeemarketApi> {
    api_left: AS,
    api_right: AT,
    subscan_left: Subscan,
    subscan_right: Subscan,
    min_relay_fee_left: <AS::Chain as ChainBase>::Balance,
    min_relay_fee_right: <AT::Chain as ChainBase>::Balance,
}

impl<AS: FeemarketApi, AT: FeemarketApi> ReasonableStrategy<AS, AT> {
    pub fn new(
        feemarket_config: FeemarketConfig,
        api_left: AS,
        api_right: AT,
        min_relay_fee_left: u128,
        min_relay_fee_right: u128,
    ) -> FeemarketResult<Self> {
        let subscan_config_left = feemarket_config.subscan_left;
        let subscan_config_right = feemarket_config.subscan_right;
        let exists_subscan_config = subscan_config_left.is_some() && subscan_config_right.is_some();
        if !exists_subscan_config {
            return Err(FeemarketError::Custom("Missing subscan config".to_string()));
        }

        let min_source = min_relay_fee_left
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        let min_target = min_relay_fee_right
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;

        let subscan_left = SubscanComponent::component(subscan_config_left.unwrap())?;
        let subscan_right = SubscanComponent::component(subscan_config_right.unwrap())?;

        Ok(Self {
            api_left,
            api_right,
            subscan_left,
            subscan_right,
            min_relay_fee_left: min_source,
            min_relay_fee_right: min_target,
        })
    }
}

#[async_trait::async_trait]
impl<AS: FeemarketApi, AT: FeemarketApi> UpdateFeeStrategy for ReasonableStrategy<AS, AT> {
    async fn handle(&self) -> FeemarketResult<()> {
        let top100_source = self.subscan_left.extrinsics(1, 100).await?;
        let top100_target = self.subscan_right.extrinsics(1, 100).await?;
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

        let conversion_balance =
            ConversionBalance::<AS::Chain, AT::Chain>::new(&self.subscan_left, &self.subscan_right);

        let top100_max_cost_source = conversion_balance
            .conversion_right_to_left(max_fee_target)
            .await?
            .saturating_add(max_fee_source);
        let top100_max_cost_target = conversion_balance
            .conversion_left_to_right(max_fee_source)
            .await?
            .saturating_add(max_fee_target);

        // Nice (
        let mul_source: <AS::Chain as ChainBase>::Balance = 15u64
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        let expected_fee_source = top100_max_cost_source
            .saturating_mul(mul_source)
            .saturating_add(self.min_relay_fee_left);
        let mul_target: <AT::Chain as ChainBase>::Balance = 15u64
            .try_into()
            .map_err(|_e| FeemarketError::WrongConvert("Wrong balance".to_string()))?;
        let expected_fee_target = top100_max_cost_target
            .saturating_mul(mul_target)
            .saturating_add(self.min_relay_fee_right);

        match self.update_left_fee(expected_fee_source).await {
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
        match self.update_right_fee(expected_fee_target).await {
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
    async fn update_left_fee(
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
        self.api_left.update_relay_fee(expected_fee_source).await
    }

    async fn update_right_fee(
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
        self.api_right.update_relay_fee(expected_fee_target).await
    }
}

struct ConversionBalance<'a, SC: Chain, TC: Chain> {
    _marker0: PhantomData<SC>,
    _marker1: PhantomData<TC>,
    subscan_left: &'a Subscan,
    subscan_right: &'a Subscan,
}

impl<'a, SC: Chain, TC: Chain> ConversionBalance<'a, SC, TC> {
    pub fn new(subscan_left: &'a Subscan, subscan_right: &'a Subscan) -> Self {
        Self {
            _marker0: Default::default(),
            _marker1: Default::default(),
            subscan_left,
            subscan_right,
        }
    }

    /// conversion source chain balance to target chain balance
    pub async fn conversion_left_to_right(
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
    pub async fn conversion_right_to_left(
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
        match self.subscan_left.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan = self
                    .subscan_left
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
        match self.subscan_right.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan = self
                    .subscan_right
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
