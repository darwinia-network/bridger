use lifeline::{Lifeline, Service, Task};
use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;

use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};

#[derive(Debug)]
pub struct SubscribeService {
    _greet_crab: Lifeline,
    _greet_kusama: Lifeline,
}

impl BridgeService for SubscribeService {}

impl Service for SubscribeService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_crab = Self::try_task(
            &format!("{}-subscribe-crab", BridgeTask::name()),
            async move {
                while let Err(e) = start_crab().await {
                    tracing::error!(target: "crab-crabparachain", "[subscribe] [crab] Failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "crab-crabparachain", "[subscribe] [crab] Try to restart subscription service.");
                }
                Ok(())
            },
        );
        let _greet_kusama = Self::try_task(
            &format!("{}-subscribe-kusama", BridgeTask::name()),
            async move {
                while let Err(e) = start_kusama().await {
                    tracing::error!(target: "crab-crabparachain", "[subscribe] [kusama] Failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "crab-crabparachain", "[subscribe] [kusama] Try to restart subscription service.");
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_crab,
            _greet_kusama,
        })
    }
}

async fn start_crab() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;

    let client_crab = bridge_config.crab.to_crab_client().await?;

    let input = JustificationInput {
        client: client_crab,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}

async fn start_kusama() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;

    let client_kusama = bridge_config.kusama.to_kusama_client().await?;

    let input = JustificationInput {
        client: client_kusama,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}
