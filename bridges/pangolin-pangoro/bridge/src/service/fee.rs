use lifeline::{Lifeline, Service, Task};

use component_subscan::SubscanConfig;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::PangolinPangoroTask;
use crate::bridge::{PangolinPangoroBus, PangolinPangoroConfig};
use crate::bridge::{TaskConfig, UpdateFeeStrategyType};
use crate::fee::strategy::{CrazyStrategy, ReasonableStrategy};
use crate::fee::UpdateFeeStrategy;

#[derive(Debug)]
pub struct UpdateFeeService {
    _greet: Lifeline,
}

impl BridgeService for UpdateFeeService {}

impl Service for UpdateFeeService {
    type Bus = PangolinPangoroBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-update-fee", PangolinPangoroTask::name()),
            async move {
                if let Err(e) = start() {
                    tracing::error!(target: "pangolin-pangoro", "{:?}", e);
                    return Err(
                        BridgerError::Custom("Failed to start fee service".to_string()).into(),
                    );
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

fn start() -> color_eyre::Result<()> {
    let bridge_config: PangolinPangoroConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let config_task: TaskConfig = bridge_config.task;
    futures::executor::block_on(cron_update_fee(config_task))?;
    Ok(())
}

async fn cron_update_fee(config_task: TaskConfig) -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangoro",
        "Use update fee strategy: {:?}",
        config_task.update_fee_strategy
    );
    loop {
        if let Err(e) = run_update_fee(config_task.clone()).await {
            tracing::error!(
                target: "pangolin-pangoro",
                "Failed to update fee: {:?}",
                e
            );
        }

        futures_timer::Delay::new(std::time::Duration::from_secs(
            config_task.interval_update_fee,
        ))
        .await;
    }
}

async fn run_update_fee(config_task: TaskConfig) -> color_eyre::Result<()> {
    let bridge_config: PangolinPangoroConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let subscan_config_pangolin: Option<SubscanConfig> = bridge_config.pangolin_subscan;
    let subscan_config_pangoro: Option<SubscanConfig> = bridge_config.pangoro_subscan;
    let exists_subscan_config =
        subscan_config_pangolin.is_some() && subscan_config_pangoro.is_some();
    match config_task.update_fee_strategy {
        UpdateFeeStrategyType::Nothing => Ok(()),
        UpdateFeeStrategyType::Crazy => {
            if !exists_subscan_config {
                return Ok(());
            }
            let mut strategy = CrazyStrategy::new().await?;
            strategy.handle().await
        }
        UpdateFeeStrategyType::Reasonable => {
            if !exists_subscan_config {
                return Ok(());
            }
            let mut strategy = ReasonableStrategy::new().await?;
            strategy.handle().await
        }
    }
}
