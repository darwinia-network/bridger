use std::{
    ops::Div,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bridge_e2e_traits::client::EthTruthLayerLightClient;
use client_beacon::{client::BeaconApiClient, types::FinalityUpdate};
use client_contracts::beacon_light_client_types::FinalizedHeaderUpdate;
use support_etherscan::wait_for_transaction_confirmation;
use web3::{
    contract::Options,
    types::{Bytes, H256, U256},
};

use crate::error::{RelayError, RelayResult};

pub struct BeaconHeaderRelayRunner<C: EthTruthLayerLightClient> {
    pub eth_light_client: C,
    pub beacon_api_client: BeaconApiClient,
    pub minimal_interval: u64,
    pub last_relay_time: u64,
}

#[derive(Debug)]
pub struct HeaderRelayState {
    // Latest relayed header slot at Darwinia
    pub relayed_slot: u64,
    // Latest relayed period at Darwinia
    pub relayed_period: u64,
    // Latest header slot at Beacon chain
    pub current_slot: u64,
    // Latest period at Beacon chain
    pub current_period: u64,
}

impl<C: EthTruthLayerLightClient> BeaconHeaderRelayRunner<C> {
    pub async fn start(&mut self) -> RelayResult<()> {
        loop {
            self.run().await?;
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }

    pub async fn run(&mut self) -> RelayResult<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RelayError::Custom(format!("{}", e)))?
            .as_secs();
        if now - self.last_relay_time <= self.minimal_interval {
            tracing::info!(
                target: "relay-e2e",
                "[Header] Last relaying time is {:?}, wait for {} seconds to start again.",
                self.last_relay_time,
                self.minimal_interval - (now - self.last_relay_time)
            );
            return Ok(());
        }

        let state = self.get_state().await?;
        let next_sync_aggregate_root = self
            .eth_light_client
            .beacon_light_client()
            .sync_committee_roots(state.relayed_period + 1)
            .await?;
        tracing::info!(
            target: "relay-e2e",
            "[Header] State: {:?}",
            state
        );
        if state.current_period == state.relayed_period {
            self.relay_latest(state).await?;
            return Ok(());
        }

        if next_sync_aggregate_root.is_zero() {
            return Ok(());
        }
        if state.current_period == state.relayed_period + 1 {
            self.relay_latest(state).await
        } else {
            self.relay_next_period(state).await
        }
    }

    pub async fn get_state(&self) -> RelayResult<HeaderRelayState> {
        let relayed = self
            .eth_light_client
            .beacon_light_client()
            .finalized_header()
            .await?;
        let current_head = self.beacon_api_client.get_header("head").await?;
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

    pub async fn relay_latest(&mut self, state: HeaderRelayState) -> RelayResult<()> {
        let finality_update: FinalityUpdate = self.beacon_api_client.get_finality_update().await?;
        let update_finality_slot = finality_update.finalized_header.beacon.slot;
        let update_finality_period = update_finality_slot.div(32).div(256);

        tracing::info!(
            target: "relay-e2e",
            "[Header] Latest finality slot: {:?}",
            &update_finality_slot
        );
        // The latest finality header has been relayed
        if update_finality_slot == state.relayed_slot {
            return Ok(());
        }

        let (signature_slot, _) = self
            .beacon_api_client
            .find_valid_header_since(
                state.current_slot,
                finality_update.attested_header.beacon.slot + 1,
            )
            .await?;
        let sync_change = self
            .beacon_api_client
            .get_sync_committee_period_update(update_finality_period - 1, 1)
            .await?;
        if sync_change.is_empty() {
            return Err(RelayError::Custom("Failed to get sync committee update".into()).into());
        }
        let fork_version = self.get_fork_version(signature_slot).await?;
        let finalized_header_update = FinalizedHeaderUpdate {
            attested_header: finality_update.attested_header.beacon.to_contract_type()?,
            signature_sync_committee: sync_change[0].next_sync_committee.to_contract_type()?,
            finalized_header: finality_update.finalized_header.beacon.to_contract_type()?,
            finality_branch: finality_update
                .finality_branch
                .iter()
                .map(|x| H256::from_str(x))
                .collect::<Result<Vec<H256>, _>>()
                .map_err(|e| RelayError::Custom(format!("{}", e)))?,
            sync_aggregate: finality_update.sync_aggregate.to_contract_type()?,
            fork_version: Bytes(fork_version.as_ref().to_vec()),
            signature_slot,
        };
        self.import_finalized_header_with_confirmation(finalized_header_update)
            .await?;
        Ok(())
    }

    pub async fn relay_next_period(&mut self, state: HeaderRelayState) -> RelayResult<()> {
        let _target_period = state.relayed_period + 1;
        let sync_change = self
            .beacon_api_client
            .get_sync_committee_period_update(state.relayed_period, 2)
            .await?;

        if let [last_finality, target_finality] = sync_change.as_slice() {
            let attested_slot: u64 = target_finality.attested_header.beacon.slot;
            let (signature_slot, _) = self
                .beacon_api_client
                .find_valid_header_since(state.current_slot, attested_slot + 1)
                .await?;
            let fork_version = self.get_fork_version(signature_slot).await?;
            let finalized_header_update = FinalizedHeaderUpdate {
                attested_header: target_finality.attested_header.beacon.to_contract_type()?,
                signature_sync_committee: last_finality.next_sync_committee.to_contract_type()?,
                finalized_header: target_finality.finalized_header.beacon.to_contract_type()?,
                finality_branch: target_finality
                    .finality_branch
                    .iter()
                    .map(|x| H256::from_str(x))
                    .collect::<Result<Vec<H256>, _>>()
                    .map_err(|e| RelayError::Custom(format!("{}", e)))?,
                sync_aggregate: target_finality.sync_aggregate.to_contract_type()?,
                fork_version: Bytes(fork_version.as_ref().to_vec()),
                signature_slot,
            };
            self.import_finalized_header_with_confirmation(finalized_header_update)
                .await?;
            return Ok(());
        }
        Err(RelayError::Custom("Failed to get sync committee update".into()).into())
    }

    async fn get_fork_version(
        &mut self,
        signature_slot: u64,
    ) -> Result<web3::ethabi::ethereum_types::H32, RelayError> {
        let signature_epoch = signature_slot.div(32);
        let fork_version_data = self.beacon_api_client.get_fork_version("head").await?;
        let fork_version = if signature_epoch
            >= u64::from_str(&fork_version_data.epoch)
                .map_err(|_| RelayError::Custom("Failed to decode fork_version.epoch".into()))?
        {
            fork_version_data.current_version
        } else {
            fork_version_data.previous_version
        };
        Ok(fork_version)
    }

    async fn import_finalized_header_with_confirmation(
        &mut self,
        finalized_header_update: FinalizedHeaderUpdate,
    ) -> RelayResult<()> {
        let gas_price = self.eth_light_client.gas_price().await?;

        let tx = self
            .eth_light_client
            .beacon_light_client()
            .import_finalized_header(
                finalized_header_update,
                &self.eth_light_client.private_key(),
                Options {
                    gas_price: Some(gas_price),
                    ..Default::default()
                },
            )
            .await?;
        tracing::info!(
            target: "relay-e2e",
            "[Header] Sending tx: {:?}",
            &tx
        );
        wait_for_transaction_confirmation(
            tx,
            self.eth_light_client.get_web3().transport(),
            Duration::from_secs(5),
            1,
        )
        .await?;
        self.last_relay_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RelayError::Custom(format!("{}", e)))?
            .as_secs();
        Ok(())
    }
}
