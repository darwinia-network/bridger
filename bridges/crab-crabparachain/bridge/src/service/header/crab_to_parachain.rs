use lifeline::{Lifeline, Service, Task};
use subquery::types::OriginType;

use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct CrabToParachainHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for CrabToParachainHeaderRelayService {}

impl Service for CrabToParachainHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("crab-to-crabparachain-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "crab-crabparachain",
                    "[header-relay] [crab-to-crabparachain] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "crab-crabparachain",
                    "[header-relay] [crab-to-crabparachain] Try to restart header relay service.",
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
        "[header-crab-to-crabparachain] [crab-to-crabparachain] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;
    let relay_config = bridge_config.relay;

    let client_crab = bridge_config.crab.to_crab_client().await?;
    let client_crab_parachain = bridge_config
        .crab_parachain
        .to_crab_parachain_client()
        .await?;

    let config_index = bridge_config.index;
    let subquery_crab = config_index.to_crab_subquery();

    let input = SolochainHeaderInput {
        client_source: client_crab,
        client_target: client_crab_parachain,
        subquery_source: subquery_crab,
        index_origin_type: OriginType::BridgeCrabParachain,
        enable_mandatory: relay_config.enable_mandatory,
    };
    let runner = SolochainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
