use crate::bridge::{BridgeConfig, PangoroKilnBus};
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
        let _greet = Self::try_task("execution-layer-kiln-to-pangoro", async move {
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
    let index_config = config.index;
    let subquery_pangoro = index_config.to_pangoro_subquery();
    loop {
        run().await?;
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

async fn run() -> color_eyre::Result<()> {
    Ok(())
}
