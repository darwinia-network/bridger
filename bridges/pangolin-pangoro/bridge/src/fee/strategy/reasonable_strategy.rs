use relay_substrate_client::ChainBase;
use relay_utils::MaybeConnectionError;
use std::time::{SystemTime, UNIX_EPOCH};

use sp_runtime::{FixedPointNumber, FixedU128};

use client_pangolin::PangolinChain;
use client_pangoro::PangoroChain;
use component_subscan::{Subscan, SubscanComponent};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;

use crate::bridge::{PangolinPangoroConfig, PangolinPangoroTask};
use crate::fee::strategy::common::StrategyHelper;
use crate::fee::UpdateFeeStrategy;

const MIN_RELAY_FEE_PANGOLIN: u128 = 15 * drml_common_primitives::COIN;
const MIN_RELAY_FEE_PANGORO: u128 = 15 * drml_common_primitives::COIN;

#[derive(Clone)]
pub struct ReasonableStrategy {
    helper: StrategyHelper,
    subscan_pangolin: Subscan,
    subscan_pangoro: Subscan,
}

impl ReasonableStrategy {
    pub async fn new() -> color_eyre::Result<Self> {
        let helper = StrategyHelper::new().await?;
        let bridge_config: PangolinPangoroConfig = Config::restore(Names::BridgePangolinPangoro)?;

        let config_subscan_pangolin = bridge_config
            .pangolin_subscan
            .ok_or_else(|| BridgerError::Custom("Not have pangolin subscan config".to_string()))?;
        let config_subscan_pangoro = bridge_config
            .pangoro_subscan
            .ok_or_else(|| BridgerError::Custom("Not have pangoro subscan config".to_string()))?;
        let subscan_pangolin = SubscanComponent::component(config_subscan_pangolin)?;
        let subscan_pangoro = SubscanComponent::component(config_subscan_pangoro)?;
        Ok(Self {
            helper,
            subscan_pangolin,
            subscan_pangoro,
        })
    }
}

impl ReasonableStrategy {
    async fn _pangolin_open_price(&self) -> color_eyre::Result<f64> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        match self.subscan_pangolin.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan_dock = self
                    .subscan_pangolin
                    .clone()
                    .endpoint("https://dock.api.subscan.io");
                let open_price = subscan_dock.price(now).await?;
                let data = open_price.data()?;
                match data {
                    Some(v) => Ok(v.price),
                    None => {
                        Err(BridgerError::Custom("Can not query pangolin price".to_string()).into())
                    }
                }
            }
        }
    }

    async fn _pangoro_open_price(&self) -> color_eyre::Result<f64> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        match self.subscan_pangoro.price(now).await?.data() {
            Ok(Some(v)) => Ok(v.price),
            _ => {
                let subscan_darwinia = self
                    .subscan_pangoro
                    .clone()
                    .endpoint("https://darwinia.api.subscan.io");
                let open_price = subscan_darwinia.price(now).await?;
                let data = open_price.data()?;
                match data {
                    Some(v) => Ok(v.price),
                    None => {
                        Err(BridgerError::Custom("Can not query pangoro price".to_string()).into())
                    }
                }
            }
        }
    }

    async fn conversion_pangolin_to_pangoro(
        &self,
        pangolin_currency: drml_common_primitives::Balance,
    ) -> color_eyre::Result<drml_common_primitives::Balance> {
        let price_pangolin = self._pangolin_open_price().await?;
        let price_pangoro = self._pangoro_open_price().await?;
        let rate = price_pangolin / price_pangoro;
        let rate = FixedU128::from_float(rate);
        let ret = rate.saturating_mul_int(pangolin_currency);
        Ok(ret)
    }
    async fn conversion_pangoro_to_pangolin(
        &self,
        pangoro_currency: drml_common_primitives::Balance,
    ) -> color_eyre::Result<drml_common_primitives::Balance> {
        let price_pangolin = self._pangolin_open_price().await?;
        let price_pangoro = self._pangoro_open_price().await?;
        let rate = price_pangoro / price_pangolin;
        let rate = FixedU128::from_float(rate);
        let ret = rate.saturating_mul_int(pangoro_currency);
        Ok(ret)
    }
}

impl ReasonableStrategy {
    async fn update_pangolin_fee(
        &self,
        expect_fee_pangolin: <PangolinChain as ChainBase>::Balance,
    ) -> color_eyre::Result<()> {
        tracing::info!(
            target: "pangolin-pangoro",
            "[reasonable] Update pangolin fee: {}",
            expect_fee_pangolin
        );
        let pangolin_signer = self.helper.pangolin_signer().clone();
        let pangolin_api = self.helper.pangolin_api();
        pangolin_api
            .update_relay_fee(pangolin_signer.clone(), expect_fee_pangolin)
            .await
    }

    async fn update_pangoro_fee(
        &self,
        expect_fee_pangoro: <PangoroChain as ChainBase>::Balance,
    ) -> color_eyre::Result<()> {
        tracing::info!(
            target: "pangolin-pangoro",
            "[reasonable] Update pangoro fee: {}",
            expect_fee_pangoro
        );
        let pangoro_signer = self.helper.pangoro_signer().clone();
        let pangoro_api = self.helper.pangoro_api();
        pangoro_api
            .update_relay_fee(pangoro_signer.clone(), expect_fee_pangoro)
            .await
    }
}

#[async_trait::async_trait]
impl UpdateFeeStrategy for ReasonableStrategy {
    async fn handle(&mut self) -> color_eyre::Result<()> {
        let top100_pangolin = self.subscan_pangolin.extrinsics(1, 100).await?;
        let top100_pangoro = self.subscan_pangoro.extrinsics(1, 100).await?;
        let top100_pangolin = top100_pangolin.data()?.ok_or_else(|| {
            BridgerError::Custom("Can not query pangolin extrinsics data".to_string())
        })?;
        let top100_pangoro = top100_pangoro.data()?.ok_or_else(|| {
            BridgerError::Custom("Can not query pangoro extrinsics data".to_string())
        })?;

        let max_fee_pangolin = top100_pangolin
            .extrinsics
            .unwrap_or_default()
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0);
        let max_fee_pangoro = top100_pangoro
            .extrinsics
            .unwrap_or_default()
            .iter()
            .map(|item| item.fee)
            .max()
            .unwrap_or(0);

        let top100_max_cost_pangolin =
            max_fee_pangolin + self.conversion_pangoro_to_pangolin(max_fee_pangoro).await?;
        let top100_max_cost_pangoro = max_fee_pangoro
            + self
                .conversion_pangolin_to_pangoro(max_fee_pangolin)
                .await?;

        // Nice (
        let expect_fee_pangolin = MIN_RELAY_FEE_PANGOLIN + (top100_max_cost_pangolin * 15);
        let expect_fee_pangoro = MIN_RELAY_FEE_PANGORO + (top100_max_cost_pangoro * 15);

        let mut times = 0;
        loop {
            times += 1;
            if times > 3 {
                tracing::error!(
                    target: "pangolin-pangoro",
                    "[pangoro] Try reconnect many times({}), skip update fee (update fee strategy reasonable)",
                    times
                );
                break;
            }
            match self.update_pangoro_fee(expect_fee_pangoro).await {
                Ok(_) => break,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            tracing::debug!(
                                "[pangoro] Try reconnect to chain (update fee strategy reasonable)"
                            );
                            if let Err(re) = self.helper.reconnect_pangoro().await {
                                tracing::error!(
                                    "[pangoro] Failed to reconnect substrate client: {:?} (update fee strategy)",
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
                tracing::error!(
                    target: "pangolin-pangoro",
                    "[pangolin] Try reconnect many times({}), skip update fee (update fee strategy reasonable)",
                    times
                );
                break;
            }

            match self.update_pangolin_fee(expect_fee_pangolin).await {
                Ok(_) => break,
                Err(e) => {
                    if let Some(client_error) = e.downcast_ref::<relay_substrate_client::Error>() {
                        if client_error.is_connection_error() {
                            tracing::debug!(
                                "[pangolin] Try reconnect to chain (update fee strategy reasonable)"
                            );
                            if let Err(re) = self.helper.reconnect_pangolin().await {
                                tracing::error!(
                                    "[pangolin] Failed to reconnect substrate client: {:?} (update fee strategy)",
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
