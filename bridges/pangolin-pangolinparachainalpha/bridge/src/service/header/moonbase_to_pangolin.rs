use lifeline::{Lifeline, Service, Task};
use subquery::types::OriginType;

use relay_s2s::header::RelaychainHeaderRunner;
use relay_s2s::types::RelaychainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct MoonbaseToPangolinHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for MoonbaseToPangolinHeaderRelayService {}

impl Service for MoonbaseToPangolinHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("moonbase-to-pangolin-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "pangolin-pangolinparachainalpha",
                    "[header-relay] [moonbase-to-pangolin] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "pangolin-pangolinparachainalpha",
                    "[header-relay] [moonbase-to-pangolin] Try to restart header relay service.",
                );
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangolinparachainalpha",
        "[header-moonbase-to-pangolin] [moonbase-to-pangolin] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachainAlpha)?;

    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;
    let client_moonbase = bridge_config.moonbase.to_moonbase_client().await?;

    let config_index = bridge_config.index;
    let subquery_moonbase = config_index.to_moonbase_subquery();
    let subquery_pangolin_parachain = config_index.to_pangolin_parachain_subquery();

    let input = RelaychainHeaderInput {
        client_relaychain: client_moonbase,
        client_solochain: client_pangolin,
        subquery_relaychain: subquery_moonbase,
        subquery_parachain: subquery_pangolin_parachain,
        index_origin_type: OriginType::BridgePangolin,
        enable_mandatory: bridge_config.relay.enable_mandatory,
    };
    let runner = RelaychainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
