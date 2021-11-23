use std::time::{SystemTime, UNIX_EPOCH};

use sp_runtime::{FixedPointNumber, FixedU128};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::StandardError;
use component_subscan::{Subscan, SubscanComponent};

use crate::fee::strategy::common::StrategyHelper;
use crate::fee::UpdateFeeStrategy;
use crate::task::DarwiniaCrabTask;

const MIN_RELAY_FEE_PANGOLIN: u128 = 15 * common_primitives::COIN;
const MIN_RELAY_FEE_PANGORO: u128 = 15 * common_primitives::COIN;

#[derive(Clone)]
pub struct ReasonableStrategy {
    helper: StrategyHelper,
    subscan_darwinia: Subscan,
    subscan_crab: Subscan,
}

impl ReasonableStrategy {
    pub async fn new() -> anyhow::Result<Self> {
        let helper = StrategyHelper::new().await?;
        let component_subcan_darwinia =
            SubscanComponent::restore_with_namespace::<DarwiniaCrabTask>("darwinia".to_string())?;
        let component_subscan_crab =
            SubscanComponent::restore_with_namespace::<DarwiniaCrabTask>("crab".to_string())?;
        let subscan_darwinia = component_subcan_darwinia.component().await?;
        let subscan_crab = component_subscan_crab.component().await?;
        Ok(Self {
            helper,
            subscan_darwinia,
            subscan_crab,
        })
    }
}

impl ReasonableStrategy {
    async fn _pangolin_open_price(&self) -> anyhow::Result<f64> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        match self.subscan_darwinia.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan_dock = self
                    .subscan_darwinia
                    .clone()
                    .endpoint("https://dock.api.subscan.io");
                let open_price = subscan_dock.price(now).await?;
                let data = open_price.data()?;
                match data {
                    Some(v) => Ok(v.price),
                    None => anyhow::bail!("Can not query pangolin price"),
                }
            }
        }
    }

    async fn _pangoro_open_price(&self) -> anyhow::Result<f64> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        match self.subscan_crab.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan_darwinia = self
                    .subscan_crab
                    .clone()
                    .endpoint("https://darwinia.api.subscan.io");
                let open_price = subscan_darwinia.price(now).await?;
                let data = open_price.data()?;
                match data {
                    Some(v) => Ok(v.price),
                    None => anyhow::bail!("Can not query pangoro price"),
                }
            }
        }
    }

    async fn conversion_darwinia_to_crab(
        &self,
        pangolin_currency: common_primitives::Balance,
    ) -> anyhow::Result<common_primitives::Balance> {
        let price_pangolin = self._pangolin_open_price().await?;
        let price_pangoro = self._pangoro_open_price().await?;
        let rate = price_pangolin / price_pangoro;
        let rate = FixedU128::from_float(rate);
        let ret = rate.saturating_mul_int(pangolin_currency);
        Ok(ret)
    }
    async fn conversion_crab_to_darwinia(
        &self,
        pangoro_currency: common_primitives::Balance,
    ) -> anyhow::Result<common_primitives::Balance> {
        let price_pangolin = self._pangolin_open_price().await?;
        let price_pangoro = self._pangoro_open_price().await?;
        let rate = price_pangoro / price_pangolin;
        let rate = FixedU128::from_float(rate);
        let ret = rate.saturating_mul_int(pangoro_currency);
        Ok(ret)
    }
}

#[async_trait::async_trait]
impl UpdateFeeStrategy for ReasonableStrategy {
    async fn handle(&self) -> anyhow::Result<()> {
        let top100_darwinia = self.subscan_darwinia.extrinsics(1, 100).await?;
        let top100_crab = self.subscan_crab.extrinsics(1, 100).await?;
        let top100_darwinia = top100_darwinia.data()?.ok_or_else(|| {
            StandardError::Api("Can not query pangolin extrinsics data".to_string())
        })?;
        let top100_crab = top100_crab.data()?.ok_or_else(|| {
            StandardError::Api("Can not query pangoro extrinsics data".to_string())
        })?;

        let max_fee_pangolin = (&top100_darwinia.extrinsics)
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0);
        let max_fee_pangoro = (&top100_crab.extrinsics)
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0);

        let top100_max_cost_darwinia =
            max_fee_pangolin + self.conversion_crab_to_darwinia(max_fee_pangoro).await?;
        let top100_max_cost_crab =
            max_fee_pangoro + self.conversion_darwinia_to_crab(max_fee_pangolin).await?;

        // Nice (
        let expect_fee_darwinia = MIN_RELAY_FEE_PANGOLIN + (top100_max_cost_darwinia * 15);
        let expect_fee_crab = MIN_RELAY_FEE_PANGORO + (top100_max_cost_crab * 15);

        log::info!(
            target: DarwiniaCrabTask::NAME,
            "[reasonable] Update crab fee: {}",
            expect_fee_crab
        );
        let crab_api = self.helper.crab_api();
        crab_api
            .update_relay_fee(self.helper.crab_signer().clone(), expect_fee_crab)
            .await?;

        log::info!(
            target: DarwiniaCrabTask::NAME,
            "[reasonable] Update darwinia fee: {}",
            expect_fee_darwinia
        );
        let darwinia_api = self.helper.darwinia_api();
        darwinia_api
            .update_relay_fee(self.helper.darwinia_signer().clone(), expect_fee_darwinia)
            .await?;
        Ok(())
    }
}
