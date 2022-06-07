use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use client_pangoro::subxt_runtime::api::bridge_pangolin_messages::storage::{
    OutboundLanes, OutboundMessages,
};
use client_pangoro::types::runtime_types;
use client_pangoro::types::runtime_types::bp_messages::{MessageKey, OutboundLaneData};
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
use std::ops::RangeInclusive;
use subquery_s2s::types::{BridgeName, NeedRelayBlock, RelayBlockOrigin};
use subquery_s2s::{Subquery, SubqueryComponent};
use subxt::storage::StorageKeyPrefix;
use subxt::{StorageEntry, StorageEntryKey, StorageHasher, StorageMapKey};

use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, RelayConfig};
use crate::types::HexLaneId;

#[derive(Debug)]
pub struct PangoroToPangolinMessageRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangoroToPangolinMessageRelayService {}

impl Service for PangoroToPangolinMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("pangoro-to-pangolin-message-relay-service", async move {
            while let Err(e) = start_runner().await {
                tracing::error!(
                    target: "pangolin-pangoro",
                    "[message-pangoro-to-pangolin] Failed to start pangolin-to-pangoro message relay, \
                    wait some seconds try again: {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start_runner() -> color_eyre::Result<()> {
    let runner = RelayRunner::new().await?;
    runner.start().await
}

struct MessageRelay {
    relay_config: RelayConfig,
    client_pangolin: PangolinClient,
    client_pangoro: PangoroClient,
    subquery_pangoro: Subquery,
}

impl MessageRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

        let index_config = bridge_config.index;
        let config_pangolin = bridge_config.pangolin;
        let config_pangoro = bridge_config.pangoro;

        let client_pangolin =
            PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?)
                .await?;
        let client_pangoro =
            PangoroClientComponent::component(config_pangoro.to_pangoro_client_config()?).await?;
        let subquery_pangoro =
            SubqueryComponent::component(index_config.pangoro, BridgeName::PangolinPangoro);
        Ok(Self {
            relay_config: bridge_config.relay,
            client_pangolin,
            client_pangoro,
            subquery_pangoro,
        })
    }
}

struct RelayRunner {
    message_relay: MessageRelay,
}

impl RelayRunner {
    async fn new() -> color_eyre::Result<Self> {
        let message_relay = MessageRelay::new().await?;
        Ok(Self { message_relay })
    }
}

// defined
impl RelayRunner {
    fn lane(&self) -> color_eyre::Result<HexLaneId> {
        Ok(self
            .message_relay
            .relay_config
            .lanes
            .clone()
            .get(0)
            .cloned()
            .ok_or_else(|| BridgerError::Custom("Missing lane id".to_string()))?)
    }
    async fn outbound_lane_data(&self) -> color_eyre::Result<OutboundLaneData> {
        let lane = self.lane()?;
        let outbound_lane_data = self
            .message_relay
            .client_pangoro
            .runtime()
            .storage()
            .bridge_pangolin_messages()
            .outbound_lanes(lane.0, None)
            .await?;
        tracing::debug!(
            target: "pangolin-pangoro",
            "[message-pangoro-to-pangolin] outbound_lane_data: {:?}",
            outbound_lane_data,
        );
        Ok(outbound_lane_data)
    }
    async fn assemble_nonces(&self, limit: u64) -> color_eyre::Result<RangeInclusive<u64>> {
        let outbound_lane_data = self.outbound_lane_data().await?;
        let (latest_confirmed_nonce, latest_generated_nonce) = (
            outbound_lane_data.latest_received_nonce,
            outbound_lane_data.latest_generated_nonce,
        );

        // assemble nonce range
        let start: u64 = latest_confirmed_nonce + 1;
        let inclusive_limit = limit - 1;
        let end: u64 = if latest_generated_nonce - start > inclusive_limit {
            start + inclusive_limit
        } else {
            latest_generated_nonce
        };
        let nonces = start..=end;
        Ok(nonces)
    }
}

impl RelayRunner {
    async fn start(&self) -> color_eyre::Result<()> {
        tracing::info!(
            target: "pangolin-pangoro",
            "[message-pangoro-to-pangolin] SERVICE RESTARTING..."
        );
        loop {
            match self.run(10).await {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!(
                        target: "pangolin-pangoro",
                        "[message-pangoro-to-pangolin] Failed to relay message: {:?}",
                        err
                    );
                    message_relay = MessageRelay::new().await?;
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    async fn run(&self, limit: u64) -> color_eyre::Result<()> {
        let lane = self.lane()?;

        // alias
        let client_pangoro = &self.message_relay.client_pangoro;
        let client_pangolin = &self.message_relay.client_pangolin;
        let subquery_pangoro = &self.message_relay.subquery_pangoro;

        let nonces = self.assemble_nonces(limit).await?;

        // query last nonce block information
        let last_relay = match subquery_pangoro
            .query_need_relay(RelayBlockOrigin::BridgePangolin, lane.0, *nonces.end())
            .await?
        {
            Some(v) => v,
            None => {
                tracing::warn!(
                    target: "pangolin-pangoro",
                    "[message-pangoro-to-pangolin] The last nonce({}) isn't storage by indexer",
                    nonces.end(),
                );
                return Ok(());
            }
        };

        // query last relayed header
        let last_relayed_pangoro_hash_in_pangolin = client_pangolin
            .runtime()
            .storage()
            .bridge_pangoro_grandpa()
            .best_finalized(None)
            .await?;
        let last_relayed_pangoro_block_in_pangolin = client_pangoro
            .subxt()
            .rpc()
            .block(Some(last_relayed_pangoro_hash_in_pangolin))
            .await?
            .ok_or_else(|| {
                BridgerError::Custom(format!(
                    "Failed to query block by [{}] in pangoro",
                    last_relayed_pangoro_hash_in_pangolin
                ))
            })?;

        // compare last nonce block with last relayed header
        let relayed_block_number = last_relayed_pangoro_block_in_pangolin.block.header.number;
        if relayed_block_number < last_relay.block_number {
            tracing::warn!(
                target: "pangolin-pangoro",
                "[message-pangoro-to-pangolin] The last nonce({}) at block {} is less then last relayed header {}, \
                please wait header relay.",
                end,
                last_relay.block_number,
                relayed_block_number,
            );
            return Ok(());
        }

        let mut storage_keys = Vec::with_capacity((nonces.end() - nonces.start()) as usize + 1);
        let mut message_nonce = *nonces.start();
        while message_nonce <= *nonces.end() {
            let prefix = StorageKeyPrefix::new::<OutboundMessages>();
            let message_key = OutboundMessages(MessageKey {
                lane_id: lane.0,
                nonce: message_nonce,
            })
            .key()
            .final_key(prefix);
            storage_keys.push(message_key);
            message_nonce += 1;
        }
        storage_keys.push(
            OutboundLanes(lane.0)
                .key()
                .final_key(StorageKeyPrefix::new::<OutboundLanes>()),
        );

        let read_proof = client_pangoro
            .subxt()
            .rpc()
            .read_proof(storage_keys, Some(last_relay.block_hash_bytes()?))
            .await?;
        let proof: Vec<Vec<u8>> = read_proof.proof.into_iter().map(|item| item.0).collect();
        let proof = FromBridgedChainMessagesProof {};

        let next_nonce = outbound_lane_data.oldest_unpruned_nonce;
        let outbound_messages = client_pangoro
            .runtime()
            .storage()
            .bridge_pangolin_messages()
            .outbound_messages(
                MessageKey {
                    lane_id: lane.0,
                    nonce: next_nonce,
                },
                None,
            )
            .await?
            .ok_or_else(|| {
                BridgerError::Custom(format!(
                    "The next nonce is {} but can not read outbound message from this nonce",
                    next_nonce
                ))
            })?;

        client_pangolin
            .runtime()
            .tx()
            .bridge_pangoro_messages()
            .receive_messages_proof();

        tracing::debug!(
            target: "pangolin-pangoro",
            "The nonce is {} and the message is {:?}",
            next_nonce,
            outbound_messages,
        );
        Ok(())
    }
}
