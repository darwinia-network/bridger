use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::runtime_types::sp_runtime::generic::header::Header as FinalityTarget;
use client_kusama::client::KusamaClient;
use client_kusama::component::KusamaClientComponent;
use client_kusama::types::runtime_types::bp_header_chain::justification::GrandpaJustification;
use client_kusama::types::runtime_types::sp_runtime::generic::header::Header;
use client_kusama::types::runtime_types::sp_runtime::traits::BlakeTwo256;
use lifeline::{Lifeline, Service, Task};
use std::str::FromStr;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use codec::{Decode, Encode};

use subquery_s2s::types::{BridgeName, OriginType};
use subquery_s2s::{Subquery, SubqueryComponent};

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};
use crate::service::subscribe::ROCOCO_JUSTIFICATIONS;

#[derive(Debug)]
pub struct KusamaToPangolinHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for KusamaToPangolinHeaderRelayService {}

impl Service for KusamaToPangolinHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-kusama-pangolin-header-relay", BridgeTask::name()),
            async move {
                if let Err(e) = start().await {
                    tracing::error!(
                        target: "pangolin-crabparachain",
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
    client_kusama: KusamaClient,
    subquery_kusama: Subquery,
    subquery_crab_parachain: Subquery,
    subquery_parachain_kusama: subquery_parachain::Subquery,
}

impl HeaderRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinCrabParachain)?;

        let config_pangolin = bridge_config.pangolin;
        let config_kusama = bridge_config.kusama;

        let client_pangolin =
            PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?)
                .await?;
        let client_kusama =
            KusamaClientComponent::component(config_kusama.to_kusama_client_config()?).await?;

        let config_index = bridge_config.index;
        let subquery_kusama =
            SubqueryComponent::component(config_index.kusama, BridgeName::CrabParachain);
        let subquery_crab_parachain = SubqueryComponent::component(
            config_index.crab_parachain,
            BridgeName::CrabParachain,
        );
        let subquery_parachain_kusama = subquery_parachain::SubqueryComponent::component(
            config_index.parachain_kusama,
            subquery_parachain::types::BridgeName::CrabParachain,
        );

        Ok(Self {
            client_pangolin,
            client_kusama,
            subquery_kusama,
            subquery_crab_parachain,
            subquery_parachain_kusama,
        })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-crabparachain",
        "[header-kusama-to-pangolin] SERVICE RESTARTING..."
    );
    let mut header_relay = HeaderRelay::new().await?;
    loop {
        match run(&header_relay).await {
            Ok(_) => {}
            Err(err) => {
                if let Some(subxt::BasicError::Rpc(request_error)) =
                    err.downcast_ref::<subxt::BasicError>()
                {
                    tracing::error!(
                        target: "pangolin-crabparachain",
                        "[header-kusama-to-pangolin] Connection Error. Try to resend later: {:?}",
                        &request_error
                    );
                    header_relay = HeaderRelay::new().await?;
                }
                tracing::error!(
                    target: "pangolin-crabparachain",
                    "[header-kusama-to-pangolin] Failed to relay header: {:?}",
                    err
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn run(header_relay: &HeaderRelay) -> color_eyre::Result<()> {
    let last_relayed_kusama_hash_in_pangolin = header_relay
        .client_pangolin
        .runtime()
        .storage()
        .bridge_kusama_grandpa()
        .best_finalized(None)
        .await?;
    tracing::debug!(
        target: "pangolin-crabparachain",
        "[header-relay-kusama-to-pangolin] Get last relayed kusama block hash: {:?}",
        &last_relayed_kusama_hash_in_pangolin
    );

    let last_relayed_kusama_block_in_pangolin = header_relay
        .client_kusama
        .subxt()
        .rpc()
        .block(Some(last_relayed_kusama_hash_in_pangolin))
        .await?
        .ok_or_else(|| {
            BridgerError::Custom(format!(
                "Failed to query block by [{}] in kusama",
                last_relayed_kusama_hash_in_pangolin
            ))
        })?;

    let block_number = last_relayed_kusama_block_in_pangolin.block.header.number;
    tracing::info!(
        target: "pangolin-crabparachain",
        "[header-relay-kusama-to-pangolin] Get last relayed kusama block number: {:?}",
        block_number
    );
    if try_to_relay_mandatory(header_relay, block_number)
        .await?
        .is_none()
    {
        try_to_relay_header_on_demand(header_relay, block_number).await?;
    }

    Ok(())
}

/// Try to relay mandatory headers, return Ok(Some(block_number)) if success, else Ok(None)
async fn try_to_relay_mandatory(
    header_relay: &HeaderRelay,
    last_block_number: u32,
) -> color_eyre::Result<Option<u32>> {
    let next_mandatory_block = header_relay
        .subquery_kusama
        .next_mandatory_header(last_block_number)
        .await?;

    if let Some(block_to_relay) = next_mandatory_block {
        tracing::info!(
            target: "pangolin-crabparachain",
            "[header-relay-kusama-to-pangolin] Next mandatory block: {:?}",
            &block_to_relay.block_number,
        );
        let justification = header_relay
            .subquery_kusama
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
            target: "pangolin-crabparachain",
            "[header-relay-kusama-to-pangolin] Next mandatory block not found",
        );
        Ok(None)
    }
}

async fn try_to_relay_header_on_demand(
    header_relay: &HeaderRelay,
    last_block_number: u32,
) -> color_eyre::Result<()> {
    let next_para_header = header_relay
        .subquery_crab_parachain
        .next_needed_header(OriginType::BridgePangolin)
        .await?;

    if next_para_header.is_none() {
        return Ok(());
    }

    if let Some(next_para_header) = next_para_header {
        let next_header = header_relay
            .subquery_parachain_kusama
            .get_block_with_para_head(next_para_header.block_hash)
            .await?
            .filter(|header| {
                tracing::debug!(
                    target: "pangolin-crabparachain",
                    "[header-relay-kusama-to-pangolin] Get related realy chain header: {:?}",
                    header.included_relay_block
                );
                header.included_relay_block > last_block_number
            });

        if next_header.is_none() {
            tracing::debug!(
                target: "pangolin-crabparachain",
                "[header-relay-kusama-to-pangolin] Para head has not been finalized"
            );
            return Ok(());
        }

        let pangolin_justification_queue = ROCOCO_JUSTIFICATIONS.lock().await;
        if let Some(justification) = pangolin_justification_queue.back().cloned() {
            let grandpa_justification = GrandpaJustification::<Header<u32, BlakeTwo256>>::decode(
                &mut justification.as_ref(),
            )
            .map_err(|err| {
                BridgerError::Custom(format!(
                    "Failed to decode justification of kusama: {:?}",
                    err
                ))
            })?;
            tracing::debug!(
                target: "pangolin-crabparachain",
                "[header-relay-kusama-to-pangolin] Test justification: {:?}",
                grandpa_justification.commit.target_number
            );
            if grandpa_justification.commit.target_number > last_block_number {
                submit_finality(
                    header_relay,
                    format!("{:#x}", grandpa_justification.commit.target_hash),
                    justification.to_vec(),
                )
                .await?;
            }
        }
    }

    Ok(())
}

async fn submit_finality(
    header_relay: &HeaderRelay,
    block_hash: impl AsRef<str>,
    justification: Vec<u8>,
) -> color_eyre::Result<()> {
    let header = header_relay
        .client_kusama
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
    let runtime = header_relay.client_pangolin.runtime();
    let track = runtime
        .tx()
        .bridge_kusama_grandpa()
        .submit_finality_proof(finality_target, grandpa_justification)
        .sign_and_submit_then_watch(header_relay.client_pangolin.account().signer())
        .await?;

    let events = track.wait_for_finalized_success().await?;
    tracing::info!(
        target: "pangolin-crabparachain",
        "[header-kusama-to-pangolin] The extrinsic hash: {:?}",
        events.extrinsic_hash()
    );
    Ok(())
}
