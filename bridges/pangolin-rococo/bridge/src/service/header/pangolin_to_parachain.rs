use client_pangolin::component::PangolinClientComponent;
use client_pangolin_parachain::component::PangolinParachainClientComponent;
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{PangolinRococoBus, PangolinRococoConfig, PangolinRococoTask};

#[derive(Debug)]
pub struct PangolinToParachainHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangolinToParachainHeaderRelayService {}

impl Service for PangolinToParachainHeaderRelayService {
    type Bus = PangolinRococoBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!(
                "{}-pangolin-parachain-header-relay",
                PangolinRococoTask::name()
            ),
            async move {
                if let Err(e) = start() {
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

fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-rococo",
        "[header-relay-pangolin-to-parachain] SERVICE RESTARTING..."
    );

    let bridge_config: PangolinRococoConfig = Config::restore(Names::BridgePangolinRococo)?;

    let config_pangolin = bridge_config.pangolin;
    let config_parachain = bridge_config.pangolin_parachain;

    let client_pangolin =
        PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?).await?;
    let client_parachain = PangolinParachainClientComponent::component(
        config_parachain.to_pangolin_parachain_client_config()?,
    )
    .await?;

    let last_relayed_pangolin_hash_in_parachain = client_parachain
        .runtime()
        .storage()
        .bridge_pangolin_grandpa()
        .best_finalized(None)
        .await?;
    let last_relayed_pangolin_block_in_parachain = client_pangolin
        .subxt()
        .rpc()
        .block(Some(last_relayed_pangolin_hash_in_parachain))
        .await?
        .ok_or_else(BridgerError::Custom(format!(
            "Failed to query block by [{}] in pangolin",
            last_relayed_pangolin_hash_in_parachain.to_string()
        )))?;
}
