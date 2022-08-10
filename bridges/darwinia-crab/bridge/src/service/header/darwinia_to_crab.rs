use lifeline::{Lifeline, Service, Task};
use subquery::types::OriginType;

use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct DarwiniaToCrabHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for DarwiniaToCrabHeaderRelayService {}

impl Service for DarwiniaToCrabHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("darwinia-to-crab-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "darwinia-crab",
                    "[header-relay] [darwinia-to-crab] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "darwinia-crab",
                    "[header-relay] [darwinia-to-crab] Try to restart header relay service.",
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
        "[header-crab-to-darwinia] [darwinia-to-crab] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
    let relay_config = bridge_config.relay;

    let client_crab = bridge_config.crab.to_crab_client().await?;
    let client_darwinia = bridge_config.darwinia.to_darwinia_client().await?;

    let config_index = bridge_config.index;
    let subquery_darwinia = config_index.to_darwinia_subquery();
    let lanes = relay_config.raw_lanes();

    let input = SolochainHeaderInput {
        lanes,
        client_source: client_darwinia,
        client_target: client_crab,
        subquery_source: subquery_darwinia,
        index_origin_type: OriginType::BridgeCrab,
        enable_mandatory: relay_config.enable_mandatory,
    };
    let runner = SolochainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
