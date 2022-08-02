use lifeline::{Lifeline, Service, Task};
use subquery_s2s::types::OriginType;

use relay_s2s::header::RelaychainHeaderRunner;
use relay_s2s::types::RelaychainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct RococoToPangolinHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for RococoToPangolinHeaderRelayService {}

impl Service for RococoToPangolinHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("rococo-to-pangolin-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "pangolin-pangolinparachain",
                    "[header-relay] [rococo-to-pangolin] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "pangolin-pangolinparachain",
                    "[header-relay] [rococo-to-pangolin] Try to restart header relay service.",
                );
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangolinparachain",
        "[header-rococo-to-pangolin] [rococo-to-pangolin] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachain)?;

    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;
    let client_rococo = bridge_config.rococo.to_rococo_client().await?;

    let config_index = bridge_config.index;
    let subquery_rococo = config_index.to_rococo_subquery();
    let subquery_pangolin_parachain = config_index.to_pangolin_parachain_subquery();
    let subquery_candidate = config_index.to_candidate_subquery();

    let input = RelaychainHeaderInput {
        client_relaychain: client_rococo,
        client_solochain: client_pangolin,
        subquery_relaychain: subquery_rococo,
        subquery_parachain: subquery_pangolin_parachain,
        index_origin_type: OriginType::BridgePangolin,
        subquery_candidate,
        enable_mandatory: bridge_config.relay.enable_mandatory,
    };
    let runner = RelaychainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
