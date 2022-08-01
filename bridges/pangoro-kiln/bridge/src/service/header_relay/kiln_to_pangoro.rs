use std::{ops::Div, str::FromStr};

use crate::{
    bridge::{BridgeConfig, PangoroKilnBus},
    kiln_client::{client::KilnClient, types::FinalityUpdate},
    pangoro_client::client::PangoroClient,
};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use web3::ethabi::ethereum_types::H32;

#[derive(Debug)]
pub struct KilnToPangoroHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for KilnToPangoroHeaderRelayService {}

impl Service for KilnToPangoroHeaderRelayService {
    type Bus = PangoroKilnBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("header-kiln-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start kiln-to-pangoro header relay service, restart after some seconds: {:?}",
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
    let config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;
    let pangoro_client = PangoroClient::new(
        &config.pangoro.endpoint,
        &config.pangoro.contract_address,
        &config.pangoro.execution_layer_contract_address,
        Some(&config.pangoro.private_key),
    )?;
    let kiln_client = KilnClient::new(&config.kiln.endpoint)?;
    let header_relay = HeaderRelay {
        pangoro_client,
        kiln_client,
    };

    loop {
        if let Err(error) = header_relay.header_relay().await {
            tracing::error!(
                target: "pangoro-kiln",
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
    // Latest header slot at Kiln
    pub current_slot: u64,
    // Latest period at Kiln
    pub current_period: u64,
}

pub struct HeaderRelay {
    pub pangoro_client: PangoroClient,
    pub kiln_client: KilnClient,
}

impl HeaderRelay {
    pub async fn get_state(&self) -> color_eyre::Result<HeaderRelayState> {
        let relayed = self.pangoro_client.finalized_header().await?;
        let current_head = self.kiln_client.get_header("head").await?;
        let current_slot = current_head.header.message.slot.parse::<u64>()?;
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
            .sync_committee_roots(state.relayed_period + 1)
            .await?;
        tracing::info!(
            target: "pangoro-kiln",
            "[Header][Kiln => Pangoro] State: {:?}",
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
        let finality_update: FinalityUpdate = self.kiln_client.get_finality_update().await?;
        let update_finality_slot = finality_update.finalized_header.slot.parse::<u64>()?;
        let update_finality_period = update_finality_slot.div(32).div(256);

        // The latest finality header has been relayed
        if update_finality_slot == state.relayed_slot {
            return Ok(());
        }

        let (_slot, sync_aggregate_slot, _attested_header, _sync_aggregate_block) = match self
            .kiln_client
            .find_valid_attested_header(
                state.current_slot,
                finality_update.attested_header.slot.parse::<u64>()?,
            )
            .await?
        {
            None => {
                tracing::info!(
                    target: "pangoro-kiln",
                    "[Header][Kiln => Pangoro] Wait for valid attested header",
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
            .kiln_client
            .get_sync_committee_period_update(update_finality_period - 1, 1)
            .await?;
        if sync_change.is_empty() {
            return Err(BridgerError::Custom("Failed to get sync committee update".into()).into());
        }
        let current_sync_committee = sync_change[0].next_sync_committee.clone();
        let fork_version = self
            .kiln_client
            .get_fork_version(sync_aggregate_slot)
            .await?;

        let _tx = self
            .pangoro_client
            .import_finalized_header(
                &finality_update.attested_header,
                &current_sync_committee,
                &finality_update.finalized_header,
                &finality_update.finality_branch,
                &finality_update.sync_aggregate,
                &fork_version.current_version,
                sync_aggregate_slot,
            )
            .await?;

        Ok(())
    }

    pub async fn relay_next_period(&self, state: HeaderRelayState) -> color_eyre::Result<()> {
        let _target_period = state.relayed_period + 1;
        let sync_change = self
            .kiln_client
            .get_sync_committee_period_update(state.relayed_period, 2)
            .await?;

        if let [last_finality, target_finality] = sync_change.as_slice() {
            let attested_slot: u64 = target_finality.attested_header.slot.parse()?;
            let (_slot, sync_aggregate_slot, _attested_header, _sync_aggregate_block) = match self
                .kiln_client
                .find_valid_attested_header(state.current_slot, attested_slot)
                .await?
            {
                None => {
                    tracing::info!(
                        target: "pangoro-kiln",
                        "[Header][Kiln => Pangoro] Wait for valid attested header",
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

            let _tx = self
                .pangoro_client
                .import_finalized_header(
                    &target_finality.attested_header,
                    &last_finality.next_sync_committee,
                    &target_finality.finalized_header,
                    &target_finality.finality_branch,
                    &target_finality.sync_aggregate,
                    &H32::from_str(&target_finality.fork_version)?,
                    sync_aggregate_slot,
                )
                .await?;
            Ok(())
        } else {
            Err(BridgerError::Custom("Failed to get sync committee update".into()).into())
        }
    }
}
