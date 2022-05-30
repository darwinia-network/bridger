use std::str::FromStr;

use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::runtime_types::bp_header_chain::justification::GrandpaJustification;
use client_pangolin::types::runtime_types::sp_runtime::generic::header::Header;
use client_pangolin::types::runtime_types::sp_runtime::traits::BlakeTwo256;
use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use client_pangoro::types::runtime_types::sp_runtime::generic::header::Header as FinalityTarget;
use codec::{Decode, Encode};
use lifeline::{Lifeline, Service, Task};
use subquery_s2s::types::{BridgeName, OriginType};
use subquery_s2s::{Subquery, SubqueryComponent};

use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};
use crate::service::subscribe::PANGOLIN_JUSTIFICATIONS;

#[derive(Debug)]
pub struct PangolinToPangoroHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangolinToPangoroHeaderRelayService {}

impl Service for PangolinToPangoroHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("pangolin-to-pangoro-header-relay-service", async move {
            start().await.map_err(|e| {
                BridgerError::Custom(format!(
                    "Failed to start pangolin-to-pangoro header relay: {:?}",
                    e
                ))
            })?;
            Ok(())
        });
        Ok(Self { _greet })
    }
}

struct HeaderRelay {
    client_pangolin: PangolinClient,
    client_pangoro: PangoroClient,
    subquery_pangolin: Subquery,
}

impl HeaderRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

        let config_pangolin = bridge_config.pangolin;
        let config_pangoro = bridge_config.pangoro;

        let client_pangolin =
            PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?)
                .await?;
        let client_pangoro =
            PangoroClientComponent::component(config_pangoro.to_pangoro_client_config()?).await?;

        let config_index = bridge_config.index;
        let subquery_pangolin =
            SubqueryComponent::component(config_index.pangolin, BridgeName::PangolinPangoro);
        Ok(Self {
            client_pangolin,
            client_pangoro,
            subquery_pangolin,
        })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangoro",
        "[header-pangolin-to-pangoro] SERVICE RESTARTING..."
    );
    let mut header_relay = HeaderRelay::new().await?;
    loop {
        match run(&header_relay).await {
            Ok(_) => {}
            Err(err) => {
                if let Some(e) = err.downcast_ref::<client_pangolin::error::ClientError>() {
                    if e.is_restart_need() {
                        tracing::error!(
                            target: "pangolin-pangoro",
                            "[header-pangolin-to-pangoro] Connection Error. Try to resend later: {:?}",
                            e
                        );
                        header_relay = HeaderRelay::new().await?;
                    }
                }
                if let Some(e) = err.downcast_ref::<client_pangoro::error::ClientError>() {
                    if e.is_restart_need() {
                        tracing::error!(
                            target: "pangolin-pangoro",
                            "[header-pangolin-to-pangoro] Connection Error. Try to resend later: {:?}",
                            e
                        );
                        header_relay = HeaderRelay::new().await?;
                    }
                }
                tracing::error!(
                    target: "pangolin-pangoro",
                    "[header-pangolin-to-pangoro] Failed to relay header: {:?}",
                    err
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn run(header_relay: &HeaderRelay) -> color_eyre::Result<()> {
    let last_relayed_pangolin_hash_in_pangoro = header_relay
        .client_pangoro
        .runtime()
        .storage()
        .bridge_pangolin_grandpa()
        .best_finalized(None)
        .await?;
    let last_relayed_pangolin_block_in_pangoro = header_relay
        .client_pangolin
        .subxt()
        .rpc()
        .block(Some(last_relayed_pangolin_hash_in_pangoro))
        .await?
        .ok_or_else(|| {
            BridgerError::Custom(format!(
                "Failed to query block by [{}] in pangolin",
                last_relayed_pangolin_hash_in_pangoro
            ))
        })?;
    let block_number = last_relayed_pangolin_block_in_pangoro.block.header.number;
    tracing::trace!(
        target: "pangolin-pangoro",
        "[header-pangolin-to-pangoro] The latest relayed pangolin block is: {:?}",
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
            target: "pangolin-pangoro",
            "[header-pangolin-to-pangoro] Next mandatory block: {:?} ",
            &block_to_relay.block_number
        );
        let justification = header_relay
            .subquery_pangolin
            .find_justification(block_to_relay.block_hash.clone(), true)
            .await?
            .ok_or_else(|| {
                BridgerError::Custom(format!(
                    "Failed to query justification for block hash: {:?}",
                    &block_to_relay.block_hash
                ))
            })?;
        submit_finality(
            header_relay,
            block_to_relay.block_hash,
            justification.justification,
        )
        .await?;

        return Ok(Some(block_to_relay.block_number));
    }
    tracing::info!(
        target: "pangolin-pangoro",
        "[header-pangolin-to-pangoro] Next mandatory block not found",
    );
    Ok(None)
}

async fn try_to_relay_header_on_demand(
    header_relay: &HeaderRelay,
    last_block_number: u32,
) -> color_eyre::Result<()> {
    let next_header = header_relay
        .subquery_pangolin
        .next_needed_header(OriginType::BridgePangoro)
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
                array_bytes::bytes2hex("", grandpa_justification.commit.target_hash.0),
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
    let grandpa_justification = Decode::decode(&mut justification.as_slice())?;
    let runtime = header_relay.client_pangoro.runtime();
    let track = runtime
        .tx()
        .bridge_pangolin_grandpa()
        .submit_finality_proof(finality_target, grandpa_justification)
        .sign_and_submit_then_watch(header_relay.client_pangoro.account().signer())
        .await?;

    let events = track.wait_for_finalized_success().await?;
    tracing::info!(
         target: "pangolin-pangoro",
          "[header-pangolin-to-pangoro] The extrinsic hash: {:?}",
           events.extrinsic_hash()
    );
    Ok(())
}
