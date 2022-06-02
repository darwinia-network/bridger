use client_crab::client::CrabClient;
use client_crab::component::CrabClientComponent;
use client_crab::types::runtime_types::{self as crab_runtime_types};
use client_kusama::client::KusamaClient;
use client_kusama::component::KusamaClientComponent;
use client_kusama::types::runtime_types as kusama_runtime_types;
use lifeline::{Lifeline, Service, Task};
use sp_runtime::traits::Hash;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, BridgeTask};

#[derive(Debug)]
pub struct KusamaToCrabParaHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for KusamaToCrabParaHeaderRelayService {}

impl Service for KusamaToCrabParaHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-kusama-crab-header-relay", BridgeTask::name()),
            async move {
                while let Err(e) = start().await {
                    tracing::error!(
                        target: "crab-crabparachain",
                        "Failed to start para-head-to-crab header relay service, restart after some seconds: {:?}",
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
    client_crab: CrabClient,
    client_kusama: KusamaClient,
    para_id: u32,
}

impl HeaderRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;

        let config_crab = bridge_config.crab;
        let config_kusama = bridge_config.kusama;

        let client_crab =
            CrabClientComponent::component(config_crab.to_crab_client_config()?).await?;
        let client_kusama =
            KusamaClientComponent::component(config_kusama.to_kusama_client_config()?).await?;

        Ok(Self {
            client_crab,
            client_kusama,
            para_id: bridge_config
                .crab_parachain
                .para_id
                .expect("ParaId not found"),
        })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "crab-crabparachain",
        "[para-header-kusama-to-crab] SERVICE RESTARTING..."
    );
    let mut header_relay = HeaderRelay::new().await?;
    loop {
        match run(&header_relay).await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!(
                    target: "crab-crabparachain",
                    "[para-header-kusama-to-crab] Failed to relay header: {:?}",
                    err
                );
                header_relay = HeaderRelay::new().await?;
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn run(header_relay: &HeaderRelay) -> color_eyre::Result<()> {
    let best_target_header = header_relay
        .client_crab
        .subxt()
        .rpc()
        .header(None)
        .await?
        .ok_or_else(|| BridgerError::Custom(String::from("Failed to get crab header")))?;
    tracing::trace!(
        target: "crab-crabparachain",
        "[para-head-relay-kusama-to-crab] Current crab block: {:?}",
        &best_target_header.number,
    );

    // TODO Hardcode ParaId
    let para_head_at_target = header_relay
        .client_crab
        .runtime()
        .storage()
        .bridge_kusama_parachain()
        .best_para_heads(
            crab_runtime_types::bp_polkadot_core::parachains::ParaId(header_relay.para_id),
            Some(best_target_header.hash()),
        )
        .await?;
    tracing::trace!(
        target: "crab-crabparachain",
        "[para-head-relay-kusama-to-crab] The latest para-head on crab: {:?}",
        &para_head_at_target,
    );

    let best_finalized_source_block_hash = header_relay
        .client_crab
        .runtime()
        .storage()
        .bridge_kusama_grandpa()
        .best_finalized(Some(best_target_header.hash()))
        .await?;

    let best_finalized_source_block_at_target = header_relay
        .client_kusama
        .subxt()
        .rpc()
        .block(Some(best_finalized_source_block_hash))
        .await?
        .ok_or_else(|| BridgerError::Custom("Failed to get Kusama block".to_string()))?;
    tracing::trace!(
        target: "crab-crabparachain",
        "[para-head-relay-kusama-to-crab] The latest kusama block on crab: {:?}",
        &best_finalized_source_block_at_target.block.header.number,
    );

    // TODO Hardcode ParaId
    let para_head_at_source = header_relay
        .client_kusama
        .runtime()
        .storage()
        .paras()
        .heads(
            kusama_runtime_types::polkadot_parachain::primitives::Id(header_relay.para_id),
            Some(best_finalized_source_block_hash),
        )
        .await?;
    tracing::trace!(
        target: "crab-crabparachain",
        "[para-head-relay-kusama-to-crab] The latest para-head on kusama {:?}",
        &best_finalized_source_block_at_target.block.header.number,
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
                target: "crab-crabparachain",
                "[para-head-relay-kusama-to-crab] Parachain is unknown to both clients"
            );
            false
        }
        (Some(_), Some(_)) => {
            tracing::info!(
                target: "crab-crabparachain",
                "[para-head-relay-kusama-to-crab] It doesn't need to relay"
            );
            false
        }
    };

    if need_relay {
        let heads_proofs = header_relay
            .client_kusama
            .subxt()
            .rpc()
            .read_proof(
                vec![bp_parachains::parachain_head_storage_key_at_source(
                    "Paras",
                    header_relay.para_id.into(),
                )],
                Some(best_finalized_source_block_hash),
            )
            .await?;
        tracing::info!(
            target: "crab-crabparachain",
            "[para-head-relay-kusama-to-crab] Submitting parachain heads update transaction to crab",
        );

        let runtime = header_relay.client_crab.runtime();
        let track = runtime
            .tx()
            .bridge_kusama_parachain()
            .submit_parachain_heads(
                best_finalized_source_block_hash,
                vec![crab_runtime_types::bp_polkadot_core::parachains::ParaId(
                    header_relay.para_id,
                )],
                heads_proofs
                    .proof
                    .into_iter()
                    .map(|bytes| bytes.0)
                    .collect(),
            )
            .sign_and_submit_then_watch(header_relay.client_crab.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await?;
        tracing::info!(
            target: "crab-crabparachain",
            "[para-head-relay-kusama-to-crab] The tx hash {:?} emitted",
            events.extrinsic_hash()
        );
    }

    Ok(())
}
