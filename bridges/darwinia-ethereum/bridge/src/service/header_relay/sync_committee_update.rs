use std::{ops::Div, time::Duration};

use crate::{
    bridge::{BridgeBus, BridgeConfig},
    goerli_client::{client::EthereumClient, types::Proof},
    pangoro_client::client::DarwiniaClient,
    web3_helper::{wait_for_transaction_confirmation, GasPriceOracle},
};
use client_contracts::beacon_light_client_types::SyncCommitteePeriodUpdate;
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use web3::{contract::Options, types::U256};

#[derive(Debug)]
pub struct SyncCommitteeUpdateService {
    _greet: Lifeline,
}

impl BridgeService for SyncCommitteeUpdateService {}

impl Service for SyncCommitteeUpdateService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("sync-committee-update-goerli-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-goerli",
                    "Failed to start goerli-to-pangoro sync committee update relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let config: BridgeConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let pangoro_client = DarwiniaClient::new(
        &config.pangoro_evm.endpoint,
        &config.pangoro_evm.contract_address,
        &config.pangoro_evm.execution_layer_contract_address,
        &config.pangoro_evm.private_key,
        U256::from_dec_str(&config.pangoro_evm.max_gas_price)?,
    )?;
    let goerli_client = EthereumClient::new(&config.goerli.endpoint)?;
    let update_manager = SyncCommitteeUpdate {
        pangoro_client,
        goerli_client,
    };

    loop {
        if let Err(error) = update_manager.sync_committee_update().await {
            tracing::error!(
                target: "pangoro-goerli",
                "[SyncCommittee][Ethereum=>Darwinia] Failed relay sync committee update : {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

pub struct SyncCommitteeUpdate {
    pub pangoro_client: DarwiniaClient,
    pub goerli_client: EthereumClient,
}

impl SyncCommitteeUpdate {
    pub async fn sync_committee_update(&self) -> color_eyre::Result<()> {
        let last_relayed_header = self
            .pangoro_client
            .beacon_light_client
            .finalized_header()
            .await?;
        let period = last_relayed_header.slot.div(32).div(256);

        let _current_sync_committee = self
            .pangoro_client
            .beacon_light_client
            .sync_committee_roots(period)
            .await?;
        let next_sync_committee = self
            .pangoro_client
            .beacon_light_client
            .sync_committee_roots(period + 1)
            .await?;
        if next_sync_committee.is_zero() {
            tracing::info!(
                target: "pangoro-goerli",
                "[SyncCommittee][Ethereum=>Darwinia] Try to relay SyncCommittee at period {:?}",
                period + 1,
            );

            let sync_committee_update = self
                .get_sync_committee_update_parameter(period, last_relayed_header.slot)
                .await?;

            let gas_price = self.pangoro_client.gas_price().await?;
            let tx = self
                .pangoro_client
                .beacon_light_client
                .import_next_sync_committee(
                    sync_committee_update,
                    &self.pangoro_client.private_key,
                    Options {
                        gas: Some(U256::from_dec_str("10000000")?),
                        gas_price: Some(gas_price),
                        ..Default::default()
                    },
                )
                .await?;

            tracing::info!(
                target: "pangoro-goerli",
                "[SyncCommittee][Ethereum=>Darwinia] Sending tx: {:?}",
                &tx
            );
            wait_for_transaction_confirmation(
                tx,
                self.pangoro_client.client.transport(),
                Duration::from_secs(5),
                3,
            )
            .await?;
        } else {
            tracing::info!(
                target: "pangoro-goerli",
                "[SyncCommittee][Ethereum=>Darwinia] Next sync committee is {:?}",
                next_sync_committee
            );
        }
        Ok(())
    }

    async fn get_sync_committee_update_parameter(
        &self,
        period: u64,
        slot: u64,
    ) -> color_eyre::Result<SyncCommitteePeriodUpdate> {
        let sync_committee_update = self
            .goerli_client
            .get_sync_committee_period_update(period, 1)
            .await?;
        if sync_committee_update.is_empty() {
            return Err(BridgerError::Custom("Failed to get sync committee update".into()).into());
        }
        let next_sync_committee = sync_committee_update
            .get(0)
            .expect("Unreachable!")
            .next_sync_committee
            .clone();
        let next_sync_committee_branch = self
            .goerli_client
            .get_next_sync_committee_branch(slot)
            .await?;
        let witnesses = match next_sync_committee_branch {
            Proof::SingleProof {
                gindex: _,
                leaf: _,
                witnesses,
            } => witnesses,
            _ => return Err(BridgerError::Custom("Not implemented!".to_string()).into()),
        };
        Ok(SyncCommitteePeriodUpdate {
            sync_committee: next_sync_committee.to_contract_type()?,
            next_sync_committee_branch: witnesses,
        })
    }
}
