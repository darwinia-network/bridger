use lifeline::{Lifeline, Service, Task};

use relay_s2s::header::ParaHeaderRunner;
use relay_s2s::types::ParaHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct ParaHeadRelayService {
    _greet: Lifeline,
}

impl BridgeService for ParaHeadRelayService {}

impl Service for ParaHeadRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("para-head-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "crab-crabparachain",
                    "[header-relay] [para-head-to-crab] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "crab-crabparachain",
                    "[header-relay] [para-head-to-crab] Try to restart header relay service.",
                );
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "crab-crabparachain",
        "[header-para-head-to-crab] [para-head-to-crab] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;
    let config_relay = bridge_config.relay;

    let client_crab = bridge_config.crab.to_crab_client().await?;
    let client_kusama = bridge_config.kusama.to_kusama_client().await?;

    let input = ParaHeaderInput {
        client_relaychain: client_kusama,
        client_solochain: client_crab,
        para_id: config_relay.para_id,
    };
    let runner = ParaHeaderRunner::new(input);
    Ok(runner.start().await?)
}
