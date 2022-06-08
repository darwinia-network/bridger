use std::str::FromStr;

use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::runtime_types::bp_header_chain::justification::GrandpaJustification;
use client_pangolin::types::runtime_types::sp_runtime::generic::header::Header;
use client_pangolin::types::runtime_types::sp_runtime::traits::BlakeTwo256;
use codec::{Decode, Encode};
use lifeline::{Lifeline, Service, Task};

use client_pangolin_parachain::client::PangolinParachainClient;
use client_pangolin_parachain::component::PangolinParachainClientComponent;
use client_pangolin_parachain::types::runtime_types::sp_runtime::generic::header::Header as FinalityTarget;
use subquery_s2s::types::{BridgeName, OriginType};
use subquery_s2s::{Subquery, SubqueryComponent};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};
use crate::service::subscribe::PANGOLIN_JUSTIFICATIONS;

#[derive(Debug)]
pub struct PangolinToParachainHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangolinToParachainHeaderRelayService {}

impl Service for PangolinToParachainHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-pangolin-parachain-header-relay", BridgeTask::name()),
            async move {
                while let Err(e) = start().await {
                    tracing::error!(
                        target: "pangolin-pangolinparachain",
                        "Failed to start pangolin-to-parachain header relay service, restart after some seconds: {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

struct HeaderRelay {
    client_pangolin: PangolinClient,
    client_parachain: PangolinParachainClient,
    subquery_pangolin: Subquery,
}

impl HeaderRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachain)?;

        let config_pangolin = bridge_config.pangolin;
        let config_parachain = bridge_config.pangolin_parachain;

        let client_pangolin =
            PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?)
                .await?;
        let client_parachain = PangolinParachainClientComponent::component(
            config_parachain.to_pangolin_parachain_client_config()?,
        )
        .await?;

        let config_index = bridge_config.index;
        let subquery_pangolin =
            SubqueryComponent::component(config_index.pangolin, BridgeName::PangolinParachain);
        Ok(Self {
            client_pangolin,
            client_parachain,
            subquery_pangolin,
        })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangolinparachain",
        "[header-pangolin-to-parachain] SERVICE RESTARTING..."
    );
    let mut header_relay = HeaderRelay::new().await?;
    loop {
        match run(&header_relay).await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!(
                    target: "pangolin-pangolinparachain",
                    "[header-pangolin-to-parachain] Failed to relay header: {:?}",
                    err
                );
                header_relay = HeaderRelay::new().await?;
                
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn run(header_relay: &HeaderRelay) -> color_eyre::Result<()> {
    let last_relayed_pangolin_hash_in_parachain = header_relay
        .client_parachain
        .runtime()
        .storage()
        .bridge_pangolin_grandpa()
        .best_finalized(None)
        .await?;
    let last_relayed_pangolin_block_in_parachain = header_relay
        .client_pangolin
        .subxt()
        .rpc()
        .block(Some(last_relayed_pangolin_hash_in_parachain))
        .await?
        .ok_or_else(|| {
            BridgerError::Custom(format!(
                "Failed to query block by [{}] in pangolin",
                last_relayed_pangolin_hash_in_parachain
            ))
        })?;
    let block_number = last_relayed_pangolin_block_in_parachain.block.header.number;
    tracing::trace!(
        target: "pangolin-pangolinparachain",
        "[header-pangolin-to-parachain] The latest relayed pangolin block is: {:?}",
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
        .subquery_pangolin
        .next_mandatory_header(last_block_number)
        .await?;
    if let Some(block_to_relay) = next_mandatory_block {
        tracing::info!(
            target: "pangolin-pangolinparachain",
            "[header-pangolin-to-parachain] Next mandatory block: {:?} ",
            &block_to_relay.block_number
        );
        let justification = header_relay
            .subquery_pangolin
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
            target: "pangolin-pangolinparachain",
            "[header-pangolin-to-parachain] Next mandatory block not found",
        );
        Ok(None)
    }
}

async fn try_to_relay_header_on_demand(
    header_relay: &HeaderRelay,
    last_block_number: u32,
) -> color_eyre::Result<()> {
    let next_header = header_relay
        .subquery_pangolin
        .next_needed_header(OriginType::BridgePangolinParachain)
        .await?
        .filter(|header| header.block_number > last_block_number);

    if next_header.is_none() {
        return Ok(());
    }

    let pangolin_justification_queue = PANGOLIN_JUSTIFICATIONS.lock().await;
    if let Some(justification) = pangolin_justification_queue.back().cloned() {
        let grandpa_justification =
            GrandpaJustification::<Header<u32, BlakeTwo256>>::decode(&mut justification.as_ref())
                .map_err(|err| {
                BridgerError::Custom(format!(
                    "Failed to decode justification of pangolin: {:?}",
                    err
                ))
            })?;
        if grandpa_justification.commit.target_number > last_block_number {
            submit_finality(
                header_relay,
                format!("{:#x}", grandpa_justification.commit.target_hash),
                justification.to_vec(),
            )
            .await?;
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
        .client_pangolin
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
    let runtime = header_relay.client_parachain.runtime();
    let track = runtime
        .tx()
        .bridge_pangolin_grandpa()
        .submit_finality_proof(finality_target, grandpa_justification)
        .sign_and_submit_then_watch(header_relay.client_parachain.account().signer())
        .await?;

    let events = track.wait_for_finalized_success().await?;
    tracing::info!(
         target: "pangolin-pangolinparachain",
          "[header-pangolin-to-parachain] The extrinsic hash: {:?}",
           events.extrinsic_hash()
    );
    Ok(())
}
