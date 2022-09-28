use std::str::FromStr;
use std::time::Duration;

use bridge_e2e_traits::strategy::RelayStrategy;
use client_contracts::PosaLightClient;
use web3::contract::Options;
use web3::types::{Address, BlockId, BlockNumber, H256, U256};

use crate::message_contract::darwinia_message_client::{
    build_darwinia_message_client, DarwiniaMessageClient,
};
use crate::message_contract::fee_market::FeeMarketRelayStrategy;
use crate::message_contract::message_client::build_message_client_with_simple_fee_market;
use crate::message_contract::simple_fee_market::SimpleFeeMarketRelayStrategy;
use crate::web3_helper::{wait_for_transaction_confirmation, GasPriceOracle};
use crate::{
    goerli_client::client::EthereumClient, message_contract::message_client::MessageClient,
    darwinia_client::client::DarwiniaClient,
};

use crate::bridge::{BridgeBus, BridgeConfig};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

#[derive(Debug)]
pub struct EthereumDarwiniaMessageRelay {
    _greet_delivery: Lifeline,
    _greet_confirmation: Lifeline,
}

impl BridgeService for EthereumDarwiniaMessageRelay {}

impl Service for EthereumDarwiniaMessageRelay {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task("message-relay-goerli-to-darwinia", async move {
            while let Err(error) = start_delivery().await {
                tracing::error!(
                    target: "darwinia-goerli",
                    "Failed to start goerli-to-darwinia message relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
            Ok(())
        });
        let _greet_confirmation = Self::try_task(
            "message-confirmation-darwinia-to-goerli",
            async move {
                while let Err(error) = start_confirmation().await {
                    tracing::error!(
                        target: "darwinia-goerli",
                        "Failed to start goerli-to-darwinia message confirmation service, restart after some seconds: {:?}",
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
    let config: BridgeConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let beacon_light_client = DarwiniaClient::new(
        &config.darwinia_evm.endpoint,
        &config.darwinia_evm.contract_address,
        &config.darwinia_evm.execution_layer_contract_address,
        &config.darwinia_evm.private_key,
        U256::from_dec_str(&config.darwinia_evm.max_gas_price)?,
    )?;
    let beacon_rpc_client = EthereumClient::new(&config.goerli.endpoint)?;
    let source = build_message_client_with_simple_fee_market(
        &config.goerli.execution_layer_endpoint,
        Address::from_str(&config.goerli.inbound_address)?,
        Address::from_str(&config.goerli.outbound_address)?,
        Address::from_str(&config.goerli.fee_market_address)?,
        Address::from_str(&config.goerli.account)?,
        Some(&config.goerli.private_key),
        U256::from_dec_str(&config.goerli.max_gas_price)?,
        &config.goerli.etherscan_api_key,
    )
    .unwrap();
    let target = build_darwinia_message_client(
        &config.darwinia_evm.endpoint,
        Address::from_str(&config.darwinia_evm.inbound_address)?,
        Address::from_str(&config.darwinia_evm.outbound_address)?,
        Address::from_str(&config.darwinia_evm.chain_message_committer_address)?,
        Address::from_str(&config.darwinia_evm.lane_message_committer_address)?,
        Address::from_str(&config.darwinia_evm.fee_market_address)?,
        Address::from_str(&config.darwinia_evm.account)?,
        Some(&config.darwinia_evm.private_key),
        config.index.to_darwinia_thegraph()?,
    )
    .unwrap();
    let posa_light_client = PosaLightClient::new(
        source.client.clone(),
        Address::from_str(&config.goerli.posa_light_client_address)?,
    )?;
    Ok(MessageRelay {
        source,
        target,
        posa_light_client,
        beacon_rpc_client,
        beacon_light_client,
        max_message_num_per_relaying: config.general.max_message_num_per_relaying,
    })
}

async fn start_delivery() -> color_eyre::Result<()> {
    let mut message_relay_service = message_relay_client_builder().await?;
    loop {
        if let Err(error) = message_relay_service.message_relay().await {
            tracing::error!(
                target: "darwinia-goerli",
                "[MessageDelivery][goerli=>Darwinia] Failed to relay message: {:?}",
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
                target: "darwinia-goerli",
                "[MessageConfirmation][goerli=>Darwinia] Failed to confirm message: {:?}",
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
    pub beacon_rpc_client: EthereumClient,
    pub beacon_light_client: DarwiniaClient,
    pub max_message_num_per_relaying: u64,
}

impl<S0: RelayStrategy, S1: RelayStrategy> MessageRelay<S0, S1> {
    async fn message_relay(&mut self) -> color_eyre::Result<()> {
        let received_nonce = self.target.inbound.inbound_lane_nonce(None).await?;
        let latest_nonce = self.source.outbound.outbound_lane_nonce(None).await?;

        if received_nonce.last_delivered_nonce == latest_nonce.latest_generated_nonce {
            tracing::info!(
                target: "darwinia-goerli",
                "[MessageDelivery][Ethereum=>Darwinia] Last delivered nonce is {:?}, equal to lastest generated. Do nothing.",
                received_nonce.last_delivered_nonce,
            );
            return Ok(());
        }

        let finalized_block_number = match self.best_source_block_at_target().await? {
            None => {
                tracing::info!(
                    target: "darwinia-goerli",
                    "[MessageDelivery][Ethereum=>Darwinia] Wait for execution layer relay",
                );
                return Ok(());
            }
            Some(num) => num,
        };

        let outbound_nonce = self
            .source
            .outbound
            .outbound_lane_nonce(Some(BlockId::Number(BlockNumber::from(
                finalized_block_number,
            ))))
            .await?;

        let (begin, end) = (
            latest_nonce.latest_received_nonce + 1,
            latest_nonce.latest_generated_nonce,
        );

        if received_nonce.last_delivered_nonce >= outbound_nonce.latest_generated_nonce {
            tracing::info!(
                target: "darwinia-goerli",
                "[MessageDelivery][Ethereum=>Darwinia] Messages: [{:?}, {:?}] need to be relayed, wait for header relay",
                begin,
                end
            );
            return Ok(());
        }

        let (begin, end) = (
            outbound_nonce.latest_received_nonce + 1,
            outbound_nonce.latest_generated_nonce,
        );

        tracing::info!(
            target: "darwinia-goerli",
            "[MessageDelivery][Ethereum=>Darwinia] Try to relay messages: [{:?}, {:?}]",
            received_nonce.last_delivered_nonce + 1,
            end
        );

        let proof = self
            .source
            .prepare_for_messages_delivery(
                begin,
                end,
                Some(BlockNumber::from(finalized_block_number)),
            )
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
            let is_delivered = current <= received_nonce.last_delivered_nonce;
            let beyond_confirm_limit =
                current - received_nonce.last_confirmed_nonce > confirm_limit;

            if beyond_confirm_limit {
                break;
            }

            if is_delivered {
                delivered += 1;
                count += 1;
                continue;
            }

            if self.source.strategy.decide(*key).await? {
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
                target: "darwinia-goerli",
                "[MessageDelivery][Ethereum=>Darwinia] No need to relay",
            );
            return Ok(());
        }
        tracing::info!(
            target: "darwinia-goerli",
            "[MessageDelivery][Ethereum=>Darwinia] Relaying messages: [{:?}, {:?}]",
            begin + delivered,
            begin + count - 1,
        );

        let gas_price = self.beacon_light_client.gas_price().await?;
        let gas = U256::from_dec_str("30000000")?;
        let tx = self
            .target
            .inbound
            .receive_messages_proof(
                proof,
                U256::from(count),
                &self.target.private_key()?,
                Options {
                    gas: Some(gas),
                    gas_price: Some(gas_price),
                    ..Default::default()
                },
            )
            .await?;

        tracing::info!(
            target: "darwinia-goerli",
            "[MessageDelivery][Ethereum=>Darwinia] Sending tx: {:?}",
            tx
        );

        wait_for_transaction_confirmation(
            tx,
            self.target.client.transport(),
            Duration::from_secs(5),
            1,
        )
        .await?;

        Ok(())
    }

    async fn best_source_block_at_target(&self) -> color_eyre::Result<Option<u64>> {
        let finalized = self
            .beacon_light_client
            .beacon_light_client
            .finalized_header()
            .await?;
        let block = self
            .beacon_rpc_client
            .get_beacon_block(finalized.slot)
            .await?;
        let execution_state_root = self
            .beacon_light_client
            .execution_layer_state_root(None)
            .await?;
        if execution_state_root != H256::from_str(&block.body.execution_payload.state_root)? {
            Ok(None)
        } else {
            Ok(Some(block.body.execution_payload.block_number.parse()?))
        }
    }
}

impl<S0: RelayStrategy, S1: RelayStrategy> MessageRelay<S0, S1> {
    pub async fn message_confirm(&self) -> color_eyre::Result<()> {
        let source_outbound_lane_data = self.source.outbound.outbound_lane_nonce(None).await?;
        if source_outbound_lane_data.latest_received_nonce
            == source_outbound_lane_data.latest_generated_nonce
        {
            tracing::info!(
                target: "darwinia-goerli",
                "[MessageConfirmation][Ethereum=>Darwinia] All confirmed({:?}), nothing to do.",
                source_outbound_lane_data
            );
            return Ok(());
        }

        // query last relayed header
        // Since the header at block number x from darwinia means the state at block number x - 1,
        // we need to minus 1 to get the relay block number.
        let last_relayed_target_block_in_source = self.best_target_block_at_source().await? - 1;

        // assemble unrewarded relayers state
        let target_inbound_state = self
            .target
            .inbound
            .inbound_lane_nonce(Some(BlockId::from(BlockNumber::from(
                last_relayed_target_block_in_source,
            ))))
            .await?;
        let (begin, end) = (
            target_inbound_state.relayer_range_front,
            target_inbound_state.relayer_range_back,
        );
        if source_outbound_lane_data.latest_received_nonce
            == target_inbound_state.last_delivered_nonce
        {
            tracing::info!(
                target: "darwinia-goerli",
                "[MessageConfirmation][Ethereum=>Darwinia] Nonce {:?} was confirmed, wait for delivery from {:?} to {:?}. ",
                source_outbound_lane_data.latest_received_nonce,
                target_inbound_state.last_delivered_nonce + 1,
                source_outbound_lane_data.latest_generated_nonce
            );
            return Ok(());
        }

        tracing::info!(
            target: "darwinia-goerli",
            "[MessageConfirmation][Ethereum=>Darwinia] Try to confirm nonces [{:?}:{:?}]",
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

        let gas_price = self.source.gas_price().await?;
        // send proof
        let hash = self
            .source
            .outbound
            .receive_messages_delivery_proof(
                proof,
                &self.source.private_key()?,
                Options {
                    gas: Some(U256::from_dec_str("2000000")?),
                    gas_price: Some(gas_price),
                    ..Default::default()
                },
            )
            .await?;

        tracing::info!(
            target: "relay-s2s",
            "[MessageConfirmation][Ethereum=>Darwinia] Messages confirmation tx: {:?}",
            hash
        );
        wait_for_transaction_confirmation(
            hash,
            self.source.client.transport(),
            Duration::from_secs(5),
            1,
        )
        .await?;

        Ok(())
    }

    async fn best_target_block_at_source(&self) -> color_eyre::Result<u64> {
        Ok(self.posa_light_client.block_number().await?.as_u64())
    }
}
