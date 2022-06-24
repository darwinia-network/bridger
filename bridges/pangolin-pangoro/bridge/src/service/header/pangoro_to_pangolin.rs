use lifeline::{Lifeline, Service, Task};
use subquery_s2s::types::OriginType;

use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct PangoroToPangolinHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangoroToPangolinHeaderRelayService {}

impl Service for PangoroToPangolinHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("pangoro-to-pangolin-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "pangolin-pangoro",
                    "[header-relay] [pangoro-to-pangolin] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "pangolin-pangoro",
                    "[header-relay] [pangoro-to-pangolin] Try to restart header relay service.",
                );
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangoro",
        "[header-pangolin-to-pangoro] [pangoro-to-pangolin] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let relay_config = bridge_config.relay;

    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;
    let client_pangoro = bridge_config.pangoro.to_pangoro_client().await?;

    let config_index = bridge_config.index;
    let subquery_pangoro = config_index.to_pangoro_subquery();
    let lanes = relay_config.raw_lanes();

    let input = SolochainHeaderInput {
        lanes,
        client_source: client_pangoro,
        client_target: client_pangolin,
        subquery_source: subquery_pangoro,
        index_origin_type: OriginType::BridgePangolin,
    };
    let runner = SolochainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
