use std::str::FromStr;

use abstract_bridge_s2s::client::S2SClientRelay;
use abstract_bridge_s2s::config::Config;
use abstract_bridge_s2s::types::bp_header_chain;
use sp_runtime::codec;
use sp_runtime::traits::Header;
use support_toolkit::convert::SmartCodecMapper;

use crate::error::{RelayError, RelayResult};
use crate::helpers;
use crate::types::{SolochainHeaderInput, M_HEADER};

/// solo chain to solo chain header relay runner
pub struct SolochainHeaderRunner<SC: S2SClientRelay, TC: S2SClientRelay> {
    input: SolochainHeaderInput<SC, TC>,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> SolochainHeaderRunner<SC, TC> {
    pub fn new(input: SolochainHeaderInput<SC, TC>) -> Self {
        Self { input }
    }
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> SolochainHeaderRunner<SC, TC> {
    /// start header relay
    pub async fn start(&self) -> RelayResult<()> {
        loop {
            self.run().await?;
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    async fn run(&self) -> RelayResult<()> {
        let client_source = &self.input.client_source;
        let client_target = &self.input.client_target;

        let last_relayed_source_hash_in_target = client_target.best_target_finalized(None).await?;
        let expected_source_hash = SmartCodecMapper::map_to(&last_relayed_source_hash_in_target)?;
        let last_relayed_source_block_in_target = client_source
            .block(Some(expected_source_hash))
            .await?
            .ok_or_else(|| {
                RelayError::Custom(format!(
                    "Failed to query block by [{}] in {}",
                    array_bytes::bytes2hex("0x", expected_source_hash),
                    SC::CHAIN,
                ))
            })?;

        let block_number = last_relayed_source_block_in_target.block.header.number();
        let block_number: u32 = SmartCodecMapper::map_to(block_number)?;
        tracing::trace!(
            target: "relay-s2s",
            "{} the last relayed {} block is: {:?}",
            helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
            SC::CHAIN,
            block_number,
        );

        if self.try_to_relay_mandatory(block_number).await?.is_none() {
            self.try_to_relay_header_on_demand(block_number).await?;
        }

        Ok(())
    }

    async fn submit_finality(
        &self,
        block_hash: impl AsRef<str>,
        justification: Vec<u8>,
    ) -> RelayResult<()> {
        let client_source = &self.input.client_source;
        let client_target = &self.input.client_target;
        let block_hash = block_hash.as_ref();
        let block_hash = sp_core::H256::from_str(block_hash).map_err(|e| {
            RelayError::Custom(format!("Wrong block hash [{}] {:?}", block_hash, e))
        })?;
        let expected_block_hash = SmartCodecMapper::map_to(&block_hash)?;
        let header = client_source
            .header(Some(expected_block_hash))
            .await?
            .ok_or_else(|| {
                RelayError::Custom(format!("Not found header by hash: {}", block_hash))
            })?;
        let grandpa_justification =
            sp_runtime::codec::Decode::decode(&mut justification.as_slice())?;
        let expected_header = SmartCodecMapper::map_to(&header)?;
        let hash = client_target
            .submit_finality_proof(expected_header, grandpa_justification)
            .await?;
        tracing::info!(
            target: "relay-s2s",
            "{} header relayed: {:?}",
            helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
            array_bytes::bytes2hex("0x", hash),
        );
        Ok(())
    }

    /// Try to relay mandatory headers, return Ok(Some(block_number)) if success, else Ok(None)
    async fn try_to_relay_mandatory(&self, last_block_number: u32) -> RelayResult<Option<u32>> {
        let subquery_source = &self.input.subquery_source;
        let next_mandatory_block = subquery_source
            .next_mandatory_header(last_block_number)
            .await?;
        if let Some(block_to_relay) = next_mandatory_block {
            tracing::info!(
                target: "relay-s2s",
                "{} the next mandatory block: {:?} ",
                helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
                &block_to_relay.block_number
            );
            let justification = subquery_source
                .find_justification(block_to_relay.block_hash.clone(), true)
                .await?
                .ok_or_else(|| {
                    RelayError::Custom(format!(
                        "Failed to query justification for block hash: {:?}",
                        &block_to_relay.block_hash
                    ))
                })?;
            self.submit_finality(block_to_relay.block_hash, justification.justification)
                .await?;

            return Ok(Some(block_to_relay.block_number));
        }
        tracing::info!(
            target: "relay-s2s",
            "{} the next mandatory block not found",
            helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
        );
        Ok(None)
    }

    async fn try_to_relay_header_on_demand(&self, last_block_number: u32) -> RelayResult<()> {
        let subquery_source = &self.input.subquery_source;
        let next_header = match subquery_source
            .next_needed_header(self.input.index_origin_type.clone())
            .await?
        {
            Some(v) => {
                if v.block_number <= last_block_number {
                    tracing::debug!(
                        target: "relay-s2s",
                        "{} the last storage block ({}) is less or equal last relayed block ({}). nothing to do.",
                        helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
                        v.block_number,
                        last_block_number,
                    );
                    return Ok(());
                }
                v
            }
            None => {
                tracing::debug!(
                    target: "relay-s2s",
                    "{} try relay header on-demand, but not found any on-demand block",
                    helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
                );
                return Ok(());
            }
        };
        tracing::debug!(
            target: "relay-s2s",
            "{} try relay header on-demand, the on-demand block is {}",
            helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
            next_header.block_number,
        );

        match crate::subscribe::recently_justification(SC::CHAIN)? {
            Some(justification) => {
                tracing::trace!(
                    target: "relay-s2s",
                    "{} found on-demand block {}, and found new justification, ready to relay header",
                    helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
                    next_header.block_number,
                );
                let grandpa_justification: bp_header_chain::justification::GrandpaJustification<
                    <SC::Config as Config>::Header,
                > = codec::Decode::decode(&mut justification.as_ref()).map_err(|err| {
                    RelayError::Custom(format!(
                        "Failed to decode justification of {}: {:?}",
                        SC::CHAIN,
                        err,
                    ))
                })?;
                let target_number: u32 =
                    SmartCodecMapper::map_to(&grandpa_justification.commit.target_number)?;
                if target_number > last_block_number {
                    self.submit_finality(
                        array_bytes::bytes2hex("", grandpa_justification.commit.target_hash),
                        justification.to_vec(),
                    )
                    .await?;
                }
            }
            None => {
                tracing::warn!(
                    target: "relay-s2s",
                    "{} found on-demand block {}, but not have justification to relay.",
                    helpers::log_prefix(M_HEADER, SC::CHAIN, TC::CHAIN),
                    next_header.block_number,
                );
            }
        }

        Ok(())
    }
}
