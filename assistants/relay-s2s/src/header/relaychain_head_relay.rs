use std::str::FromStr;

use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay};
use bridge_s2s_traits::types::bp_header_chain;
use bridge_s2s_traits::types::bp_runtime::Chain;
use sp_runtime::codec;
use sp_runtime::traits::Header;
use subquery::types::NeedRelayBlock;

use support_toolkit::{convert::SmartCodecMapper, logk};

use crate::error::{RelayError, RelayResult};
use crate::types::{RelaychainHeaderInput, M_HEADER};

/// relay chain to solo chain header relay runner
pub struct RelaychainHeaderRunner<SC: S2SClientGeneric, TC: S2SClientRelay> {
    input: RelaychainHeaderInput<SC, TC>,
}

impl<SC: S2SClientGeneric, TC: S2SClientRelay> RelaychainHeaderRunner<SC, TC> {
    pub fn new(input: RelaychainHeaderInput<SC, TC>) -> Self {
        Self { input }
    }
}

impl<SC: S2SClientGeneric, TC: S2SClientRelay> RelaychainHeaderRunner<SC, TC> {
    pub async fn start(&self) -> RelayResult<()> {
        loop {
            self.run().await?;
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    async fn run(&self) -> RelayResult<()> {
        let client_relaychain = &self.input.client_relaychain;
        let client_solochain = &self.input.client_solochain;
        let last_relayed_relaychain_block_in_solochain =
            match client_solochain.best_target_finalized(None).await? {
                Some(v) => v,
                None => {
                    tracing::warn!(
                        target: "relay-s2s",
                        "{} the bridge not initialized.please init first.",
                        logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
                    );
                    return Ok(());
                }
            };
        let expected_relaychain_hash =
            SmartCodecMapper::map_to(&last_relayed_relaychain_block_in_solochain.1)?;
        // tracing::debug!(
        //     target: "relay-s2s",
        //     "{} get last relayed relaychain block hash: {:?}",
        //     logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
        //     array_bytes::bytes2hex("0x", expected_relaychain_hash.as_ref()),
        // );
        let last_relayed_relaychain_block_in_solochain = client_relaychain
            .block(Some(expected_relaychain_hash))
            .await?
            .ok_or_else(|| {
                RelayError::Custom(format!(
                    "Failed to query block by [{}] in relaychain",
                    array_bytes::bytes2hex("0x", expected_relaychain_hash)
                ))
            })?;

        let block_number = last_relayed_relaychain_block_in_solochain
            .block
            .header
            .number();
        let block_number: u32 = SmartCodecMapper::map_to(block_number)?;
        tracing::info!(
            target: "relay-s2s",
            "{} get last relayed relaychain block number: {:?}",
            logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
            block_number,
        );

        let subquery_relaychain = &self.input.subquery_relaychain;
        let next_mandatory_block = subquery_relaychain
            .next_mandatory_header(block_number)
            .await?;

        match next_mandatory_block {
            Some(block_to_relay) => {
                if self.input.enable_mandatory {
                    self.try_to_relay_mandatory(block_to_relay).await?;
                } else {
                    tracing::warn!(
                        target: "relay-s2s",
                        "{} found mandatory header ({}) but you disabled relay it.",
                        logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
                        block_to_relay.block_number,
                    );
                }
            }
            None => {
                self.try_to_relay_header_on_demand(block_number).await?;
            }
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
            "{} header relayed: {:?}",
            logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
            array_bytes::bytes2hex("0x", hash),
        );
        Ok(())
    }

    /// Try to relay mandatory headers, return Ok(Some(block_number)) if success, else Ok(None)
    async fn try_to_relay_mandatory(&self, block_to_relay: NeedRelayBlock) -> RelayResult<()> {
        tracing::info!(
            target: "relay-s2s",
            "{} the next mandatory block: {:?}",
            logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
            &block_to_relay.block_number,
        );
        let subquery_relaychain = &self.input.subquery_relaychain;
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

        Ok(())
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
        tracing::trace!(
            target: "relay-s2s",
            "{} queryied last need relay parachain header is {}",
            logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
            next_para_header.block_number,
        );

        let subquery_relaychain = &self.input.subquery_relaychain;
        let next_header = subquery_relaychain
            .get_block_with_para_head(next_para_header.block_hash)
            .await?
            .filter(|header| {
                tracing::debug!(
                    target: "relay-s2s",
                    "{} get related realy chain header: {}, last relayed header at solochain is {}",
                    logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
                    header.included_relay_block,
                    last_block_number,
                );
                header.included_relay_block > last_block_number
            });

        if next_header.is_none() {
            tracing::debug!(
                target: "relay-s2s",
                "{} para head has not been finalized",
                logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
            );
            return Ok(());
        }
        let next_header = next_header.expect("Unreachable");

        match crate::keepstate::get_recently_justification(SC::CHAIN) {
            Some(justification) => {
                let grandpa_justification: bp_header_chain::justification::GrandpaJustification<
                    <SC::Chain as Chain>::Header,
                > = codec::Decode::decode(&mut justification.as_ref()).map_err(|err| {
                    RelayError::Custom(format!(
                        "Failed to decode justification of relaychain: {:?}",
                        err
                    ))
                })?;
                tracing::debug!(
                    target: "relay-s2s",
                    "{} found justification at block {} and last block number is {}",
                    logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
                    grandpa_justification.commit.target_number,
                    last_block_number,
                );

                let target_number: u32 =
                    SmartCodecMapper::map_to(&grandpa_justification.commit.target_number)?;
                if target_number > last_block_number {
                    self.submit_finality(
                        array_bytes::bytes2hex(
                            "",
                            grandpa_justification.commit.target_hash.as_ref(),
                        ),
                        justification.to_vec(),
                    )
                    .await?;
                }
            }
            None => {
                tracing::warn!(
                    target: "relay-s2s",
                    "{} found on-demand block {}, but not have justification to relay.",
                    logk::prefix_with_bridge(M_HEADER, SC::CHAIN, TC::CHAIN),
                    next_header.para_head,
                );
            }
        }

        Ok(())
    }
}
