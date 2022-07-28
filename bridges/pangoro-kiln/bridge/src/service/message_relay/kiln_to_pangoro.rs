use web3::types::BlockNumber;

use crate::{
    kiln_client::{client::KilnClient, message_client::MessageClient},
    pangoro_client::client::PangoroClient,
};

use crate::bridge::{BridgeConfig, PangoroKilnBus};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

#[derive(Debug)]
pub struct KilnPangoroMessageRelay {
    _greet: Lifeline,
}

impl BridgeService for KilnPangoroMessageRelay {}

impl Service for KilnPangoroMessageRelay {
    type Bus = PangoroKilnBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("message-relay-kiln-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start kiln-to-pangoro message relay service, restart after some seconds: {:?}",
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
    let beacon_light_client = PangoroClient::new(
        &config.pangoro.endpoint,
        &config.pangoro.contract_address,
        &config.pangoro.execution_layer_contract_address,
        Some(&config.pangoro.private_key),
    )?;
    let beacon_rpc_client = KilnClient::new(&config.kiln.endpoint)?;
    let source = MessageClient::new(
        &config.kiln.execution_layer_endpoint,
        &config.kiln.inbound_address,
        &config.kiln.outbound_address,
        Some(&config.pangoro.private_key),
    )
    .unwrap();
    let target = MessageClient::new(
        &config.pangoro.endpoint,
        &config.pangoro.inbound_address,
        &config.pangoro.outbound_address,
        Some(&config.pangoro.private_key),
    )
    .unwrap();
    let message_relay_service = MessageRelay {
        source,
        target,
        beacon_rpc_client,
        beacon_light_client,
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

pub struct MessageRelay {
    pub source: MessageClient,
    pub target: MessageClient,
    pub beacon_rpc_client: KilnClient,
    pub beacon_light_client: PangoroClient,
}

impl MessageRelay {
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
