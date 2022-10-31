use std::{ops::Div, time::Duration};

use bridge_e2e_traits::client::EthTruthLayerLightClient;
use client_beacon::{client::BeaconApiClient, types::Proof};
use client_contracts::beacon_light_client_types::SyncCommitteePeriodUpdate;
use web3::{contract::Options, types::U256};

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

            let sync_committee_update = self
                .get_sync_committee_update_parameter(period, last_relayed_header.slot)
                .await?;

            let gas_price = self.eth_light_client.gas_price().await?;
            let tx = self
                .eth_light_client
                .beacon_light_client()
                .import_next_sync_committee(
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
        slot: u64,
    ) -> RelayResult<SyncCommitteePeriodUpdate> {
        let sync_committee_update = self
            .beacon_api_client
            .get_sync_committee_period_update(period, 1)
            .await?;
        if sync_committee_update.is_empty() {
            return Err(RelayError::Custom("Failed to get sync committee update".into()).into());
        }
        let next_sync_committee = sync_committee_update
            .get(0)
            .expect("Unreachable!")
            .next_sync_committee
            .clone();
        let next_sync_committee_branch = self
            .beacon_api_client
            .get_next_sync_committee_branch(slot)
            .await?;
        let witnesses = match next_sync_committee_branch {
            Proof::SingleProof {
                gindex: _,
                leaf: _,
                witnesses,
            } => witnesses,
            _ => return Err(RelayError::Custom("Not implemented!".to_string()).into()),
        };
        Ok(SyncCommitteePeriodUpdate {
            sync_committee: next_sync_committee.to_contract_type()?,
            next_sync_committee_branch: witnesses,
        })
    }
}
