use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::runtime_types::sp_runtime::generic::header::Header as FinalityTarget;
use client_rococo::client::RococoClient;
use client_rococo::component::RococoClientComponent;
use lifeline::{Lifeline, Service, Task};
use std::str::FromStr;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use codec::{Decode, Encode};

use subquery_s2s::types::BridgeName;
use subquery_s2s::{Subquery, SubqueryComponent};

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

struct HeaderRelay {
    client_pangolin: PangolinClient,
    client_rococo: RococoClient,
    subquery_rococo: Subquery,
}

impl HeaderRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: PangolinRococoConfig = Config::restore(Names::BridgePangolinRococo)?;

        let config_pangolin = bridge_config.pangolin;
        let config_rococo = bridge_config.rococo;

        let client_pangolin =
            PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?)
                .await?;
        let client_rococo =
            RococoClientComponent::component(config_rococo.to_rococo_client_config()?).await?;

        let config_index = bridge_config.index;
        let subquery_rococo =
            SubqueryComponent::component(config_index.rococo, BridgeName::PangolinParachain);

        Ok(Self {
            client_pangolin,
            client_rococo,
            subquery_rococo,
        })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-rococo",
        "[header-rococo-to-pangolin] SERVICE RESTARTING..."
    );
    let mut header_relay = HeaderRelay::new().await?;
    loop {
        match run(&header_relay).await {
            Ok(_) => continue,
            Err(err) => {
                if let Some(client_error) =
                    err.downcast_ref::<client_pangolin::error::ClientError>()
                {
                    if client_error.is_restart_need() {
                        tracing::error!(
                            target: "pangolin-rococo",
                            "[header-rococo-to-pangolin] Connection Error. Try to resend later",
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        header_relay = HeaderRelay::new().await?;
                    }
                }
                tracing::error!(
                    target: "pangolin-rococo",
                    "[header-rococo-to-pangolin] Failed to relay header: {:?}",
                    err
                );
            }
        }
    }
}

async fn run(header_relay: &HeaderRelay) -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-rococo",
        "[header-relay-pangolin-to-parachain] SERVICE RESTARTING..."
    );
    tracing::info!(
        target: "pangolin-rococo",
        "[header-relay-pangolin-to-parachain] Begin",
    );

    let last_relayed_rococo_hash_in_pangolin = header_relay
        .client_pangolin
        .runtime()
        .storage()
        .bridge_rococo_grandpa()
        .best_finalized(None)
        .await?;
    tracing::info!(
        target: "pangolin-rococo",
        "[header-relay-pangolin-to-parachain] Get last relayed rococo block hash: {:?}",
        &last_relayed_rococo_hash_in_pangolin
    );

    let last_relayed_rococo_block_in_pangolin = header_relay
        .client_rococo
        .subxt()
        .rpc()
        .block(Some(last_relayed_rococo_hash_in_pangolin))
        .await?
        .ok_or_else(|| {
            BridgerError::Custom(format!(
                "Failed to query block by [{}] in rococo",
                last_relayed_rococo_hash_in_pangolin
            ))
        })?;

    let block_number = last_relayed_rococo_block_in_pangolin.block.header.number;
    tracing::info!(
        target: "pangolin-rococo",
        "[header-relay-pangolin-to-parachain] Get last relayed rococo block number: {:?}",
        block_number
    );
    if try_to_relay_mandatory(header_relay, block_number)
        .await?
        .is_none()
    {
        try_to_relay_non_mandatory(header_relay, block_number).await?;
    }

    Ok(())
}

/// Try to relay mandatory headers, return Ok(Some(block_number)) if success, else Ok(None)
async fn try_to_relay_mandatory(
    header_relay: &HeaderRelay,
    last_block_number: u32,
) -> color_eyre::Result<Option<u32>> {
    let next_mandatory_block = header_relay
        .subquery_rococo
        .next_mandatory_header(last_block_number)
        .await?;

    if let Some(block_to_relay) = next_mandatory_block {
        tracing::info!(
            target: "pangolin-rococo",
            "[header-relay-pangolin-to-parachain] Next mandatory block: {:?}",
            &block_to_relay.block_number,
        );
        let justification = header_relay
            .subquery_rococo
            .find_justification(block_to_relay.block_hash.clone(), true)
            .await?;
        submit_finality(
            header_relay,
            block_to_relay.block_hash,
            justification.unwrap().justification,
        )
        .await?;

        Ok(Some(block_to_relay.block_number))
    } else {
        tracing::info!(
            target: "pangolin-rococo",
            "[header-relay-pangolin-to-parachain] Next mandatory block not found",
        );
        Ok(None)
    }
}

async fn try_to_relay_non_mandatory(
    header_relay: &HeaderRelay,
    last_block_number: u32,
) -> color_eyre::Result<()> {
    let latest_justification = header_relay
        .subquery_rococo
        .find_justification("", false)
        .await?
        .ok_or_else(|| {
            BridgerError::Custom("Failed to query latest justification in rococo".to_string())
        })?;
    if latest_justification.block_number > last_block_number {
        submit_finality(
            header_relay,
            latest_justification.block_hash,
            latest_justification.justification,
        )
        .await?;
    }
    Ok(())
}

async fn submit_finality(
    header_relay: &HeaderRelay,
    block_hash: impl AsRef<str>,
    justification: Vec<u8>,
) -> color_eyre::Result<()> {
    let header = header_relay
        .client_rococo
        .subxt()
        .rpc()
        .header(Some(sp_core::H256::from_str(block_hash.as_ref()).unwrap()))
        .await?
        .unwrap();
    let finality_target = FinalityTarget {
        parent_hash: header.parent_hash,
        number: header.number,
        state_root: header.state_root,
        extrinsics_root: header.extrinsics_root,
        digest: Decode::decode(&mut header.digest.encode().as_slice())?,
        __subxt_unused_type_params: Default::default(),
    };
    let grandpa_justification = codec::Decode::decode(&mut justification.as_slice())?;
    let hash = header_relay
        .client_pangolin
        .runtime()
        .tx()
        .bridge_rococo_grandpa()
        .submit_finality_proof(finality_target, grandpa_justification)
        .sign_and_submit(header_relay.client_pangolin.account().signer())
        .await?;
    tracing::info!(
        target: "pangolin-rococo",
        "[header-rococo-to-pangolin] The block {} relay emitted",
        hash
    );
    Ok(())
}
