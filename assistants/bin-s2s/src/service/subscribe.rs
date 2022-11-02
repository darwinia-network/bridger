use lifeline::{Lifeline, Service, Task};
use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;

use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};
use crate::error::BinS2SResult;

#[derive(Debug)]
pub struct SubscribeService {
    _greet_darwinia: Lifeline,
    _greet_crab: Lifeline,
}

impl BridgeService for SubscribeService {}

impl Service for SubscribeService {
    type Bus = BridgeBus;
    type Lifeline = BinS2SResult<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_darwinia = Self::try_task(
            &format!("{}-subscribe-darwinia", BridgeTask::name()),
            async move {
                while let Err(e) = start_darwinia().await {
                    tracing::error!(target: "bin-s2s", "[subscribe] [darwinia] failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "bin-s2s", "[subscribe] [crab] try to restart subscription service.");
                }
                Ok(())
            },
        );
        let _greet_crab = Self::try_task(
            &format!("{}-subscribe-crab", BridgeTask::name()),
            async move {
                while let Err(e) = start_crab().await {
                    tracing::error!(target: "bin-s2s", "[subscribe] [crab] failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "bin-s2s", "[subscribe] [crab] try to restart subscription service.");
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_darwinia,
            _greet_crab,
        })
    }
}

async fn start_darwinia() -> BinS2SResult<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaCrab)?;

    let client_darwinia = bridge_config.target.to_darwinia_client().await?;

    let input = JustificationInput {
        client: client_darwinia,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}

async fn start_crab() -> BinS2SResult<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaCrab)?;

    let client_crab = bridge_config.source.to_crab_client().await?;

    let input = JustificationInput {
        client: client_crab,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}
