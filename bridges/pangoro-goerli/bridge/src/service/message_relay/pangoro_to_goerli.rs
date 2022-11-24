use std::str::FromStr;

use relay_e2e::message::darwinia_message_client::DarwiniaMessageClient;
use relay_e2e::message::ethereum_message_client::EthMessageClient;
use relay_e2e::message::message_relay_runner::{ChannelState, MessageRelayRunner};
use web3::types::{Address, U256};

use crate::bridge::{BridgeBus, BridgeConfig};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

#[derive(Debug)]
pub struct PangoroGoerliMessageRelay {
    _greet_delivery: Lifeline,
    _greet_confirmation: Lifeline,
}

impl BridgeService for PangoroGoerliMessageRelay {}

impl Service for PangoroGoerliMessageRelay {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task("message-relay-pangoro-to-goerli", async move {
            while let Err(error) = start_delivery().await {
                tracing::error!(
                    target: "pangoro-goerli",
                    "Failed to start pangoro-to-goerli message relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
            Ok(())
        });
        let _greet_confirmation = Self::try_task(
            "message-confirmation-pangoro-to-goerli",
            async move {
                while let Err(error) = start_confirmation().await {
                    tracing::error!(
                        target: "pangoro-goerli",
                        "Failed to start pangoro-to-goerli message confirmation service, restart after some seconds: {:?}",
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
    let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;

    let eth_message_client = EthMessageClient::new_with_simple_fee_market(
        "Goerli",
        &config.goerli.execution_layer_endpoint,
        Address::from_str(&config.goerli.inbound_address)?,
        Address::from_str(&config.goerli.outbound_address)?,
        Address::from_str(&config.goerli.fee_market_address)?,
        Address::from_str(&config.goerli.posa_light_client_address)?,
        &config.goerli.private_key,
        U256::from_dec_str(&config.goerli.max_gas_price)?,
        &config.goerli.etherscan_api_key,
    )?;
    let darwinia_message_client = DarwiniaMessageClient::new_with_fee_market(
        "Pangoro",
        &config.pangoro_evm.endpoint,
        &config.goerli.endpoint,
        Address::from_str(&config.pangoro_evm.inbound_address)?,
        Address::from_str(&config.pangoro_evm.outbound_address)?,
        Address::from_str(&config.pangoro_evm.chain_message_committer_address)?,
        Address::from_str(&config.pangoro_evm.lane_message_committer_address)?,
        Address::from_str(&config.pangoro_evm.fee_market_address)?,
        Address::from_str(&config.pangoro_evm.contract_address)?,
        Address::from_str(&config.pangoro_evm.execution_layer_contract_address)?,
        U256::from_dec_str(&config.pangoro_evm.max_gas_price)?,
        &config.pangoro_evm.private_key,
        config.index.to_pangoro_thegraph()?,
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
                target: "pangoro-goerli",
                "[MessagesDelivery][Pangoro=>Goerli] Failed to relay messages: {:?}",
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
                target: "pangoro-goerli",
                "[MessagesConfirmation][Pangoro=>Goerli] Failed to confirm messages: {:?}",
                error
            );
            return Err(error.into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}
