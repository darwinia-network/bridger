use std::time::Duration;

use bridge_e2e_traits::client::{MessageClient, MessageEventsQuery, OnDemandHeader};
use client_contracts::{inbound_types::InboundLaneNonce, outbound_types::OutboundLaneNonce};
use support_etherscan::wait_for_transaction_confirmation_with_timeout;
use tokio::sync::broadcast::Sender;
use web3::{
    contract::Options,
    types::{BlockId, BlockNumber, U256},
};

use crate::error::{RelayError, RelayResult};

#[derive(Debug)]
pub struct MessageRelayRunner<S0, S1, O1, O2>
where
    S0: MessageClient + MessageEventsQuery,
    S1: MessageClient + MessageEventsQuery,
    O1: OnDemandHeader,
    O2: OnDemandHeader,
{
    pub state: ChannelState,
    pub max_message_num_per_relaying: u64,
    pub relay_notifier: Option<Sender<O1>>,
    pub confirm_notifier: Option<Sender<O2>>,
    pub source: S0,
    pub target: S1,
}

#[derive(Default, Debug)]
pub struct ChannelState {
    // Latest status of outbound at source side
    source_outbound: OutboundLaneNonce,
    // Status of outbound at source side from the perspective of the target side
    source_outbound_relayed: OutboundLaneNonce,
    // Latest status of inbound at target side
    target_inbound: InboundLaneNonce,
    // Status of inbound at target side from the perspective of the source side
    target_inbound_relayed: InboundLaneNonce,
    // Latest source block at target light client
    source_block_at_target: Option<BlockNumber>,
    // Latest target block at source light client
    target_block_at_source: Option<BlockNumber>,
}

impl<S0, S1, O1, O2> MessageRelayRunner<S0, S1, O1, O2>
where
    S0: MessageClient + MessageEventsQuery,
    S1: MessageClient + MessageEventsQuery,
    O1: OnDemandHeader,
    O2: OnDemandHeader,
{
    pub async fn update_channel_state(&mut self) -> RelayResult<()> {
        let target_inbound = self.target.inbound().inbound_lane_nonce(None).await?;
        let source_outbound = self.source.outbound().outbound_lane_nonce(None).await?;
        let source_block_at_target = self
            .target
            .latest_light_client_block_number()
            .await?
            .map(BlockNumber::from);
        let target_block_at_source = self
            .source
            .latest_light_client_block_number()
            .await?
            .map(BlockNumber::from);
        let source_outbound_relayed = self
            .source
            .outbound()
            .outbound_lane_nonce(source_block_at_target.map(BlockId::Number))
            .await?;
        let target_inbound_relayed = self
            .target
            .inbound()
            .inbound_lane_nonce(target_block_at_source.map(BlockId::Number))
            .await?;
        self.state = ChannelState {
            source_outbound,
            source_outbound_relayed,
            target_inbound,
            target_inbound_relayed,
            source_block_at_target,
            target_block_at_source,
        };
        Ok(())
    }

    pub async fn message_relay(&mut self) -> RelayResult<()> {
        self.update_channel_state().await?;
        if self.state.target_inbound.last_delivered_nonce
            == self.state.source_outbound.latest_generated_nonce
        {
            tracing::info!(
                target: "relay-e2e",
                "[MessageDelivery][{}=>{}] Last delivered nonce is {:?}, equal to lastest generated. Do nothing.",
                self.source.chain(),
                self.target.chain(),
                self.state.target_inbound.last_delivered_nonce,
            );
            return Ok(());
        }

        let (begin, end) = (
            self.state.source_outbound.latest_received_nonce + 1,
            self.state.source_outbound.latest_generated_nonce,
        );
        self.delivery_notify_on_demand_header(self.state.target_inbound.last_delivered_nonce + 1)
            .await?;

        match self.state.source_block_at_target {
            None => {
                tracing::info!(
                    target: "relay-e2e",
                    "[MessageDelivery] Wait for header relay",
                );
                return Ok(());
            }
            Some(num) => num,
        };

        if self.state.target_inbound.last_delivered_nonce
            >= self.state.source_outbound_relayed.latest_generated_nonce
            || self.state.source_block_at_target.is_none()
        {
            tracing::info!(
                target: "relay-e2e",
                "[MessageDelivery][{}=>{}] Messages: [{:?}, {:?}] need to be relayed, wait for header relay",
                self.source.chain(),
                self.target.chain(),
                begin,
                end
            );
            return Ok(());
        }

        let (begin, end) = (
            self.state.source_outbound_relayed.latest_received_nonce + 1,
            self.state.source_outbound_relayed.latest_generated_nonce,
        );
        tracing::info!(
            target: "relay-e2e",
            "[MessageDelivery][{}=>{}] Try to relay messages: [{:?}, {:?}]",
            self.source.chain(),
            self.target.chain(),
            self.state.target_inbound.last_delivered_nonce + 1,
            end
        );

        let proof = self
            .source
            .prepare_for_delivery(begin, end, self.state.source_block_at_target)
            .await?;
        let encoded_keys: Vec<U256> = proof
            .outbound_lane_data
            .messages
            .iter()
            .map(|x| x.encoded_key)
            .collect();

        let confirm_limit = 20;

        // Calculate devliery_size parameter in inbound.receive_messages_proof
        let mut count = 0;
        let mut delivered = 0;
        for (index, key) in encoded_keys.iter().enumerate() {
            let current = index as u64 + begin;

            // Messages less or equal than last_delivered_nonce have been delivered.
            let is_delivered = current <= self.state.target_inbound.last_delivered_nonce;
            let beyond_confirm_limit =
                current - proof.outbound_lane_data.latest_received_nonce > confirm_limit;

            if beyond_confirm_limit {
                break;
            }

            if is_delivered {
                delivered += 1;
                count += 1;
                continue;
            }

            if self.source.decide(*key).await? {
                count += 1;
            } else {
                break;
            }

            if count - delivered >= self.max_message_num_per_relaying {
                break;
            }
        }

        if count == delivered {
            tracing::info!(
                target: "relay-e2e",
                "[MessageDelivery][{}=>{}] No need to relay",
                self.source.chain(),
                self.target.chain(),
            );
            return Ok(());
        }

        tracing::info!(
            target: "relay-e2e",
            "[MessageDelivery][{}=>{}] Relaying messages: [{:?}, {:?}]",
            self.source.chain(),
            self.target.chain(),
            begin + delivered,
            begin + count - 1,
        );

        let gas_price = self.target.gas_price().await?;
        let tx = self
            .target
            .inbound()
            .receive_messages_proof(
                proof,
                U256::from(count),
                self.target.private_key(),
                Options {
                    gas_price: Some(gas_price),
                    ..Default::default()
                },
            )
            .await?;

        tracing::info!(
            target: "relay-e2e",
            "[MessageDelivery][{}=>{}] Sending tx: {:?}",
            self.source.chain(),
            self.target.chain(),
            tx
        );

        wait_for_transaction_confirmation_with_timeout(
            tx,
            self.target.get_web3().transport(),
            Duration::from_secs(5),
            1,
            150,
        )
        .await?;

        Ok(())
    }

    pub async fn message_confirm(&mut self) -> RelayResult<()> {
        self.update_channel_state().await?;
        if self.state.source_outbound.latest_received_nonce
            == self.state.source_outbound.latest_generated_nonce
        {
            tracing::info!(
                target: "relay-e2e",
                "[MessageConfirmation][{}=>{}] All confirmed({:?}), nothing to do.",
                self.source.chain(),
                self.target.chain(),
                self.state.source_outbound
            );
            return Ok(());
        }

        // assemble unrewarded relayers state
        let (begin, end) = (
            self.state.target_inbound_relayed.relayer_range_front,
            self.state.target_inbound_relayed.relayer_range_back,
        );

        if self.state.source_outbound.latest_received_nonce
            == self.state.target_inbound_relayed.last_delivered_nonce
        {
            if self.state.source_outbound.latest_received_nonce
                < self.state.target_inbound.last_delivered_nonce
            {
                self.confirm_notify_on_demand_header().await?;
            } else {
                tracing::info!(
                    target: "relay-e2e",
                    "[MessageConfirmation][{}=>{}] Nonce {:?} was confirmed, wait for delivery from {:?} to {:?}. ",
                    self.source.chain(),
                    self.target.chain(),
                    self.state.source_outbound.latest_received_nonce,
                    self.state.target_inbound_relayed.last_delivered_nonce + 1,
                    self.state.source_outbound.latest_generated_nonce
                );
            }
            return Ok(());
        }

        if self.state.target_block_at_source.is_none() {
            tracing::info!(
                target: "relay-e2e",
                "[MessageConfirmation][{}=>{}] Nonce [{:?}:{:?}] was delivered, wait for header relay",
                self.source.chain(),
                self.target.chain(),
                self.state.source_outbound.latest_received_nonce + 1,
                self.state.target_inbound_relayed.last_delivered_nonce,
            );
            return Ok(());
        }

        tracing::info!(
            target: "relay-e2e",
            "[MessageConfirmation][{}=>{}] Try to confirm nonces [{:?}:{:?}]",
            self.source.chain(),
            self.target.chain(),
            begin,
            end,
        );
        // read proof
        let proof = self
            .target
            .prepare_for_confirmation(begin, end, self.state.target_block_at_source)
            .await?;

        let gas_price = self.source.gas_price().await?;
        // send proof
        let hash = self
            .source
            .outbound()
            .receive_messages_delivery_proof(
                proof,
                self.source.private_key(),
                Options {
                    gas_price: Some(gas_price),
                    ..Default::default()
                },
            )
            .await?;

        tracing::info!(
            target: "relay-e2e",
            "[MessageConfirmation][{}=>{}] Messages confirmation tx: {:?}",
            self.source.chain(),
            self.target.chain(),
            hash
        );
        wait_for_transaction_confirmation_with_timeout(
            hash,
            self.source.get_web3().transport(),
            Duration::from_secs(5),
            1,
            150,
        )
        .await?;

        Ok(())
    }

    async fn confirm_notify_on_demand_header(&mut self) -> RelayResult<()> {
        if let Some(s) = self.confirm_notifier.as_ref() {
            let mut block_number = self
                .state
                .target_block_at_source
                .unwrap_or(BlockNumber::Earliest);

            // Latest confirmed message is at block_number, so we need to search events after block_number
            if let BlockNumber::Number(n) = block_number {
                block_number = BlockNumber::Number(n + 1);
            }
            let events = self.target.query_message_dispatched(block_number).await?;
            if events.is_empty() {
                return Ok(());
            }
            let event = events.first().expect("Unreachable!");
            if s.len() > 0 {
                tracing::trace!(
                    target: "relay-e2e",
                    "[MessageConfirmation][{}=>{}] Tunnel length: {:?}, required header: {:?}",
                    self.source.chain(),
                    self.target.chain(),
                    s.len(),
                    event.block_number,
                );
                return Ok(());
            }

            s.send(event.block_number.into())
                .map_err(|_| RelayError::Custom("Tunnel error".into()))?;
            tracing::info!(
                target: "relay-e2e",
                "[MessageConfirmation][{}=>{}] Signal: required header({:?})",
                self.source.chain(),
                self.target.chain(),
                event.block_number,
            );
        }
        Ok(())
    }

    async fn delivery_notify_on_demand_header(&mut self, nonce: u64) -> RelayResult<()> {
        if let Some(s) = self.relay_notifier.as_ref() {
            let event = self.source.query_message_accepted(nonce).await?;
            if event.is_none() {
                return Ok(());
            }

            let event = event.unwrap();
            if s.len() > 0 {
                tracing::trace!(
                    target: "relay-e2e",
                    "[MessageDelivery][{}=>{}] Tunnel length: {:?}, required header: {:?}",
                    self.source.chain(),
                    self.target.chain(),
                    s.len(),
                    event.block_number,
                );
                return Ok(());
            }
            s.send(event.block_number.into())
                .map_err(|_| RelayError::Custom("Tunnel error".into()))?;
            tracing::info!(
                target: "relay-e2e",
                "[MessageDelivery][{}=>{}] Signal: required header({:?})",
                self.source.chain(),
                self.target.chain(),
                event.block_number,
            );
        }
        Ok(())
    }
}
