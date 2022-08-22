use std::str::FromStr;

use bridge_e2e_traits::strategy::{EnforcementRelayStrategy, RelayStrategy};
use client_contracts::PosaLightClient;
use web3::types::{Address, BlockId, BlockNumber, U256};

use crate::message_contract::darwinia_message_client::{
    build_darwinia_message_client, DarwiniaMessageClient,
};
use crate::message_contract::fee_market::FeeMarketRelayStrategy;
use crate::message_contract::message_client::build_message_client_with_simple_fee_market;
use crate::message_contract::simple_fee_market::SimpleFeeMarketRelayStrategy;
use crate::{
    kiln_client::client::KilnClient, message_contract::message_client::MessageClient,
    pangoro_client::client::PangoroClient,
};

use crate::bridge::{BridgeBus, BridgeConfig};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

#[derive(Debug)]
pub struct KilnPangoroMessageRelay {
    _greet_delivery: Lifeline,
    _greet_confirmation: Lifeline,
}

impl BridgeService for KilnPangoroMessageRelay {}

impl Service for KilnPangoroMessageRelay {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task("message-relay-kiln-to-pangoro", async move {
            while let Err(error) = start_delivery().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start kiln-to-pangoro message relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
            Ok(())
        });
        let _greet_confirmation = Self::try_task(
            "message-confirmation-pangoro-to-kiln",
            async move {
                while let Err(error) = start_delivery().await {
                    tracing::error!(
                        target: "pangoro-kiln",
                        "Failed to start kiln-to-pangoro message confirmation service, restart after some seconds: {:?}",
                        error
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_delivery,
            _greet_confirmation,
        })
    }
}

async fn message_relay_client_builder(
) -> color_eyre::Result<MessageRelay<SimpleFeeMarketRelayStrategy, FeeMarketRelayStrategy>> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;
    let beacon_light_client = PangoroClient::new(
        &config.pangoro_evm.endpoint,
        &config.pangoro_evm.contract_address,
        &config.pangoro_evm.execution_layer_contract_address,
        Some(&config.pangoro_evm.private_key),
    )?;
    let beacon_rpc_client = KilnClient::new(&config.kiln.endpoint)?;
    let source = build_message_client_with_simple_fee_market(
        &config.kiln.execution_layer_endpoint,
        Address::from_str(&config.kiln.inbound_address)?,
        Address::from_str(&config.kiln.outbound_address)?,
        Address::from_str(&config.kiln.fee_market_address)?,
        Address::from_str(&config.kiln.account)?,
        Some(&config.kiln.private_key),
    )
    .unwrap();
    let target = build_darwinia_message_client(
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
        target.client.clone(),
        Address::from_str(&config.kiln.posa_light_client_address)?,
    )?;
    Ok(MessageRelay {
        source,
        target,
        posa_light_client,
        beacon_rpc_client,
        beacon_light_client,
    })
}

async fn start_delivery() -> color_eyre::Result<()> {
    let message_relay_service = message_relay_client_builder().await?;
    loop {
        if let Err(error) = message_relay_service.message_relay().await {
            tracing::error!(
                target: "pangoro-kiln",
                "[MessageDelivery][kiln=>Pangoro] Failed to relay message: {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

async fn start_confirmation() -> color_eyre::Result<()> {
    let message_relay_service = message_relay_client_builder().await?;
    loop {
        if let Err(error) = message_relay_service.message_confirm().await {
            tracing::error!(
                target: "pangoro-kiln",
                "[MessageConfirmation][kiln=>Pangoro] Failed to confirm message: {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

pub struct MessageRelay<S0: RelayStrategy, S1: RelayStrategy> {
    pub source: MessageClient<S0>,
    pub target: DarwiniaMessageClient<S1>,
    pub posa_light_client: PosaLightClient,
    pub beacon_rpc_client: KilnClient,
    pub beacon_light_client: PangoroClient,
}

impl<S0: RelayStrategy, S1: RelayStrategy> MessageRelay<S0, S1> {
    async fn message_relay(&self) -> color_eyre::Result<()> {
        let received_nonce = self.target.inbound.inbound_lane_nonce().await?;
        let latest_nonce = self.source.outbound.outbound_lane_nonce().await?;

        if received_nonce.last_delivered_nonce == latest_nonce.latest_generated_nonce {
            tracing::info!(
                target: "pangoro-kiln",
                "[MessageDelivery][Kiln => Pangoro] Last delivered nonce is {:?}, equal to lastest generated. Do nothing.",
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
            "[MessageDelivery][Kiln => Pangoro] Nonce range: [{:?}, {:?}]",
            begin,
            end,
        );
        let finalized_block_number = self.finalized_target_header_number_at_source().await?;
        let end_event = self.source.query_message_accepted(end).await?;

        if let Some(event) = end_event {
            tracing::info!(
                target: "pangoro-kiln",
                "[MessageDelivery][Kiln => Pangoro] Message at block: {:?}, latest relayed header: {:?}",
                event.block_number,
                finalized_block_number,
            );

            // Need to wait for header relay.
            if event.block_number > finalized_block_number {
                tracing::info!(
                    target: "pangoro-kiln",
                    "[MessageDelivery][Kiln => Pangoro] Message at block: {:?}, latest relayed header: {:?}, wait for header relay.",
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
                "[MessageDelivery][Kiln => Pangoro] The relay strategy decide to not relay thess messaages {:?}",
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
            "[MessageDelivery][Kiln => Pangoro] Sending tx: {:?}",
            tx
        );

        Ok(())
    }

    async fn finalized_target_header_number_at_source(&self) -> color_eyre::Result<u64> {
        let finalized = self.beacon_light_client.finalized_header().await?;
        let block = self
            .beacon_rpc_client
            .get_beacon_block(finalized.slot)
            .await?;
        Ok(block.body.execution_payload.block_number.parse()?)
    }
}

impl<S0: RelayStrategy, S1: RelayStrategy> MessageRelay<S0, S1> {
    pub async fn message_confirm(&self) -> color_eyre::Result<()> {
        let source_outbound_lane_data = self.source.outbound.outbound_lane_nonce().await?;
        if source_outbound_lane_data.latest_received_nonce
            == source_outbound_lane_data.latest_generated_nonce
        {
            tracing::info!(
                target: "pangoro-kiln",
                "[MessageConfirmation][kiln=>Pangoro] All confirmed({:?}), nothing to do.",
                source_outbound_lane_data
            );
            return Ok(());
        }

        // query last relayed header
        let last_relayed_target_block_in_source = self.best_target_block_at_source().await?;

        // assemble unrewarded relayers state
        let target_inbound_state = self.target.inbound.inbound_lane_nonce().await?;
        let (begin, end) = (
            target_inbound_state.relayer_range_front,
            target_inbound_state.relayer_range_back,
        );
        if source_outbound_lane_data.latest_received_nonce
            == target_inbound_state.last_delivered_nonce
        {
            tracing::info!(
                target: "pangoro-kiln",
                "[MessageConfirmation][kiln=>Pangoro] Nonce {:?} was confirmed, wait for delivery from {:?} to {:?}. ",
                source_outbound_lane_data.latest_received_nonce,
                target_inbound_state.last_delivered_nonce + 1,
                source_outbound_lane_data.latest_generated_nonce
            );
            return Ok(());
        }

        tracing::info!(
            target: "pangoro-kiln",
            "[MessageConfirmation][kiln=>Pangoro] Try to confirm nonces [{:?}:{:?}]",
            begin,
            end,
        );
        // read proof
        let proof = self
            .target
            .prepare_for_messages_confirmation(Some(BlockId::Number(BlockNumber::from(
                last_relayed_target_block_in_source,
            ))))
            .await?;

        // send proof
        let hash = self
            .source
            .outbound
            .receive_messages_delivery_proof(proof, &self.source.private_key()?)
            .await?;

        tracing::info!(
            target: "relay-s2s",
            "[MessageConfirmation][kiln=>Pangoro] Messages confirmation tx: {:?}",
            hash
        );
        Ok(())
    }

    async fn best_target_block_at_source(&self) -> color_eyre::Result<u64> {
        Ok(self.posa_light_client.block_number().await?.as_u64())
    }
}
