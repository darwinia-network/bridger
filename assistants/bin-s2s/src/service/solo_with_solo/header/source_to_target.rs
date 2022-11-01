use lifeline::{Lifeline, Service, Task};
use subquery::types::OriginType;

use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct SourceToTargetHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for SourceToTargetHeaderRelayService {}

impl Service for SourceToTargetHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("source-to-target-header-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "bind-s2s",
                    "[header-relay] [source-to-target] An error occurred for header relay {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bind-s2s",
                    "[header-relay] [source-to-target] Try to restart header relay service.",
                );
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "bind-s2s",
        "[header-source-to-target] [source-to-target] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::Bridgetargetsource)?;
    let relay_config = bridge_config.relay;

    let client_source = bridge_config.source.to_source_client().await?;
    let client_target = bridge_config.target.to_target_client().await?;

    let config_index = bridge_config.index;
    let subquery_source = config_index.to_source_subquery();

    let input = SolochainHeaderInput {
        client_source: client_source,
        client_target: client_target,
        subquery_source: subquery_source,
        index_origin_type: OriginType::Bridgetarget,
        enable_mandatory: relay_config.enable_mandatory,
    };
    let runner = SolochainHeaderRunner::new(input);
    Ok(runner.start().await?)
}
