use bridge_s2s_traits::client::{S2SParaBridgeClientRelaychain, S2SParaBridgeClientSolochain};
use bridge_s2s_traits::types::ParaId;
use sp_runtime::traits::Header;

use support_toolkit::{convert::SmartCodecMapper, logk};

use crate::error::{RelayError, RelayResult};
use crate::types::{ParaHeaderInput, M_PARA_HEAD};

/// para head to solo chain header relay runner
pub struct ParaHeaderRunner<SC: S2SParaBridgeClientRelaychain, TC: S2SParaBridgeClientSolochain> {
    input: ParaHeaderInput<SC, TC>,
}

impl<SC: S2SParaBridgeClientRelaychain, TC: S2SParaBridgeClientSolochain> ParaHeaderRunner<SC, TC> {
    pub fn new(input: ParaHeaderInput<SC, TC>) -> Self {
        Self { input }
    }
}

impl<SC: S2SParaBridgeClientRelaychain, TC: S2SParaBridgeClientSolochain> ParaHeaderRunner<SC, TC> {
    pub async fn start(&self) -> RelayResult<()> {
        loop {
            self.run().await?;
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    async fn run(&self) -> RelayResult<()> {
        let client_solochain = &self.input.client_solochain;
        let client_relaychain = &self.input.client_relaychain;

        let best_target_header = client_solochain
            .header(None)
            .await?
            .ok_or_else(|| RelayError::Custom(format!("Failed to get {} header", SC::CHAIN)))?;
        tracing::trace!(
            target: "relay-s2s",
            "{} current {} block: {:?}",
            logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
            SC::CHAIN,
            best_target_header.number(),
        );
        let para_head_at_target = client_solochain
            .best_para_heads(ParaId(self.input.para_id), Some(best_target_header.hash()))
            .await?;
        tracing::trace!(
            target: "relay-s2s",
            "{} the last para-head on {}: {}",
            logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
            SC::CHAIN,
            if let Some(phat) = &para_head_at_target {
                format!("{}@{}", phat.best_head_hash.at_relay_block_number, phat.best_head_hash.head_hash)
            } else {
                "".to_string()
            }
        );

        let best_finalized_source_block_hash = match client_solochain
            .best_target_finalized(Some(best_target_header.hash()))
            .await?
        {
            Some(v) => v,
            None => {
                tracing::warn!(
                    target: "relay-s2s",
                    "{} no best target finalized block queried from {} at block {}@{}",
                    logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
                    TC::CHAIN,
                    best_target_header.number(),
                    best_target_header.hash(),
                );
                return Ok(());
            }
        };

        let expected_source_block_hash =
            SmartCodecMapper::map_to(&best_finalized_source_block_hash)?;
        let best_finalized_source_block_at_target = client_relaychain
            .block(Some(expected_source_block_hash))
            .await?
            .ok_or_else(|| RelayError::Custom("Failed to get Rococo block".to_string()))?;
        // todo: fix this types
        let best_finalized_source_block_at_target_number: u32 =
            SmartCodecMapper::map_to(best_finalized_source_block_at_target.block.header.number())?;
        tracing::trace!(
            target: "relay-s2s",
            "{} the last relaychain block on solochain: {:?}",
            logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
            best_finalized_source_block_at_target_number,
        );
        let para_head_at_source = client_relaychain
            .para_head_data(ParaId(self.input.para_id), Some(expected_source_block_hash))
            .await?;
        tracing::trace!(
            target: "relay-s2s",
            "{} the last para-head on relaychain {:?}",
            logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
            best_finalized_source_block_at_target_number,
        );

        let mut para_head_hash = None;
        let need_relay = match (para_head_at_source, para_head_at_target) {
            (Some(head_at_source), Some(head_at_target))
                if head_at_target.best_head_hash.at_relay_block_number
                    < best_finalized_source_block_at_target_number
                    && head_at_target.best_head_hash.head_hash != head_at_source.hash() =>
            {
                para_head_hash = Some(head_at_source.hash());
                true
            }
            (Some(head_at_source), None) => {
                para_head_hash = Some(head_at_source.hash());
                true
            }
            (None, Some(head_at_target)) => {
                para_head_hash = Some(head_at_target.best_head_hash.head_hash);
                true
            }
            (None, None) => {
                tracing::info!(
                    target: "relay-s2s",
                    "{} parachain is unknown to both clients",
                    logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
                );
                false
            }
            (Some(_), Some(_)) => {
                tracing::info!(
                    target: "relay-s2s",
                    "{} not need to relay",
                    logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
                );
                false
            }
        };

        if !need_relay {
            return Ok(());
        }

        let heads_proofs = client_relaychain
            .read_proof(
                vec![client_relaychain.gen_parachain_head_storage_key(self.input.para_id)],
                Some(expected_source_block_hash),
            )
            .await?;
        tracing::info!(
            target: "relay-s2s",
            "{} submitting parachain heads update transaction to {}",
            logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
            SC::CHAIN,
        );

        let hash = client_solochain
            .submit_parachain_heads(
                best_finalized_source_block_hash,
                vec![(
                    ParaId(self.input.para_id),
                    SmartCodecMapper::map_to(&para_head_hash.expect("Unreachable"))?,
                )],
                heads_proofs,
            )
            .await?;
        tracing::info!(
            target: "relay-s2s",
            "{} the tx hash {} emitted",
            logk::prefix_with_bridge(M_PARA_HEAD, SC::CHAIN, TC::CHAIN),
            array_bytes::bytes2hex("0x", hash),
        );
        Ok(())
    }
}
