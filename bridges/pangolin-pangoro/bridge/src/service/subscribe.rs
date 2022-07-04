use lifeline::{Lifeline, Service, Task};

use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};

#[derive(Debug)]
pub struct SubscribeService {
    _greet_pangolin: Lifeline,
    _greet_pangoro: Lifeline,
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
                    tracing::error!(target: "pangolin-pangoro", "[subscribe] [pangolin] failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "pangolin-pangoro", "[subscribe] [pangolin] try to restart subscription service.");
                }
                Ok(())
            },
        );
        let _greet_pangoro = Self::try_task(
            &format!("{}-subscribe-pangoro", BridgeTask::name()),
            async move {
                while let Err(e) = start_pangoro().await {
                    tracing::error!(target: "pangolin-pangoro", "[subscribe] [pangoro] failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "pangolin-pangoro", "[subscribe] [pangoro] try to restart subscription service.");
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_pangolin,
            _greet_pangoro,
        })
    }
}

async fn start_pangolin() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;

    let input = JustificationInput {
        client: client_pangolin,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}

async fn start_pangoro() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

    let client_pangoro = bridge_config.pangoro.to_pangoro_client().await?;

    let input = JustificationInput {
        client: client_pangoro,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}
