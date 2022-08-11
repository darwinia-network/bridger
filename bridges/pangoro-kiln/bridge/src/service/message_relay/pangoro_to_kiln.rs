use std::str::FromStr;

use bridge_e2e_traits::strategy::{EnforcementRelayStrategy, RelayStrategy};
use client_contracts::PosaLightClient;

use support_common::error::BridgerError;
use web3::types::{Address, BlockNumber, U256};

use crate::message_contract::darwinia_message_client::{
    build_darwinia_message_client, DarwiniaMessageClient,
};
use crate::message_contract::message_client::build_message_client_with_simple_fee_market;
use crate::message_contract::message_client::MessageClient;
use crate::message_contract::utils::query_message_accepted;

use crate::bridge::{BridgeConfig, PangoroKilnBus};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

#[derive(Debug)]
pub struct PangoroKilnMessageRelay {
    _greet: Lifeline,
}

impl BridgeService for PangoroKilnMessageRelay {}

impl Service for PangoroKilnMessageRelay {
    type Bus = PangoroKilnBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("message-relay-pangoro-to-kiln", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start pangoro-to-kiln message relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;
    let target = build_message_client_with_simple_fee_market(
        &config.kiln.execution_layer_endpoint,
        Address::from_str(&config.kiln.inbound_address)?,
        Address::from_str(&config.kiln.outbound_address)?,
        Address::from_str(&config.kiln.fee_market_address)?,
        Address::from_str(&config.kiln.account)?,
        Some(&config.kiln.private_key.ok_or_else(|| {
            BridgerError::Custom("Private key of kiln not found in the config".into())
        })?),
    )
    .unwrap();
    let source = build_darwinia_message_client(
        &config.pangoro_evm.endpoint,
        Address::from_str(&config.pangoro_evm.inbound_address)?,
        Address::from_str(&config.pangoro_evm.outbound_address)?,
        Address::from_str(&config.pangoro_evm.chain_message_committer_address)?,
        Address::from_str(&config.pangoro_evm.lane_message_committer_address)?,
        Address::from_str(&config.pangoro_evm.fee_market_address)?,
        Address::from_str(&config.pangoro_evm.account)?,
        Some(&config.pangoro_evm.private_key),
    )
    .unwrap();
    let posa_light_client = PosaLightClient::new(
        &target.client,
        Address::from_str(&config.kiln.posa_light_client_address)?,
    )?;
    let message_relay_service = MessageRelay {
        source,
        target,
        posa_light_client,
    };

    loop {
        if let Err(error) = message_relay_service.message_relay().await {
            tracing::error!(
                target: "pangoro-kiln",
                "Failed to relay message: {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

pub struct MessageRelay<S0: RelayStrategy, S1: RelayStrategy> {
    pub source: DarwiniaMessageClient<S0>,
    pub target: MessageClient<S1>,
    pub posa_light_client: PosaLightClient,
}

impl<S0: RelayStrategy, S1: RelayStrategy> MessageRelay<S0, S1> {
    async fn message_relay(&self) -> color_eyre::Result<()> {
        let received_nonce = self.target.inbound.inbound_lane_nonce().await?;
        let latest_nonce = self.source.outbound.outbound_lane_nonce().await?;

        if received_nonce.last_delivered_nonce == latest_nonce.latest_generated_nonce {
            tracing::info!(
                target: "pangoro-kiln",
                "[MessageDelivery][Pangoro=>Kiln] Last delivered nonce is {:?}, equal to lastest generated. Do nothing.",
                received_nonce.last_delivered_nonce,
            );
            return Ok(());
        }

        let (begin, end) = (
            latest_nonce.latest_received_nonce + 1,
            latest_nonce.latest_generated_nonce,
        );
        tracing::info!(
            target: "pangoro-kiln",
            "[MessageDelivery][Pangoro=>Kiln] Nonce range: [{:?}, {:?}]",
            begin,
            end,
        );
        let finalized_block_number = self.finalized_target_header_number_at_source().await?;
        let end_event =
            query_message_accepted(&self.source.client, &self.source.outbound, end).await?;

        if let Some(event) = end_event {
            tracing::info!(
                target: "pangoro-kiln",
                "[MessageDelivery][Pangoro=>Kiln] Message at block: {:?}, latest relayed header: {:?}",
                event.block_number,
                finalized_block_number,
            );

            // Need to wait for header relay.
            if event.block_number > finalized_block_number {
                tracing::info!(
                    target: "pangoro-kiln",
                    "[MessageDelivery][Pangoro=>Kiln] Message at block: {:?}, latest relayed header: {:?}, wait for header relay.",
                    event.block_number,
                    finalized_block_number,
                );
                return Ok(());
            }
        }

        let proof = self
            .source
            .prepare_for_messages_delivery(
                begin,
                end,
                Some(BlockNumber::from(finalized_block_number)),
            )
            .await?;
        let mut strategy = EnforcementRelayStrategy::new(self.source.strategy.clone());
        let encoded_keys: Vec<U256> = proof
            .outbound_lane_data
            .messages
            .iter()
            .map(|x| x.encoded_key)
            .collect();

        if !strategy.decide(&encoded_keys).await? {
            tracing::info!(
                target: "pangoro-kiln",
                "[MessageDelivery][Pangoro=>Kiln] The relay strategy decide to not relay thess messaages {:?}",
                (begin, end)
            );

            return Ok(());
        }

        let tx = self
            .target
            .inbound
            .receive_messages_proof(proof, &self.target.private_key()?)
            .await?;

        tracing::info!(
            target: "pangoro-kiln",
            "[MessageDelivery][Pangoro=>Kiln] Sending tx: {:?}",
            tx
        );

        Ok(())
    }

    async fn finalized_target_header_number_at_source(&self) -> color_eyre::Result<u64> {
        Ok(self.posa_light_client.block_number().await?.as_u64())
    }
}
