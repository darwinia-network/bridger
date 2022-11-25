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
pub struct EthereumDarwiniaMessageRelay {
    _greet_delivery: Lifeline,
    _greet_confirmation: Lifeline,
}

impl BridgeService for EthereumDarwiniaMessageRelay {}

impl Service for EthereumDarwiniaMessageRelay {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task("message-relay-eth-to-darwinia", async move {
            while let Err(error) = start_delivery().await {
                tracing::error!(
                    target: "darwinia-eth",
                    "Failed to start eth-to-darwinia message relay service, restart after some seconds: {:?}",
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
                        "Failed to start eth-to-darwinia message confirmation service, restart after some seconds: {:?}",
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
) -> color_eyre::Result<MessageRelayRunner<EthMessageClient, DarwiniaMessageClient>> {
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
        source: eth_message_client,
        target: darwinia_message_client,
    })
}

async fn start_delivery() -> color_eyre::Result<()> {
    let mut message_relay_service = message_relay_client_builder().await?;
    loop {
        if let Err(error) = message_relay_service.message_relay().await {
            tracing::error!(
                target: "darwinia-eth",
                "[MessageDelivery][Eth=>Darwinia] Failed to relay message: {:?}",
                error
            );
            return Err(error.into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

async fn start_confirmation() -> color_eyre::Result<()> {
    let mut message_relay_service = message_relay_client_builder().await?;
    loop {
        if let Err(error) = message_relay_service.message_confirm().await {
            tracing::error!(
                target: "darwinia-eth",
                "[MessageConfirmation][Eth=>Darwinia] Failed to confirm message: {:?}",
                error
            );
            return Err(error.into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}
