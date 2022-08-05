use lifeline::{Lifeline, Service, Task};
use subquery_s2s::types::OriginType;

use relay_s2s::header::RelaychainHeaderRunner;
use relay_s2s::types::RelaychainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct KusamaToCrabHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for KusamaToCrabHeaderRelayService {}

impl Service for KusamaToCrabHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("kusama-to-crab-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "crab-crabparachain",
                    "[header-relay] [kusama-to-crab] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "crab-crabparachain",
                    "[header-relay] [kusama-to-crab] Try to restart header relay service.",
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
        "[header-kusama-to-crab] [kusama-to-crab] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;

    let client_crab = bridge_config.crab.to_crab_client().await?;
    let client_kusama = bridge_config.kusama.to_kusama_client().await?;

    let config_index = bridge_config.index;
    let subquery_kusama = config_index.to_kusama_subquery();
    let subquery_crab_parachain = config_index.to_crab_parachain_subquery();

    let input = RelaychainHeaderInput {
        client_relaychain: client_kusama,
        client_solochain: client_crab,
        subquery_relaychain: subquery_kusama,
        subquery_parachain: subquery_crab_parachain,
        index_origin_type: OriginType::BridgeCrab,
        enable_mandatory: bridge_config.relay.enable_mandatory,
    };
    let runner = RelaychainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
