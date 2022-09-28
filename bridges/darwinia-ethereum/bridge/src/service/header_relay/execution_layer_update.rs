use std::{str::FromStr, time::Duration};

use crate::{
    bridge::{BridgeBus, BridgeConfig},
    eth_client::{client::EthClient, types::Proof},
    darwinia_client::client::DarwiniaClient,
    web3_helper::{wait_for_transaction_confirmation, GasPriceOracle},
};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use web3::{
    contract::{tokens::Tokenize, Options},
    ethabi::Token,
    types::{H256, U256},
};

#[derive(Debug)]
pub struct ExecutionLayerRelay {
    _greet: Lifeline,
}

impl BridgeService for ExecutionLayerRelay {}

impl Service for ExecutionLayerRelay {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("execution-layer-eth-to-darwinia", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "darwinia-eth",
                    "Failed to start eth-to-darwinia execution payload state root relay service, restart after some seconds: {:?}",
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
    let config: BridgeConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let darwinia_client = DarwiniaClient::new(
        &config.darwinia_evm.endpoint,
        &config.darwinia_evm.contract_address,
        &config.darwinia_evm.execution_layer_contract_address,
        &config.darwinia_evm.private_key,
        U256::from_dec_str(&config.darwinia_evm.max_gas_price)?,
    )?;
    let eth_client = EthClient::new(&config.eth.endpoint)?;
    let execution_layer_relay = ExecutionLayer {
        darwinia_client,
        eth_client,
    };

    loop {
        if let Err(error) = execution_layer_relay.execution_layer_relay().await {
            tracing::error!(
                target: "darwinia-eth",
                "Failed to relay exection payload state root: {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

pub struct ExecutionLayer {
    pub darwinia_client: DarwiniaClient,
    pub eth_client: EthClient,
}

impl ExecutionLayer {
    pub async fn execution_layer_relay(&self) -> color_eyre::Result<()> {
        let last_relayed_header = self
            .darwinia_client
            .beacon_light_client
            .finalized_header()
            .await?;
        let finalized_block = self
            .eth_client
            .get_beacon_block(last_relayed_header.slot)
            .await?;
        let latest_execution_payload_state_root =
            H256::from_str(&finalized_block.body.execution_payload.state_root)?;
        let relayed_state_root = self.darwinia_client.execution_layer_state_root(None).await?;

        if relayed_state_root != latest_execution_payload_state_root {
            tracing::info!(
                target: "darwinia-eth",
                "[ExecutionLayer][Eth=>Darwinia] Try to relay execution layer state at slot: {:?}",
                last_relayed_header.slot,
            );

            let state_root_branch = self
                .eth_client
                .get_latest_execution_payload_state_root_branch(last_relayed_header.slot)
                .await?;
            let witnesses = match state_root_branch {
                Proof::SingleProof {
                    gindex: _,
                    leaf: _,
                    witnesses,
                } => witnesses,
                _ => return Err(BridgerError::Custom("Not implemented!".to_string()).into()),
            };
            let parameter =
                Token::Tuple((latest_execution_payload_state_root, witnesses).into_tokens());

            let gas_price = self.darwinia_client.gas_price().await?;
            let tx = self
                .darwinia_client
                .execution_layer_contract
                .signed_call(
                    "import_latest_execution_payload_state_root",
                    (parameter,),
                    Options {
                        gas: Some(U256::from(10000000)),
                        gas_price: Some(gas_price),
                        ..Default::default()
                    },
                    &self.darwinia_client.private_key,
                )
                .await?;
            tracing::info!(
                target: "darwinia-eth",
                "[ExecutionLayer][Eth=>Darwinia] Sending tx: {:?}",
                &tx
            );
            wait_for_transaction_confirmation(
                tx,
                self.darwinia_client.client.transport(),
                Duration::from_secs(5),
                3,
            )
            .await?;
        } else {
            tracing::info!(
                target: "darwinia-eth",
                "[ExecutionLayer][Eth=>Darwinia] Latest execution payload state root at slot {:?} is : {:?}",
                last_relayed_header.slot,
                &relayed_state_root,
            );
        }
        Ok(())
    }
}
