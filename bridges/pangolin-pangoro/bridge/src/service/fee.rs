use bp_messages::LaneId;
use feemarket_s2s::config::FeemarketConfig;
use lifeline::{Lifeline, Service, Task};
use relay_pangolin_client::PangolinChain;
use relay_pangoro_client::PangoroChain;

use feemarket_s2s::fee::{CrazyStrategy, NothingStrategy, ReasonableStrategy, UpdateFeeStrategy};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{ChainInfoConfig, PangolinPangoroTask, RelayConfig};
use crate::bridge::{PangolinPangoroBus, PangolinPangoroConfig};
use crate::bridge::{TaskConfig, UpdateFeeStrategyType};
use crate::feemarket::{PangolinFeemarketApi, PangoroFeemarketApi};

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
    std::thread::spawn(move || futures::executor::block_on(cron_update_fee(config_task)))
        .join()
        .map_err(|_| BridgerError::Custom("Failed to join thread handle".to_string()))??;
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

    let config_relay: RelayConfig = bridge_config.relay;

    let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
    let config_pangoro: ChainInfoConfig = bridge_config.pangoro;
    let (pangolin_chain, pangoro_chain) = (
        config_pangolin.to_chain_info_with_expect_signer(config_relay.signer_pangolin.clone())?,
        config_pangoro.to_chain_info_with_expect_signer(config_relay.signer_pangoro.clone())?,
    );

    let pangolin_client = pangolin_chain
        .to_substrate_relay_chain::<PangolinChain>()
        .await?;
    let pangoro_client = pangoro_chain
        .to_substrate_relay_chain::<PangoroChain>()
        .await?;

    let pangolin_signer = pangolin_chain.to_keypair::<PangolinChain>()?;
    let pangoro_signer = pangoro_chain.to_keypair::<PangoroChain>()?;

    // todo: If there are multiple lanes, support is also required here
    let lanes = config_relay.lanes;
    let lane: LaneId = lanes
        .get(0)
        .cloned()
        .ok_or_else(|| BridgerError::Custom("Missing lane id".to_string()))?
        .into();

    let pangolin_feemarket_api =
        PangolinFeemarketApi::new(pangolin_client.clone(), lane, pangolin_signer.clone());
    let pangoro_feemarket_api =
        PangoroFeemarketApi::new(pangoro_client.clone(), lane, pangoro_signer.clone());

    match config_task.update_fee_strategy {
        UpdateFeeStrategyType::Nothing => Ok(NothingStrategy.handle().await?),
        UpdateFeeStrategyType::Crazy => {
            let strategy = CrazyStrategy::new(pangolin_feemarket_api, pangoro_feemarket_api);
            Ok(strategy.handle().await?)
        }
        UpdateFeeStrategyType::Reasonable => {
            let feemarket_config: FeemarketConfig = bridge_config.feemarket;
            let strategy = ReasonableStrategy::new(
                feemarket_config,
                pangolin_feemarket_api,
                pangoro_feemarket_api,
                15 * 1000000000,
                15 * 1000000000,
            )?;
            Ok(strategy.handle().await?)
        }
    }
}
