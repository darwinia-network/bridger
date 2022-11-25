use std::str::FromStr;

use relay_e2e::message::darwinia_message_client::DarwiniaMessageClient;
use relay_e2e::message::ethereum_message_client::EthMessageClient;
use relay_e2e::message::message_relay_runner::{ChannelState, MessageRelayRunner};
use web3::types::{Address, U256};

use crate::bridge::{BridgeBus, BridgeConfig};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;
use thegraph::types::LikethChain;

#[derive(Debug)]
pub struct DarwiniaEthereumMessageRelay {
    _greet_delivery: Lifeline,
    _greet_confirmation: Lifeline,
}

impl BridgeService for DarwiniaEthereumMessageRelay {}

impl Service for DarwiniaEthereumMessageRelay {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task("message-relay-darwinia-to-eth", async move {
            while let Err(error) = start_delivery().await {
                tracing::error!(
                    target: "darwinia-eth",
                    "Failed to start darwinia-to-eth message relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
            Ok(())
        });
        let _greet_confirmation = Self::try_task(
            "message-confirmation-darwinia-to-eth",
            async move {
                while let Err(error) = start_confirmation().await {
                    tracing::error!(
                        target: "darwinia-eth",
                        "Failed to start darwinia-to-eth message confirmation service, restart after some seconds: {:?}",
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

pub async fn message_relay_client_builder(
) -> color_eyre::Result<MessageRelayRunner<DarwiniaMessageClient, EthMessageClient>> {
    let config: BridgeConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;

    let eth_message_client = EthMessageClient::new_with_simple_fee_market(
        "Eth",
        &config.eth.execution_layer_endpoint,
        Address::from_str(&config.eth.inbound_address)?,
        Address::from_str(&config.eth.outbound_address)?,
        Address::from_str(&config.eth.fee_market_address)?,
        Address::from_str(&config.eth.posa_light_client_address)?,
        &config.eth.private_key,
        U256::from_dec_str(&config.eth.max_gas_price)?,
        &config.eth.etherscan_api_key,
    )?;
    let darwinia_message_client = DarwiniaMessageClient::new_with_fee_market(
        "Darwinia",
        &config.darwinia_evm.endpoint,
        &config.eth.endpoint,
        config.eth.api_supplier,
        Address::from_str(&config.darwinia_evm.inbound_address)?,
        Address::from_str(&config.darwinia_evm.outbound_address)?,
        Address::from_str(&config.darwinia_evm.chain_message_committer_address)?,
        Address::from_str(&config.darwinia_evm.lane_message_committer_address)?,
        Address::from_str(&config.darwinia_evm.fee_market_address)?,
        Address::from_str(&config.darwinia_evm.contract_address)?,
        Address::from_str(&config.darwinia_evm.execution_layer_contract_address)?,
        U256::from_dec_str(&config.darwinia_evm.max_gas_price)?,
        &config.darwinia_evm.private_key,
        config.index.to_evm_thegraph(LikethChain::Darwinia)?,
    )?;

    Ok(MessageRelayRunner {
        state: ChannelState::default(),
        max_message_num_per_relaying: config.general.max_message_num_per_relaying,
        source: darwinia_message_client,
        target: eth_message_client,
    })
}

async fn start_delivery() -> color_eyre::Result<()> {
    let mut service = message_relay_client_builder().await?;
    loop {
        if let Err(error) = service.message_relay().await {
            tracing::error!(
                target: "darwinia-eth",
                "[MessagesDelivery][Darwinia=>Eth] Failed to relay messages: {:?}",
                error
            );
            return Err(error.into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

async fn start_confirmation() -> color_eyre::Result<()> {
    let mut service = message_relay_client_builder().await?;
    loop {
        if let Err(error) = service.message_confirm().await {
            tracing::error!(
                target: "darwinia-eth",
                "[MessagesConfirmation][Darwinia=>Eth] Failed to confirm messages: {:?}",
                error
            );
            return Err(error.into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}
