use lifeline::{Lifeline, Service, Task};
use subquery_s2s::types::OriginType;

use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct CrabToDarwiniaHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for CrabToDarwiniaHeaderRelayService {}

impl Service for CrabToDarwiniaHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("crab-to-darwinia-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "darwinia-crab",
                    "[header-relay] [crab-to-darwinia] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "darwinia-crab",
                    "[header-relay] [crab-to-darwinia] Try to restart header relay service.",
                );
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "darwinia-crab",
        "[header-crab-to-darwinia] [crab-to-darwinia] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
    let relay_config = bridge_config.relay;

    let client_crab = bridge_config.crab.to_crab_client().await?;
    let client_darwinia = bridge_config.darwinia.to_darwinia_client().await?;

    let config_index = bridge_config.index;
    let subquery_crab = config_index.to_crab_subquery();
    let lanes = relay_config.raw_lanes();

    let input = SolochainHeaderInput {
        lanes,
        client_source: client_crab,
        client_target: client_darwinia,
        subquery_source: subquery_crab,
        index_origin_type: OriginType::BridgeDarwinia,
    };
    let runner = SolochainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
