use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use lifeline::{Lifeline, Service, Task};

use client_pangolin_parachain::client::PangolinParachainClient;
use client_pangolin_parachain::component::PangolinParachainClientComponent;
use subquery_s2s::types::BridgeName;
use subquery_s2s::{Subquery, SubqueryComponent};
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
                start().await.map_err(|e| {
                    BridgerError::Custom(format!(
                        "Failed to start pangolin-to-parachain header relay: {:?}",
                        e
                    ))
                })?;
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
    subquery_parachain: Subquery,
}

impl HeaderRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: PangolinRococoConfig = Config::restore(Names::BridgePangolinRococo)?;

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
        let subquery_parachain = SubqueryComponent::component(
            config_index.pangolin_parachain,
            BridgeName::PangolinParachain,
        );
        Ok(Self {
            client_pangolin,
            client_parachain,
            subquery_pangolin,
            subquery_parachain,
        })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-rococo",
        "[header-pangolin-to-parachain] SERVICE RESTARTING..."
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
                            "[header-pangolin-to-parachain] Connection Error. Try to resend later",
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        header_relay = HeaderRelay::new().await?;
                    }
                }
                tracing::error!(
                    target: "pangolin-rococo",
                    "[header-pangolin-to-parachain] Failed to relay header: {:?}",
                    err
                );
            }
        }
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
                last_relayed_pangolin_hash_in_parachain.to_string()
            ))
        })?;
    let block_number = last_relayed_pangolin_block_in_parachain.block.header.number;
    let next_block = header_relay
        .subquery_pangolin
        .next_header(block_number)
        .await?;
    if next_block.is_none() {
        tracing::info!(
            target: "pangolin-rococo",
            "[header-pangolin-to-parachain] No more header to relay after block: {}",
            block_number,
        );
        return Ok(());
    }
    let next_block = next_block.unwrap();
    let justification = header_relay
        .subquery_pangolin
        .find_justification(next_block.block_hash, next_block.is_mandatory())
        .await?;
    if justification.is_none() {
        tracing::error!(
            target: "pangolin-rococo",
            "[header-pangolin-to-parachain] No more header to relay after block: {}, block info: {:?}",
            block_number,
            next_block,
        );
        return Ok(());
    }
    let justification = justification.unwrap();

    let finality_target =
        client_pangolin_parachain::types::runtime_types::sp_runtime::generic::header::Header {
            parent_hash: Default::default(),
            number: next_block.block_number,
            state_root: Default::default(),
            extrinsics_root: Default::default(),
            digest: Digest {},
            __subxt_unused_type_params: Default::default(),
        };
    let grandpa_justification = client_pangolin_parachain::types::runtime_types::bp_header_chain::justification::GrandpaJustification {
        round: (),
        commit: Commit {},
        votes_ancestries: vec![]
    };
    let hash = header_relay
        .client_parachain
        .runtime()
        .tx()
        .bridge_pangolin_grandpa()
        .submit_finality_proof(finality_target, grandpa_justification)
        .sign_and_submit(header_relay.client_parachain.account().signer())
        .await?;
    tracing::info!(
        target: "pangolin-rococo",
        "[header-pangolin-to-parachain] The block {} relay emitted",
        hash
    );
    Ok(())
}
