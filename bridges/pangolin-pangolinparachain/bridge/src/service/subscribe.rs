use lifeline::{Lifeline, Service, Task};

use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};

#[derive(Debug)]
pub struct SubscribeService {
    _greet_pangolin: Lifeline,
    _greet_rococo: Lifeline,
}

impl BridgeService for SubscribeService {}

impl Service for SubscribeService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_pangolin = Self::try_task(
            &format!("{}-subscribe-pangolin", BridgeTask::name()),
            async move {
                while let Err(e) = start_pangolin().await {
                    tracing::error!(target: "pangolin-pangolinparachain", "[subscribe] [pangolin] failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "pangolin-pangolinparachain", "[subscribe] [pangolin] try to restart subscription service.");
                }
                Ok(())
            },
        );
        let _greet_rococo = Self::try_task(
            &format!("{}-subscribe-rococo", BridgeTask::name()),
            async move {
                while let Err(e) = start_rococo().await {
                    tracing::error!(target: "pangolin-pangolinparachain", "[subscribe] [rococo] failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "pangolin-pangolinparachain", "[subscribe] [rococo] try to restart subscription service.");
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_pangolin,
            _greet_rococo,
        })
    }
}

async fn start_pangolin() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachain)?;

    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;

    let input = JustificationInput {
        client: client_pangolin,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}

async fn start_rococo() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachain)?;

    let client_rococo = bridge_config.rococo.to_rococo_client().await?;

    let input = JustificationInput {
        client: client_rococo,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}
