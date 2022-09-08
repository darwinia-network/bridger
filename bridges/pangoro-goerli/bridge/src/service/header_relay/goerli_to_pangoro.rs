use std::{ops::Div, str::FromStr};

use crate::{
    bridge::{BridgeBus, BridgeConfig},
    goerli_client::{client::GoerliClient, types::FinalityUpdate},
    pangoro_client::client::PangoroClient,
};
use client_contracts::beacon_light_client::FinalizedHeaderUpdate;
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use web3::{
    ethabi::ethereum_types::H32,
    types::{Bytes, H256},
};

#[derive(Debug)]
pub struct GoerliToPangoroHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for GoerliToPangoroHeaderRelayService {}

impl Service for GoerliToPangoroHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("header-goerli-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-goerli",
                    "Failed to start goerli-to-pangoro header relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;
    let pangoro_client = PangoroClient::new(
        &config.pangoro_evm.endpoint,
        &config.pangoro_evm.contract_address,
        &config.pangoro_evm.execution_layer_contract_address,
        &config.pangoro_evm.private_key,
    )?;
    let goerli_client = GoerliClient::new(&config.goerli.endpoint)?;
    let header_relay = HeaderRelay {
        pangoro_client,
        goerli_client,
    };

    loop {
        if let Err(error) = header_relay.header_relay().await {
            tracing::error!(
                target: "pangoro-goerli",
                "Failed relay header : {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

#[derive(Debug)]
pub struct HeaderRelayState {
    // Latest relayed header slot at Pangoro
    pub relayed_slot: u64,
    // Latest relayed period at Pangoro
    pub relayed_period: u64,
    // Latest header slot at Goerli
    pub current_slot: u64,
    // Latest period at Goerli
    pub current_period: u64,
}

pub struct HeaderRelay {
    pub pangoro_client: PangoroClient,
    pub goerli_client: GoerliClient,
}

impl HeaderRelay {
    pub async fn get_state(&self) -> color_eyre::Result<HeaderRelayState> {
        let relayed = self
            .pangoro_client
            .beacon_light_client
            .finalized_header()
            .await?;
        let current_head = self.goerli_client.get_header("head").await?;
        let current_slot = current_head.header.message.slot;
        let current_period = current_slot.div(32).div(256);
        let relayed_period = relayed.slot.div(32).div(256);
        Ok(HeaderRelayState {
            relayed_slot: relayed.slot,
            relayed_period,
            current_slot,
            current_period,
        })
    }

    pub async fn header_relay(&self) -> color_eyre::Result<()> {
        let state = self.get_state().await?;
        let next_sync_aggregate_root = self
            .pangoro_client
            .beacon_light_client
            .sync_committee_roots(state.relayed_period + 1)
            .await?;
        tracing::info!(
            target: "pangoro-goerli",
            "[Header][Goerli=>Pangoro] State: {:?}",
            state
        );
        if state.current_period == state.relayed_period {
            self.relay_latest(state).await
        } else {
            if next_sync_aggregate_root.is_zero() {
                return Ok(());
            }
            if state.current_period == state.relayed_period + 1 {
                self.relay_latest(state).await
            } else {
                self.relay_next_period(state).await
            }
        }
    }

    pub async fn relay_latest(&self, state: HeaderRelayState) -> color_eyre::Result<()> {
        let finality_update: FinalityUpdate = self.goerli_client.get_finality_update().await?;
        let update_finality_slot = finality_update.finalized_header.slot;
        let update_finality_period = update_finality_slot.div(32).div(256);

        tracing::info!(
            target: "pangoro-goerli",
            "[Header][Goerli=>Pangoro] Latest finality slot: {:?}",
            &update_finality_slot
        );
        // The latest finality header has been relayed
        if update_finality_slot == state.relayed_slot {
            return Ok(());
        }

        let (_slot, sync_aggregate_slot, _attested_header, _sync_aggregate_block) = match self
            .goerli_client
            .find_valid_attested_header(state.current_slot, finality_update.attested_header.slot)
            .await?
        {
            None => {
                tracing::info!(
                    target: "pangoro-goerli",
                    "[Header][Goerli=>Pangoro] Wait for valid attested header",
                );
                return Ok(());
            }
            Some((slot, sync_aggregate_slot, attested_header, sync_aggregate_block)) => (
                slot,
                sync_aggregate_slot,
                attested_header,
                sync_aggregate_block,
            ),
        };

        let sync_change = self
            .goerli_client
            .get_sync_committee_period_update(update_finality_period - 1, 1)
            .await?;
        if sync_change.is_empty() {
            return Err(BridgerError::Custom("Failed to get sync committee update".into()).into());
        }
        let fork_version = self
            .goerli_client
            .get_fork_version(sync_aggregate_slot)
            .await?;

        let finalized_header_update = FinalizedHeaderUpdate {
            attested_header: finality_update.attested_header.to_contract_type()?,
            signature_sync_committee: sync_change[0].next_sync_committee.to_contract_type()?,
            finalized_header: finality_update.finalized_header.to_contract_type()?,
            finality_branch: finality_update
                .finality_branch
                .iter()
                .map(|x| H256::from_str(x))
                .collect::<Result<Vec<H256>, _>>()?,
            sync_aggregate: finality_update.sync_aggregate.to_contract_type()?,
            fork_version: Bytes(fork_version.current_version.as_ref().to_vec()),
            signature_slot: sync_aggregate_slot,
        };
        let tx = self
            .pangoro_client
            .beacon_light_client
            .import_finalized_header(finalized_header_update, &self.pangoro_client.private_key)
            .await?;

        Ok(())
    }

    pub async fn relay_next_period(&self, state: HeaderRelayState) -> color_eyre::Result<()> {
        let _target_period = state.relayed_period + 1;
        let sync_change = self
            .goerli_client
            .get_sync_committee_period_update(state.relayed_period, 2)
            .await?;

        if let [last_finality, target_finality] = sync_change.as_slice() {
            let attested_slot: u64 = target_finality.attested_header.slot;
            let (_slot, sync_aggregate_slot, _attested_header, _sync_aggregate_block) = match self
                .goerli_client
                .find_valid_attested_header(state.current_slot, attested_slot)
                .await?
            {
                None => {
                    tracing::info!(
                        target: "pangoro-goerli",
                        "[Header][Goerli=>Pangoro] Wait for valid attested header",
                    );
                    return Ok(());
                }
                Some((slot, sync_aggregate_slot, attested_header, sync_aggregate_block)) => (
                    slot,
                    sync_aggregate_slot,
                    attested_header,
                    sync_aggregate_block,
                ),
            };
            let fork_version = H32::from_str(&target_finality.fork_version)?
                .as_ref()
                .to_vec();

            let finalized_header_update = FinalizedHeaderUpdate {
                attested_header: target_finality.attested_header.to_contract_type()?,
                signature_sync_committee: last_finality.next_sync_committee.to_contract_type()?,
                finalized_header: target_finality.finalized_header.to_contract_type()?,
                finality_branch: target_finality
                    .finality_branch
                    .iter()
                    .map(|x| H256::from_str(x))
                    .collect::<Result<Vec<H256>, _>>()?,
                sync_aggregate: target_finality.sync_aggregate.to_contract_type()?,
                fork_version: Bytes(fork_version),
                signature_slot: sync_aggregate_slot,
            };
            let tx = self
                .pangoro_client
                .beacon_light_client
                .import_finalized_header(finalized_header_update, &self.pangoro_client.private_key)
                .await?;
            Ok(())
        } else {
            Err(BridgerError::Custom("Failed to get sync committee update".into()).into())
        }
    }
}
