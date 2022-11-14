use lifeline::{Lifeline, Service, Task};
use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;

use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};

#[derive(Debug)]
pub struct SubscribeService {
    _greet_darwinia: Lifeline,
    _greet_polkadot: Lifeline,
}

impl BridgeService for SubscribeService {}

impl Service for SubscribeService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_darwinia = Self::try_task(
            &format!("{}-subscribe-darwinia", BridgeTask::name()),
            async move {
                while let Err(e) = start_darwinia().await {
                    tracing::error!(target: "darwinia-darwiniaparachain", "[subscribe] [darwinia] Failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "darwinia-darwiniaparachain", "[subscribe] [darwinia] Try to restart subscription service.");
                }
                Ok(())
            },
        );
        let _greet_polkadot = Self::try_task(
            &format!("{}-subscribe-polkadot", BridgeTask::name()),
            async move {
                while let Err(e) = start_polkadot().await {
                    tracing::error!(target: "darwinia-darwiniaparachain", "[subscribe] [polkadot] Failed to start subscribe {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(target: "darwinia-darwiniaparachain", "[subscribe] [polkadot] Try to restart subscription service.");
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_darwinia,
            _greet_polkadot,
        })
    }
}

async fn start_darwinia() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;

    let client_darwinia = bridge_config.darwinia.to_darwinia_client().await?;

    let input = JustificationInput {
        client: client_darwinia,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}

async fn start_polkadot() -> color_eyre::Result<()> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;

    let client_polkadot = bridge_config.polkadot.to_polkadot_client().await?;

    let input = JustificationInput {
        client: client_polkadot,
    };
    let subscribe = SubscribeJustification::new(input);
    subscribe.start().await?;
    Ok(())
}
