use std::str::FromStr;

use abstract_client_s2s::client::S2SClientRelay;
use abstract_client_s2s::config::Config;
use abstract_client_s2s::convert::SmartCodecMapper;
use abstract_client_s2s::types::bp_header_chain;
use sp_runtime::codec;
use sp_runtime::traits::Header;

use crate::error::{RelayError, RelayResult};
use crate::types::RelaychainHeaderInput;

/// relay chain to solo chain header relay runner
pub struct RelaychainHeaderRunner<SC: S2SClientRelay, TC: S2SClientRelay> {
    input: RelaychainHeaderInput<SC, TC>,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> RelaychainHeaderRunner<SC, TC> {
    pub async fn start(&self) -> RelayResult<()> {
        let client_relaychain = &self.input.client_relaychain;
        let client_solochain = &self.input.client_solochain;
        let last_relayed_rococo_hash_in_pangolin =
            client_solochain.best_target_finalized(None).await?;
        let expected_relaychain_hash =
            SmartCodecMapper::map_to(&last_relayed_rococo_hash_in_pangolin)?;
        tracing::debug!(
            target: "relay-s2s",
            "[header] [{}>{}] Get last relayed rococo block hash: {:?}",
            SC::CHAIN,
            TC::CHAIN,
            array_bytes::bytes2hex("0x", expected_relaychain_hash),
        );
        let last_relayed_rococo_block_in_pangolin = client_relaychain
            .block(Some(expected_relaychain_hash))
            .await?
            .ok_or_else(|| {
                RelayError::Custom(format!(
                    "Failed to query block by [{}] in rococo",
                    array_bytes::bytes2hex("0x", expected_relaychain_hash)
                ))
            })?;

        let block_number = last_relayed_rococo_block_in_pangolin.block.header.number();
        let block_number: u32 = SmartCodecMapper::map_to(block_number)?;
        tracing::info!(
            target: "relay-s2s",
            "[header] [{}>{}] Get last relayed rococo block number: {:?}",
            SC::CHAIN,
            TC::CHAIN,
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
        let client_relaychain = &self.input.client_relaychain;
        let client_solochain = &self.input.client_solochain;
        let block_hash = block_hash.as_ref();
        let block_hash = sp_core::H256::from_str(block_hash).map_err(|e| {
            RelayError::Custom(format!("Wrong block hash [{}] {:?}", block_hash, e))
        })?;
        let expected_block_hash = SmartCodecMapper::map_to(&block_hash)?;
        let header = client_relaychain
            .header(Some(expected_block_hash))
            .await?
            .ok_or_else(|| {
                RelayError::Custom(format!("Not found header by hash: {}", block_hash))
            })?;
        let grandpa_justification =
            sp_runtime::codec::Decode::decode(&mut justification.as_slice())?;
        let expected_header = SmartCodecMapper::map_to(&header)?;
        let hash = client_solochain
            .submit_finality_proof(expected_header, grandpa_justification)
            .await?;
        tracing::info!(
            target: "relay-s2s",
            "[header] [{}>{}] Header relayed: {:?}",
            SC::CHAIN,
            TC::CHAIN,
            array_bytes::bytes2hex("0x", hash),
        );
        Ok(())
    }

    /// Try to relay mandatory headers, return Ok(Some(block_number)) if success, else Ok(None)
    async fn try_to_relay_mandatory(&self, last_block_number: u32) -> RelayResult<Option<u32>> {
        let subquery_relaychain = &self.input.subquery_relaychain;

        let next_mandatory_block = subquery_relaychain
            .next_mandatory_header(last_block_number)
            .await?;

        if let Some(block_to_relay) = next_mandatory_block {
            tracing::info!(
                target: "relay-s2s",
                "[header] [{}>{}] Next mandatory block: {:?}",
                SC::CHAIN,
                TC::CHAIN,
                &block_to_relay.block_number,
            );
            let justification = subquery_relaychain
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

            Ok(Some(block_to_relay.block_number))
        } else {
            tracing::info!(
                target: "relay-s2s",
                "[header] [{}>{}] Next mandatory block not found",
                SC::CHAIN,
                TC::CHAIN,
            );
            Ok(None)
        }
    }

    async fn try_to_relay_header_on_demand(&self, last_block_number: u32) -> RelayResult<()> {
        let subquery_parachain = &self.input.subquery_parachain;
        let next_para_header = subquery_parachain
            .next_needed_header(self.input.index_origin_type.clone())
            .await?;
        if next_para_header.is_none() {
            return Ok(());
        }
        let next_para_header = next_para_header.expect("Unreachable");

        let subquery_candidate = &self.input.subquery_candidate;
        let next_header = subquery_candidate
            .get_block_with_para_head(next_para_header.block_hash)
            .await?
            .filter(|header| {
                tracing::debug!(
                    target: "relay-s2s",
                    "[header] [{}>{}] Get related realy chain header: {:?}",
                    SC::CHAIN,
                    TC::CHAIN,
                    header.included_relay_block,
                );
                header.included_relay_block > last_block_number
            });

        if next_header.is_none() {
            tracing::debug!(
                target: "relay-s2s",
                "[header] [{}>{}] Para head has not been finalized",
                SC::CHAIN,
                TC::CHAIN,
            );
            return Ok(());
        }
        let next_header = next_header.expect("Unreachable");

        match crate::subscribe::recently_justification(SC::CHAIN)? {
            Some(justification) => {
                let grandpa_justification: bp_header_chain::justification::GrandpaJustification<
                    <SC::Config as Config>::Header,
                > = codec::Decode::decode(&mut justification.as_ref()).map_err(|err| {
                    RelayError::Custom(format!(
                        "Failed to decode justification of rococo: {:?}",
                        err
                    ))
                })?;
                tracing::debug!(
                    target: "relay-s2s",
                    "[header] [{}>{}] Test justification: {:?}",
                    SC::CHAIN,
                    TC::CHAIN,
                    grandpa_justification.commit.target_number,
                );

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
                    "[header] [{}>{}] Found on-demand block {}, but not have justification to relay.",
                    SC::CHAIN,
                    TC::CHAIN,
                    next_header.para_head,
                );
            }
        }

        Ok(())
    }
}
