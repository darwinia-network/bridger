use lifeline::{Lifeline, Service, Task};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_subscan::SubscanConfig;

use crate::bus::DarwiniaCrabBus;
use crate::config::{TaskConfig, UpdateFeeStrategyType};
use crate::fee::strategy::{CrazyStrategy, ReasonableStrategy};
use crate::fee::UpdateFeeStrategy;
use crate::task::DarwiniaCrabTask;

#[derive(Debug)]
pub struct UpdateFeeService {
    _greet: Lifeline,
}

impl BridgeService for UpdateFeeService {}

impl Service for UpdateFeeService {
    type Bus = DarwiniaCrabBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-update-fee", DarwiniaCrabTask::NAME),
            async move {
                let config_task: TaskConfig = Config::restore_unwrap(DarwiniaCrabTask::NAME)?;
                std::thread::spawn(move || {
                    futures::executor::block_on(cron_update_fee(config_task))
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
    log::info!(
        target: DarwiniaCrabTask::NAME,
        "Use update fee strategy: {:?}",
        config_task.update_fee_strategy
    );
    loop {
        if let Err(e) = run_update_fee(config_task.clone()).await {
            log::error!(
                target: DarwiniaCrabTask::NAME,
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

async fn run_update_fee(config_task: TaskConfig) -> anyhow::Result<()> {
    let subscan_config_darwinia: Option<SubscanConfig> =
        Config::restore_with_namespace(DarwiniaCrabTask::NAME, "darwinia")?;
    let subscan_config_crab: Option<SubscanConfig> =
        Config::restore_with_namespace(DarwiniaCrabTask::NAME, "crab")?;
    let exists_subscan_config = subscan_config_darwinia.is_some() && subscan_config_crab.is_some();
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
