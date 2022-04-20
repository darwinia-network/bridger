use client_pangolin::component::PangolinClientComponent;
use client_rococo::component::RococoClientComponent;
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use subquery_s2s::types::BridgeName;
use subquery_s2s::SubqueryComponent;

use crate::bridge::{PangolinRococoBus, PangolinRococoConfig, PangolinRococoTask};

#[derive(Debug)]
pub struct RococoToPangolinHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for RococoToPangolinHeaderRelayService {}

impl Service for RococoToPangolinHeaderRelayService {
    type Bus = PangolinRococoBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!(
                "{}-rococo-pangolin-header-relay",
                PangolinRococoTask::name()
            ),
            async move {
                if let Err(e) = start().await {
                    tracing::error!(
                        target: "pangolin-rococo",
                        "{:?}",
                        e,
                    );
                    return Err(BridgerError::Custom(
                        "Failed to start header relay service".to_string(),
                    )
                    .into());
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-rococo",
        "[header-relay-pangolin-to-parachain] SERVICE RESTARTING..."
    );

    let bridge_config: PangolinRococoConfig = Config::restore(Names::BridgePangolinRococo)?;

    let config_pangolin = bridge_config.pangolin;
    let config_rococo = bridge_config.pangolin_parachain;

    let client_pangolin =
        PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?).await?;
    let client_rococo =
        RococoClientComponent::component(config_rococo.to_rococo_client_config()?).await?;

    let subquery =
        SubqueryComponent::component(bridge_config.index.rococo, BridgeName::PangolinParachain);

    let last_relayed_rococo_hash_in_pangolin = client_pangolin
        .runtime()
        .storage()
        .bridge_rococo_grandpa()
        .best_finalized(None)
        .await?;
    let last_relayed_pangolin_header_in_rococo = client_rococo
        .subxt()
        .rpc()
        .header(Some(last_relayed_rococo_hash_in_pangolin))
        .await?
        .ok_or_else(|| {
            BridgerError::Custom(format!(
                "Failed to query block by [{}] in rococo",
                last_relayed_rococo_hash_in_pangolin.to_string()
            ))
        })?;

    Ok(())
}
