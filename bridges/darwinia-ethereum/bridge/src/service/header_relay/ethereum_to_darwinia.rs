use std::str::FromStr;

use crate::bridge::{BridgeBus, BridgeConfig};
use client_beacon::client::BeaconApiClient;

use lifeline::{Lifeline, Service, Task};
use relay_e2e::header::{common::EthLightClient, eth_beacon_header_relay::BeaconHeaderRelayRunner};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;
use web3::types::{Address, U256};

#[derive(Debug)]
pub struct EthereumToDarwiniaHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for EthereumToDarwiniaHeaderRelayService {}

impl Service for EthereumToDarwiniaHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("header-eth-to-darwinia", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "darwinia-eth",
                    "Failed to start eth-to-darwinia header relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let config: BridgeConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let darwinia_client = EthLightClient::new(
        &config.darwinia_evm.endpoint,
        Address::from_str(&config.darwinia_evm.contract_address)?,
        Address::from_str(&config.darwinia_evm.execution_layer_contract_address)?,
        &config.darwinia_evm.private_key,
        U256::from_dec_str(&config.darwinia_evm.max_gas_price)?,
    )?;
    let eth_client = BeaconApiClient::new(&config.ethereum.endpoint, config.ethereum.api_supplier)?;
    let mut header_relay = BeaconHeaderRelayRunner {
        eth_light_client: darwinia_client,
        beacon_api_client: eth_client,
        minimal_interval: config.general.header_relay_minimum_interval,
        last_relay_time: u64::MIN,
    };

    header_relay.start().await?;
    Ok(())
}
