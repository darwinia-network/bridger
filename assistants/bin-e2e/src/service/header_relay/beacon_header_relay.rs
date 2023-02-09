use std::{marker::PhantomData, str::FromStr};

use crate::bridge::BridgeBus;
use crate::config::BridgeConfig;
use bridge_e2e_traits::client::EcdsaClient;
use client_beacon::client::BeaconApiClient;

use support_toolkit::timecount::TimeCount;
use lifeline::{dyn_bus::DynBus, Lifeline, Service, Task};
use relay_e2e::header::{common::EthLightClient, eth_beacon_header_relay::BeaconHeaderRelayRunner};
use support_lifeline::service::BridgeService;
use web3::types::{Address, U256};

#[derive(Debug)]
pub struct EthereumToDarwiniaHeaderRelayService<T: EcdsaClient> {
    _greet: Lifeline,
    _ecdsa: PhantomData<T>,
}

impl<T: EcdsaClient> BridgeService for EthereumToDarwiniaHeaderRelayService<T> {}

impl<T: EcdsaClient> Service for EthereumToDarwiniaHeaderRelayService<T> {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<T> = bus.storage().clone_resource()?;
        let _greet = Self::try_task("header-eth-to-darwinia", async move {
            let mut timecount = TimeCount::new();
            while let Err(error) = Self::start(bridge_config.clone()).await {
                tracing::error!(
                    target: "substrate-eth",
                    "Failed to start header relay service, restart after some seconds: {:?}",
                    error
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "substrate-eth",
                        "[beacon-header-relay] many errors occurred, wait {} seconds",
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
            Ok(())
        });
        Ok(Self {
            _greet,
            _ecdsa: Default::default(),
        })
    }
}

impl<T: EcdsaClient> EthereumToDarwiniaHeaderRelayService<T> {
    async fn start(config: BridgeConfig<T>) -> color_eyre::Result<()> {
        let darwinia_client = EthLightClient::new(
            &config.darwinia_evm.endpoint,
            Address::from_str(&config.darwinia_evm.contract_address)?,
            Address::from_str(&config.darwinia_evm.execution_layer_contract_address)?,
            &config.darwinia_evm.private_key,
            U256::from_dec_str(&config.darwinia_evm.max_gas_price)?,
        )?;
        let eth_client = BeaconApiClient::new(&config.beacon.endpoint, config.beacon.api_supplier)?;
        let mut header_relay = BeaconHeaderRelayRunner {
            eth_light_client: darwinia_client,
            beacon_api_client: eth_client,
            minimal_interval: config.general.header_relay_minimum_interval,
            last_relay_time: u64::MIN,
        };

        header_relay.start().await?;
        Ok(())
    }
}
