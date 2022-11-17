use std::{ops::Div, str::FromStr, time::Duration};

use bridge_e2e_traits::client::EthTruthLayerLightClient;
use client_beacon::client::BeaconApiClient;
use client_contracts::beacon_light_client_types::{
    FinalizedHeaderUpdate, SyncCommitteePeriodUpdate,
};
use web3::{
    contract::Options,
    ethabi::ethereum_types::H32,
    types::{Bytes, H256, U256},
};

use crate::error::{RelayError, RelayResult};

pub struct SyncCommitteeRelayRunner<C: EthTruthLayerLightClient> {
    eth_light_client: C,
    beacon_api_client: BeaconApiClient,
}

impl<C: EthTruthLayerLightClient> SyncCommitteeRelayRunner<C> {
    pub async fn start(&mut self) -> RelayResult<()> {
        loop {
            self.run().await?;
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    }

    pub async fn run(&mut self) -> RelayResult<()> {
        let last_relayed_header = self
            .eth_light_client
            .beacon_light_client()
            .finalized_header()
            .await?;
        let period = last_relayed_header.slot.div(32).div(256);

        let _current_sync_committee = self
            .eth_light_client
            .beacon_light_client()
            .sync_committee_roots(period)
            .await?;
        let next_sync_committee = self
            .eth_light_client
            .beacon_light_client()
            .sync_committee_roots(period + 1)
            .await?;
        if next_sync_committee.is_zero() {
            tracing::info!(
                target: "relay-e2e",
                "[SyncCommittee] Try to relay SyncCommittee at period {:?}",
                period + 1,
            );

            let (finalized_header_update, sync_committee_update) =
                self.get_sync_committee_update_parameter(period).await?;

            let gas_price = self.eth_light_client.gas_price().await?;
            let tx = self
                .eth_light_client
                .beacon_light_client()
                .import_next_sync_committee(
                    finalized_header_update,
                    sync_committee_update,
                    self.eth_light_client.private_key(),
                    Options {
                        gas: Some(
                            U256::from_dec_str("10000000")
                                .map_err(|e| RelayError::Custom(format!("{}", e)))?,
                        ),
                        gas_price: Some(gas_price),
                        ..Default::default()
                    },
                )
                .await?;

            tracing::info!(
                target: "relay-e2e",
                "[SyncCommittee] Sending tx: {:?}",
                &tx
            );
            support_etherscan::wait_for_transaction_confirmation(
                tx,
                self.eth_light_client.get_web3().transport(),
                Duration::from_secs(5),
                3,
            )
            .await?;
        } else {
            tracing::info!(
                target: "relay-e2e",
                "[SyncCommittee] Next sync committee is {:?}",
                next_sync_committee
            );
        }
        Ok(())
    }

    async fn get_sync_committee_update_parameter(
        &self,
        period: u64,
    ) -> RelayResult<(FinalizedHeaderUpdate, SyncCommitteePeriodUpdate)> {
        let sync_committee_update = self
            .beacon_api_client
            .get_sync_committee_period_update(period, 1)
            .await?;
        if sync_committee_update.is_empty() {
            return Err(RelayError::Custom("Failed to get sync committee update".into()).into());
        }
        let sync_committee_update = sync_committee_update.get(0).expect("Unreachable!");
        let header_root = self
            .beacon_api_client
            .get_beacon_block_root(sync_committee_update.finalized_header.slot)
            .await?;
        let snapshot = self.beacon_api_client.get_bootstrap(&header_root).await?;
        let next_sync_committee_branch = sync_committee_update
            .next_sync_committee_branch
            .clone()
            .iter()
            .map(|x| H256::from_str(x))
            .collect::<Result<Vec<H256>, _>>()
            .map_err(|_| {
                RelayError::Custom("Failed to decode next_sync_committee_branch".into())
            })?;
        let current_head = self.beacon_api_client.get_header("head").await?;
        let (signature_slot, _) = self
            .beacon_api_client
            .find_valid_header_since(
                current_head.header.message.slot,
                sync_committee_update.attested_header.slot + 1,
            )
            .await?;
        let finalized_header_update = FinalizedHeaderUpdate {
            attested_header: sync_committee_update.attested_header.to_contract_type()?,
            signature_sync_committee: snapshot.current_sync_committee.to_contract_type()?,
            finalized_header: sync_committee_update.finalized_header.to_contract_type()?,
            finality_branch: sync_committee_update
                .finality_branch
                .iter()
                .map(|x| H256::from_str(x))
                .collect::<Result<Vec<H256>, _>>()
                .map_err(|_| RelayError::Custom("Failed to decode finality_branch".into()))?,
            sync_aggregate: sync_committee_update.sync_aggregate.to_contract_type()?,
            fork_version: Bytes(
                H32::from_str(&sync_committee_update.fork_version)
                    .map_err(|_| RelayError::Custom("Failed to decode fork_version".into()))?
                    .as_ref()
                    .to_vec(),
            ),
            signature_slot,
        };
        Ok((
            finalized_header_update,
            SyncCommitteePeriodUpdate {
                sync_committee: sync_committee_update
                    .next_sync_committee
                    .to_contract_type()?,
                next_sync_committee_branch,
            },
        ))
    }
}
