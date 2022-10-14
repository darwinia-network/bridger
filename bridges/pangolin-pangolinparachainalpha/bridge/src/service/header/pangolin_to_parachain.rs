use lifeline::{Lifeline, Service, Task};
use subquery::types::OriginType;

use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct PangolinToParachainAlphaHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangolinToParachainAlphaHeaderRelayService {}

impl Service for PangolinToParachainAlphaHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            "pangolin-to-pangolinparachain-header-relay-service",
            async move {
                while let Err(e) = start().await {
                    tracing::error!(
                        target: "pangolin-pangolinparachainalpha",
                        "[header-relay] [pangolin-to-pangolinparachain] An error occurred for header relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "pangolin-pangolinparachainalpha",
                        "[header-relay] [pangolin-to-pangolinparachain] Try to restart header relay service.",
                    );
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangolinparachainalpha",
        "[header-pangolin-to-pangolinparachain] [pangolin-to-pangolinparachain] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachainAlpha)?;
    let relay_config = bridge_config.relay;

    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;
    let client_pangolin_parachain = bridge_config
        .pangolin_parachain_alpha
        .to_pangolin_parachain_client()
        .await?;

    let config_index = bridge_config.index;
    let subquery_pangolin = config_index.to_pangolin_subquery();
    let lanes = relay_config.raw_lanes();

    let input = SolochainHeaderInput {
        lanes,
        client_source: client_pangolin,
        client_target: client_pangolin_parachain,
        subquery_source: subquery_pangolin,
        index_origin_type: OriginType::BridgePangolinParachainAlpha,
        enable_mandatory: relay_config.enable_mandatory,
    };
    let runner = SolochainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
