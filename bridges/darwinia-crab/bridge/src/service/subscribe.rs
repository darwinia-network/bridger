use lifeline::{Lifeline, Service, Task};

use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};

#[derive(Debug)]
pub struct SubscribeService {
    _greet: Lifeline,
}

impl BridgeService for SubscribeService {}

impl Service for SubscribeService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(&format!("{}-relay", BridgeTask::name()), async move {
            while let Err(e) = start().await {
                tracing::error!(target: "darwinia-crab", "[subscribe] Failed to start subscribe {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(target: "darwinia-crab", "[subscribe] Try to restart subscription service.");
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaCrab)?;

    let client_crab = bridge_config.crab.to_crab_client().await?;
    let client_darwinia = bridge_config.darwinia.to_darwinia_client().await?;

    let input = JustificationInput {
        client_source: client_crab,
        client_target: client_darwinia,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}
