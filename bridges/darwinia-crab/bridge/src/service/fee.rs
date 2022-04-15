use bp_messages::LaneId;
use feemarket_s2s::config::FeemarketConfig;
use lifeline::{Lifeline, Service, Task};
use relay_crab_client::CrabChain;
use relay_darwinia_client::DarwiniaChain;

use feemarket_s2s::fee::{CrazyStrategy, NothingStrategy, ReasonableStrategy, UpdateFeeStrategy};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{ChainInfoConfig, CrabDarwiniaTask, RelayConfig};
use crate::bridge::{DarwiniaCrabBus, DarwiniaCrabConfig};
use crate::bridge::{TaskConfig, UpdateFeeStrategyType};
use crate::feemarket::{CrabFeemarketApi, DarwiniaFeemarketApi};

#[derive(Debug)]
pub struct UpdateFeeService {
    _greet: Lifeline,
}

impl BridgeService for UpdateFeeService {}

impl Service for UpdateFeeService {
    type Bus = DarwiniaCrabBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-update-fee", CrabDarwiniaTask::name()),
            async move {
                if let Err(e) = start() {
                    tracing::error!(target: "darwinia-crab", "{:?}", e);
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
    let bridge_config: DarwiniaCrabConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
    let config_task: TaskConfig = bridge_config.task;
    std::thread::spawn(move || futures::executor::block_on(cron_update_fee(config_task)))
        .join()
        .map_err(|_| BridgerError::Custom("Failed to join thread handle".to_string()))??;
    Ok(())
}

async fn cron_update_fee(config_task: TaskConfig) -> color_eyre::Result<()> {
    tracing::info!(
        target: "darwinia-crab",
        "Use update fee strategy: {:?}",
        config_task.update_fee_strategy
    );
    loop {
        if let Err(e) = run_update_fee(config_task.clone()).await {
            tracing::error!(
                target: "darwinia-crab",
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
    let bridge_config: DarwiniaCrabConfig = Config::restore(Names::BridgeDarwiniaCrab)?;

    let config_relay: RelayConfig = bridge_config.relay;

    let config_crab: ChainInfoConfig = bridge_config.crab;
    let config_darwinia: ChainInfoConfig = bridge_config.darwinia;
    let (crab_chain, darwinia_chain) = (
        config_crab.to_chain_info_with_expect_signer(config_relay.signer_crab.clone())?,
        config_darwinia.to_chain_info_with_expect_signer(config_relay.signer_darwinia.clone())?,
    );

    let crab_client = crab_chain.to_substrate_relay_chain::<CrabChain>().await?;
    let darwinia_client = darwinia_chain
        .to_substrate_relay_chain::<DarwiniaChain>()
        .await?;

    let crab_signer = crab_chain.to_keypair::<CrabChain>()?;
    let darwinia_signer = darwinia_chain.to_keypair::<DarwiniaChain>()?;

    // todo: If there are multiple lanes, support is also required here
    let lanes = config_relay.lanes;
    let lane: LaneId = lanes
        .get(0)
        .cloned()
        .ok_or_else(|| BridgerError::Custom("Missing lane id".to_string()))?
        .into();

    let crab_feemarket_api = CrabFeemarketApi::new(crab_client.clone(), lane, crab_signer.clone());
    let darwinia_feemarket_api =
        DarwiniaFeemarketApi::new(darwinia_client.clone(), lane, darwinia_signer.clone());

    match config_task.update_fee_strategy {
        UpdateFeeStrategyType::Nothing => Ok(NothingStrategy.handle().await?),
        UpdateFeeStrategyType::Crazy => {
            let strategy = CrazyStrategy::new(crab_feemarket_api, darwinia_feemarket_api);
            Ok(strategy.handle().await?)
        }
        UpdateFeeStrategyType::Reasonable => {
            let feemarket_config: FeemarketConfig = bridge_config.feemarket;
            let strategy = ReasonableStrategy::new(
                feemarket_config,
                crab_feemarket_api,
                darwinia_feemarket_api,
                15 * 1000000000,
                15 * 1000000000,
            )?;
            Ok(strategy.handle().await?)
        }
    }
}
