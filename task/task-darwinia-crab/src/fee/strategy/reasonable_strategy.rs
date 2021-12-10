use relay_substrate_client::ChainBase;
use relay_utils::MaybeConnectionError;
use std::time::{SystemTime, UNIX_EPOCH};

use sp_runtime::{FixedPointNumber, FixedU128};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::StandardError;
use component_crab_s2s::CrabChain;
use component_darwinia_s2s::DarwiniaChain;
use component_subscan::{Subscan, SubscanComponent};

use crate::fee::strategy::common::StrategyHelper;
use crate::fee::UpdateFeeStrategy;
use crate::task::DarwiniaCrabTask;

const MIN_RELAY_FEE_DARWINIA: u128 = 15 * darwinia_common_primitives::COIN;
const MIN_RELAY_FEE_CRAB: u128 = 15 * darwinia_common_primitives::COIN;

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
    async fn _darwinia_open_price(&self) -> anyhow::Result<f64> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        match self.subscan_darwinia.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan_darwinia = self
                    .subscan_darwinia
                    .clone()
                    .endpoint("https://darwinia.api.subscan.io");
                let open_price = subscan_darwinia.price(now).await?;
                let data = open_price.data()?;
                match data {
                    Some(v) => Ok(v.price),
                    None => anyhow::bail!("Can not query crab price"),
                }
            }
        }
    }

    async fn _crab_open_price(&self) -> anyhow::Result<f64> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        match self.subscan_crab.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan_dock = self
                    .subscan_crab
                    .clone()
                    .endpoint("https://dock.api.subscan.io");
                let open_price = subscan_dock.price(now).await?;
                let data = open_price.data()?;
                match data {
                    Some(v) => Ok(v.price),
                    None => anyhow::bail!("Can not query darwinia price"),
                }
            }
        }
    }

    async fn conversion_darwinia_to_crab(
        &self,
        darwinia_currency: darwinia_common_primitives::Balance,
    ) -> anyhow::Result<darwinia_common_primitives::Balance> {
        let price_darwinia = self._darwinia_open_price().await?;
        let price_crab = self._crab_open_price().await?;
        let rate = price_darwinia / price_crab;
        let rate = FixedU128::from_float(rate);
        let ret = rate.saturating_mul_int(darwinia_currency);
        Ok(ret)
    }
    async fn conversion_crab_to_darwinia(
        &self,
        crab_currency: darwinia_common_primitives::Balance,
    ) -> anyhow::Result<darwinia_common_primitives::Balance> {
        let price_darwinia = self._darwinia_open_price().await?;
        let price_crab = self._crab_open_price().await?;
        let rate = price_crab / price_darwinia;
        let rate = FixedU128::from_float(rate);
        let ret = rate.saturating_mul_int(crab_currency);
        Ok(ret)
    }
}

impl ReasonableStrategy {
    async fn update_darwinia_fee(
        &self,
        expect_fee_darwinia: <DarwiniaChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        log::info!(
            target: DarwiniaCrabTask::NAME,
            "[reasonable] Update darwinia fee: {}",
            expect_fee_darwinia
        );
        let darwinia_signer = self.helper.darwinia_signer().clone();
        let darwinia_api = self.helper.darwinia_api();
        darwinia_api
            .update_relay_fee(darwinia_signer.clone(), expect_fee_darwinia)
            .await
    }

    async fn update_crab_fee(
        &self,
        expect_fee_crab: <CrabChain as ChainBase>::Balance,
    ) -> anyhow::Result<()> {
        log::info!(
            target: DarwiniaCrabTask::NAME,
            "[reasonable] Update crab fee: {}",
            expect_fee_crab
        );
        let crab_signer = self.helper.crab_signer().clone();
        let crab_api = self.helper.crab_api();
        crab_api
            .update_relay_fee(crab_signer.clone(), expect_fee_crab)
            .await
    }
}

#[async_trait::async_trait]
impl UpdateFeeStrategy for ReasonableStrategy {
    async fn handle(&mut self) -> anyhow::Result<()> {
        let top100_darwinia = self.subscan_darwinia.extrinsics(1, 100).await?;
        let top100_crab = self.subscan_crab.extrinsics(1, 100).await?;
        let top100_darwinia = top100_darwinia.data()?.ok_or_else(|| {
            StandardError::Api("Can not query darwinia extrinsics data".to_string())
        })?;
        let top100_crab = top100_crab
            .data()?
            .ok_or_else(|| StandardError::Api("Can not query crab extrinsics data".to_string()))?;

        let max_fee_darwinia = top100_darwinia
            .extrinsics
            .unwrap_or_default()
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0);
        let max_fee_crab = top100_crab
            .extrinsics
            .unwrap_or_default()
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0);

        let top100_max_cost_darwinia =
            max_fee_darwinia + self.conversion_crab_to_darwinia(max_fee_crab).await?;
        let top100_max_cost_crab =
            max_fee_crab + self.conversion_darwinia_to_crab(max_fee_darwinia).await?;

        // Nice (
        let expect_fee_darwinia = MIN_RELAY_FEE_DARWINIA + (top100_max_cost_darwinia * 15);
        let expect_fee_crab = MIN_RELAY_FEE_CRAB + (top100_max_cost_crab * 15);

        let mut times = 0;
        loop {
            times += 1;
            if times > 3 {
                log::error!(
                    target: DarwiniaCrabTask::NAME,
                    "[darwinia] Try reconnect many times({}), skip update fee (update fee strategy reasonable)",
                    times
                );
                break;
            }
            match self.update_darwinia_fee(expect_fee_darwinia).await {
                Ok(_) => break,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            log::debug!(
                                "[darwinia] Try reconnect to chain (update fee strategy reasonable)"
                            );
                            if let Err(re) = self.helper.reconnect_darwinia().await {
                                log::error!(
                                    "[darwinia] Failed to reconnect substrate client: {:?} (update fee strategy)",
                                    re
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        }

        times = 0;
        loop {
            times += 1;
            if times > 3 {
                log::error!(
                    target: DarwiniaCrabTask::NAME,
                    "[crab] Try reconnect many times({}), skip update fee (update fee strategy reasonable)",
                    times
                );
                break;
            }

            match self.update_crab_fee(expect_fee_crab).await {
                Ok(_) => break,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            log::debug!(
                                "[crab] Try reconnect to chain (update fee strategy reasonable)"
                            );
                            if let Err(re) = self.helper.reconnect_crab().await {
                                log::error!(
                                    "[crab] Failed to reconnect substrate client: {:?} (update fee strategy)",
                                    re
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
