use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::runtime_types::{self as pangolin_runtime_types};
use client_rococo::client::RococoClient;
use client_rococo::component::RococoClientComponent;
use client_rococo::types::runtime_types as rococo_runtime_types;
use lifeline::{Lifeline, Service, Task};
use sp_runtime::traits::Hash;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{PangolinRococoBus, PangolinRococoConfig, PangolinRococoTask};

#[derive(Debug)]
pub struct RococoToPangolinParaHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for RococoToPangolinParaHeaderRelayService {}

impl Service for RococoToPangolinParaHeaderRelayService {
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

        Ok(Self {
            client_pangolin,
            client_rococo,
        })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-rococo",
        "[para-header-rococo-to-pangolin] SERVICE RESTARTING..."
    );
    let mut header_relay = HeaderRelay::new().await?;
    loop {
        match run(&header_relay).await {
            Ok(_) => continue,
            Err(err) => {
                if let Some(subxt::BasicError::Rpc(request_error)) =
                    err.downcast_ref::<subxt::BasicError>()
                {
                    tracing::error!(
                        target: "pangolin-rococo",
                        "[para-header-rococo-to-pangolin] Connection Error. Try to resend later: {:?}",
                        request_error
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    header_relay = HeaderRelay::new().await?;
                }
                tracing::error!(
                    target: "pangolin-rococo",
                    "[para-header-rococo-to-pangolin] Failed to relay header: {:?}",
                    err
                );
            }
        }
    }
}

async fn run(header_relay: &HeaderRelay) -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-rococo",
        "[para-head-relay-rococo-to-pangolin] SERVICE RESTARTING..."
    );

    let best_target_header = header_relay
        .client_pangolin
        .subxt()
        .rpc()
        .header(None)
        .await?
        .ok_or_else(|| BridgerError::Custom(String::from("Failed to get pangolin header")))?;
    tracing::info!(
        target: "pangolin-rococo",
        "[para-head-relay-rococo-to-pangolin] Current pangolin block: {:?}",
        &best_target_header.number,
    );

    // TODO Hardcode ParaId
    let para_head_at_target = header_relay
        .client_pangolin
        .runtime()
        .storage()
        .bridge_rococo_parachains()
        .best_para_heads(
            pangolin_runtime_types::bp_polkadot_core::parachains::ParaId(2105),
            Some(best_target_header.hash()),
        )
        .await?;
    tracing::info!(
        target: "pangolin-rococo",
        "[para-head-relay-rococo-to-pangolin] The latest para-head on pangolin: {:?}",
        &para_head_at_target,
    );

    let best_finalized_source_block_hash = header_relay
        .client_pangolin
        .runtime()
        .storage()
        .bridge_rococo_grandpa()
        .best_finalized(Some(best_target_header.hash()))
        .await?;

    let best_finalized_source_block_at_target = header_relay
        .client_rococo
        .subxt()
        .rpc()
        .block(Some(best_finalized_source_block_hash))
        .await?
        .ok_or_else(|| BridgerError::Custom("Failed to get Rococo block".to_string()))?;
    tracing::info!(
        target: "pangolin-rococo",
        "[para-head-relay-rococo-to-pangolin] The latest rococo block on pangolin: {:?}",
        &best_finalized_source_block_at_target.block.header.number,
    );

    // TODO Hardcode ParaId
    let para_head_at_source = header_relay
        .client_rococo
        .runtime()
        .storage()
        .paras()
        .heads(
            rococo_runtime_types::polkadot_parachain::primitives::Id(2105),
            Some(best_finalized_source_block_hash),
        )
        .await?;
    tracing::info!(
        target: "pangolin-rococo",
        "[para-head-relay-rococo-to-pangolin] The latest para-head on rococo {:?} : {:?}",
        &best_finalized_source_block_at_target.block.header.number,
        &para_head_at_source,
    );

    let need_relay = match (para_head_at_source, para_head_at_target) {
        (Some(head_at_source), Some(head_at_target))
            if head_at_target.at_relay_block_number
                < best_finalized_source_block_at_target.block.header.number
                && head_at_target.head_hash
                    != sp_runtime::traits::BlakeTwo256::hash(head_at_source.0.as_slice()) =>
        {
            true
        }
        (Some(_), None) => true,
        (None, Some(_)) => true,

        (None, None) => {
            tracing::info!(
                target: "pangolin-rococo",
                "[para-head-relay-rococo-to-pangolin] Parachain is unknown to both clients"
            );
            false
        }
        (Some(_), Some(_)) => {
            tracing::info!(
                target: "pangolin-rococo",
                "[para-head-relay-rococo-to-pangolin] It doesn't need to relay"
            );
            false
        }
    };

    if need_relay {
        let heads_proofs = header_relay
            .client_rococo
            .subxt()
            .rpc()
            .read_proof(
                vec![bp_parachains::parachain_head_storage_key_at_source(
                    "Paras",
                    2105.into(),
                )],
                Some(best_finalized_source_block_hash),
            )
            .await?;
        tracing::info!(
            target: "pangolin-rococo",
            "[para-head-relay-rococo-to-pangolin] Submitting parachain heads update transaction to pangolin",
        );

        let runtime = header_relay.client_pangolin.runtime();
        let track = runtime
            .tx()
            .bridge_rococo_parachains()
            .submit_parachain_heads(
                best_finalized_source_block_hash,
                vec![pangolin_runtime_types::bp_polkadot_core::parachains::ParaId(2105)],
                heads_proofs
                    .proof
                    .into_iter()
                    .map(|bytes| bytes.0)
                    .collect(),
            )
            .sign_and_submit_then_watch(header_relay.client_pangolin.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await?;
        tracing::info!(
            target: "pangolin-rococo",
            "[para-head-relay-rococo-to-pangolin] The tx hash {:?} emitted",
            events.extrinsic_hash()
        );
    }

    Ok(())
}
