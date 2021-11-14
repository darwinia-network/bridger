use bridge_traits::bridge::config::Config;
use lifeline::{Lifeline, Service, Task};
use relay_substrate_client::TransactionSignScheme;

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_s2s::api::PangolinApi;
use component_pangolin_s2s::PangolinChain;
use component_pangoro_s2s::api::PangoroApi;
use component_pangoro_s2s::PangoroChain;
use component_subscan::SubscanConfig;

use crate::bus::PangolinPangoroBus;
use crate::config::{ChainInfoConfig, RelayConfig, TaskConfig, UpdateFeeStrategyType};
use crate::fee::strategy::{CrazyStrategy, NothingStrategy, ReasonableStrategy};
use crate::fee::UpdateFeeStrategy;
use crate::task::PangolinPangoroTask;

#[derive(Debug)]
pub struct UpdateFeeService {
    _greet: Lifeline,
}

impl BridgeService for UpdateFeeService {}

impl Service for UpdateFeeService {
    type Bus = PangolinPangoroBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-update-fee", PangolinPangoroTask::NAME),
            async move {
                let config_task: TaskConfig = Config::restore_unwrap(PangolinPangoroTask::NAME)?;
                std::thread::spawn(move || {
                    futures::executor::block_on(cron_update_fee(config_task.clone()))
                })
                .join()
                .map_err(|_| anyhow::Error::msg("Failed to join thread handle"))??;

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn cron_update_fee(config_task: TaskConfig) -> anyhow::Result<()> {
    loop {
        if let Err(e) = run_update_fee(config_task.clone()).await {
            log::error!(
                target: PangolinPangoroTask::NAME,
                "Failed to update fee: {:?}",
                e
            );
        }
        // tokio::time::sleep(std::time::Duration::from_secs(
        //     config_task.interval_update_fee,
        // ))
        // .await;

        // futures_timer::Delay::new(std::time::Duration::from_secs(3)).await?;
    }
}

async fn run_update_fee(config_task: TaskConfig) -> anyhow::Result<()> {
    let subscan_config_pangolin: Option<SubscanConfig> =
        Config::restore_with_namespace(PangolinPangoroTask::NAME, "pangolin")?;
    let subscan_config_pangoro: Option<SubscanConfig> =
        Config::restore_with_namespace(PangolinPangoroTask::NAME, "pangoro")?;
    let exists_subscan_config =
        subscan_config_pangolin.is_some() && subscan_config_pangoro.is_some();
    match config_task.update_fee_strategy {
        UpdateFeeStrategyType::Nothing => NothingStrategy.handle().await,
        UpdateFeeStrategyType::Crazy => {
            if !exists_subscan_config {
                return Ok(());
            }
            let strategy = CrazyStrategy::new().await?;
            strategy.handle().await
        }
        UpdateFeeStrategyType::Reasonable => {
            if !exists_subscan_config {
                return Ok(());
            }
            let strategy = ReasonableStrategy::new().await?;
            strategy.handle().await
        }
    }
}
