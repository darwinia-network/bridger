use crate::{
    bridge::{BridgeBus, BridgeConfig},
    pangoro_client::client::PangoroClient,
};
use client_beacon::client::BeaconApiClient;

use lifeline::{Lifeline, Service, Task};
use relay_e2e::header::eth_beacon_header_relay::BeaconHeaderRelayRunner;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;
use web3::types::U256;

#[derive(Debug)]
pub struct GoerliToPangoroHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for GoerliToPangoroHeaderRelayService {}

impl Service for GoerliToPangoroHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("header-goerli-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-goerli",
                    "Failed to start goerli-to-pangoro header relay service, restart after some seconds: {:?}",
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
    let config: BridgeConfig = Config::restore(Names::BridgePangoroGoerli)?;
    let pangoro_client = PangoroClient::new(
        &config.pangoro_evm.endpoint,
        &config.pangoro_evm.contract_address,
        &config.pangoro_evm.execution_layer_contract_address,
        &config.pangoro_evm.private_key,
        U256::from_dec_str(&config.pangoro_evm.max_gas_price)?,
    )?;
    let goerli_client = BeaconApiClient::new(&config.goerli.endpoint)?;
    let mut header_relay = BeaconHeaderRelayRunner {
        eth_light_client: pangoro_client,
        beacon_api_client: goerli_client,
        minimal_interval: config.general.header_relay_minimum_interval,
        last_relay_time: u64::MIN,
    };

    loop {
        if let Err(error) = header_relay.start().await {
            tracing::error!(
                target: "pangoro-goerli",
                "Failed relay header : {:?}",
                error
            );
            return Err(error.into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
