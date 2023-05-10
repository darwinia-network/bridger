use std::marker::PhantomData;
use std::str::FromStr;

use bridge_e2e_traits::client::{EcdsaClient, OnDemandHeader};
use lifeline::dyn_bus::DynBus;
use relay_e2e::message::darwinia_message_client::DarwiniaMessageClient;
use relay_e2e::message::ethereum_message_client::EthMessageClient;
use relay_e2e::message::message_relay_runner::{ChannelState, MessageRelayRunner};
use tokio::sync::broadcast::Sender;
use web3::types::{Address, U256};

use crate::bridge::BridgeBus;
use crate::config::BridgeConfig;
use crate::service::header_relay::types::{DarwiniaHeader, EthereumHeader};
use lifeline::{Lifeline, Service, Task, Bus};
use support_lifeline::service::BridgeService;
use support_toolkit::timecount::TimeCount;

#[derive(Debug)]
pub struct DarwiniaEthereumMessageRelay<T: EcdsaClient> {
    _greet_delivery: Lifeline,
    _greet_confirmation: Lifeline,
    _ecdsa: PhantomData<T>,
}

impl<T: EcdsaClient> BridgeService for DarwiniaEthereumMessageRelay<T> {}

impl<T: EcdsaClient> Service for DarwiniaEthereumMessageRelay<T> {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<T> = bus.storage().clone_resource()?;
        let config = bridge_config.clone();
        let _greet_delivery = Self::try_task("message-relay-darwinia-to-eth", async move {
            let mut timecount = TimeCount::new();
            while let Err(error) = start_delivery(config.clone()).await {
                tracing::error!(
                    target: "darwinia-eth",
                    "Failed to start darwinia-to-eth message relay service, restart after some seconds: {:?}",
                    error
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "darwinia-eth",
                        "[message-darwinia-eth-delivery] many errors occurred, wait {} seconds",
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
            Ok(())
        });
        let _rx = bus.rx::<EthereumHeader>()?;
        let tx = bus.tx::<EthereumHeader>()?;
        let _greet_confirmation = Self::try_task(
            "message-confirmation-darwinia-to-eth",
            async move {
                let mut timecount = TimeCount::new();
                while let Err(error) = start_confirmation(bridge_config.clone(), tx.clone()).await {
                    tracing::error!(
                        target: "darwinia-eth",
                        "Failed to start darwinia-to-eth message confirmation service, restart after some seconds: {:?}",
                        error
                    );
                    if let Err(duration) = timecount.plus_and_check() {
                        tokio::time::sleep(duration).await;
                        tracing::error!(
                            target: "darwinia-eth",
                            "[message-darwinia-eth-confirmation] many errors occurred, wait {} seconds",
                            duration.as_secs(),
                        );
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_delivery,
            _greet_confirmation,
            _ecdsa: Default::default(),
        })
    }
}

pub fn message_relay_client_builder<T, O1, O2>(
    config: BridgeConfig<T>,
    delivery_channel_tx: Option<Sender<O1>>,
    confirm_channel_tx: Option<Sender<O2>>,
) -> color_eyre::Result<MessageRelayRunner<DarwiniaMessageClient, EthMessageClient, O1, O2>>
where
    T: EcdsaClient,
    O1: OnDemandHeader,
    O2: OnDemandHeader,
{
    let eth_message_client = EthMessageClient::new_with_simple_fee_market(
        "Eth",
        &config.ethereum.endpoint,
        Address::from_str(&config.ethereum.inbound_address)?,
        Address::from_str(&config.ethereum.outbound_address)?,
        Address::from_str(&config.ethereum.fee_market_address)?,
        Address::from_str(&config.ethereum.posa_light_client_address)?,
        &config.ethereum.private_key,
        U256::from_dec_str(&config.ethereum.max_gas_price)?,
        &config.ethereum.etherscan_api_key,
    )?;
    let darwinia_message_client = DarwiniaMessageClient::new_with_fee_market(
        "Substrate",
        &config.darwinia_evm.endpoint,
        &config.beacon.endpoint,
        config.beacon.api_supplier,
        Address::from_str(&config.darwinia_evm.inbound_address)?,
        Address::from_str(&config.darwinia_evm.outbound_address)?,
        Address::from_str(&config.darwinia_evm.chain_message_committer_address)?,
        Address::from_str(&config.darwinia_evm.lane_message_committer_address)?,
        Address::from_str(&config.darwinia_evm.fee_market_address)?,
        Address::from_str(&config.darwinia_evm.contract_address)?,
        U256::from_dec_str(&config.darwinia_evm.max_gas_price)?,
        &config.darwinia_evm.private_key,
        config.evm_index,
    )?;

    Ok(MessageRelayRunner {
        state: ChannelState::default(),
        max_message_num_per_relaying: config.general.max_message_num_per_relaying,
        source: darwinia_message_client,
        target: eth_message_client,
        relay_notifier: delivery_channel_tx,
        confirm_notifier: confirm_channel_tx,
    })
}

async fn start_delivery<T: EcdsaClient>(config: BridgeConfig<T>) -> color_eyre::Result<()> {
    let mut service =
        message_relay_client_builder::<_, DarwiniaHeader, EthereumHeader>(config, None, None)?;
    loop {
        if let Err(error) = service.message_relay().await {
            tracing::error!(
                target: "substrate-eth",
                "[MessagesDelivery][Substrate=>Eth] Failed to relay messages: {:?}",
                error
            );
            return Err(error.into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

async fn start_confirmation<T: EcdsaClient>(
    config: BridgeConfig<T>,
    confirm_channel_tx: Sender<EthereumHeader>,
) -> color_eyre::Result<()> {
    let mut service = message_relay_client_builder::<_, DarwiniaHeader, EthereumHeader>(
        config,
        None,
        Some(confirm_channel_tx),
    )?;
    loop {
        if let Err(error) = service.message_confirm().await {
            tracing::error!(
                target: "substrate-eth",
                "[MessagesConfirmation][Substrate=>Eth] Failed to confirm messages: {:?}",
                error
            );
            return Err(error.into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}
