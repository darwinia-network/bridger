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
                    target: "pangolin-pangolinparachainalpha",
                    "[header-relay] [para-head-to-pangolin] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "pangolin-pangolinparachainalpha",
                    "[header-relay] [para-head-to-pangolin] Try to restart header relay service.",
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
        "[header-para-head-to-pangolin] [para-head-to-pangolin] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachainAlpha)?;
    let config_relay = bridge_config.relay;

    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;
    let client_moonbase = bridge_config.moonbase.to_moonbase_client().await?;

    let input = ParaHeaderInput {
        client_relaychain: client_moonbase,
        client_solochain: client_pangolin,
        para_id: config_relay.para_id,
    };
    let runner = ParaHeaderRunner::new(input);
    Ok(runner.start().await?)
}
