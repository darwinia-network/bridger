use std::str::FromStr;

use crate::bridge::{BridgeBus, BridgeConfig};
use client_beacon::client::BeaconApiClient;
use lifeline::{Lifeline, Service, Task};
use relay_e2e::header::{
    common::EthLightClient, eth_execution_layer_relay::ExecutionLayerRelayRunner,
};
use support_common::config::{Config, Names};

use support_lifeline::service::BridgeService;
use web3::types::{Address, U256};

#[derive(Debug)]
pub struct ExecutionLayerRelay {
    _greet: Lifeline,
}

impl BridgeService for ExecutionLayerRelay {}

impl Service for ExecutionLayerRelay {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("execution-layer-goerli-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-goerli",
                    "Failed to start goerli-to-pangoro execution payload state root relay service, restart after some seconds: {:?}",
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
    let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;
    let pangoro_client = EthLightClient::new(
        &config.pangoro_evm.endpoint,
        Address::from_str(&config.pangoro_evm.contract_address)?,
        Address::from_str(&config.pangoro_evm.execution_layer_contract_address)?,
        &config.pangoro_evm.private_key,
        U256::from_dec_str(&config.pangoro_evm.max_gas_price)?,
    )?;
    let goerli_client = BeaconApiClient::new(&config.goerli.endpoint)?;
    let mut execution_layer_relay = ExecutionLayerRelayRunner {
        eth_light_client: pangoro_client,
        beacon_api_client: goerli_client,
    };

    execution_layer_relay.start().await?;
    Ok(())
}
