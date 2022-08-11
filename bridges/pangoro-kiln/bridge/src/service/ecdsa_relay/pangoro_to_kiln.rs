use crate::bridge::{BridgeConfig, PangoroKilnBus};
use crate::service::ecdsa_relay::collected_enough_authorities_change_signatures::CollectedEnoughAuthoritiesChangeSignaturesRunner;
use crate::service::ecdsa_relay::collected_enough_new_message_root_signatures::CollectedEnoughNewMessageRootSignaturesRunner;
use crate::service::ecdsa_relay::collecting_authorities_change_signatures::CollectingAuthoritiesChangeSignaturesRunner;
use crate::service::ecdsa_relay::collecting_new_message_root_signatures::CollectingNewMessageRootSignaturesRunner;
use crate::service::ecdsa_relay::types::EcdsaSource;
use lifeline::{Lifeline, Service, Task};
use subquery::Subquery;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

#[derive(Debug)]
pub struct ECDSARelayService {
    _greet: Lifeline,
}

impl BridgeService for ECDSARelayService {}

impl Service for ECDSARelayService {
    type Bus = PangoroKilnBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("ecdsa-relay-pangoro-to-kiln", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start pangoro-to-kiln ecdsa relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;
    let subquery = config.index.to_pangoro_subquery();
    let client = config.pangoro_substrate.to_substrate_client().await?;
    let source = EcdsaSource { subquery, client };
    loop {
        run(source.clone()).await?;
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

async fn run(source: EcdsaSource) -> color_eyre::Result<()> {
    let runner = CollectingAuthoritiesChangeSignaturesRunner::new(source.clone());
    runner.start().await?;

    let runner = CollectingNewMessageRootSignaturesRunner::new(source.clone());
    runner.start().await?;

    let runner = CollectedEnoughAuthoritiesChangeSignaturesRunner::new(source.clone());
    runner.start().await?;

    let runner = CollectedEnoughNewMessageRootSignaturesRunner::new(source);
    runner.start().await?;
    Ok(())
}
