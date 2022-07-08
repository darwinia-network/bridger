use std::str::FromStr;

use crate::{
    bridge::{BridgeConfig, PangoroKilnBus},
    kiln_client::{client::KilnClient, types::Proof},
    pangoro_client::client::PangoroClient,
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
    type Bus = PangoroKilnBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("execution-layer-kiln-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start kiln-to-pangoro execution payload state root relay service, restart after some seconds: {:?}",
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
    let config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;
    println!("{:?}", &config);
    let pangoro_client = PangoroClient::new(
        &config.pangoro.endpoint,
        &config.pangoro.contract_abi_path,
        &config.pangoro.contract_address,
        &config.pangoro.execution_layer_contract_abi_path,
        &config.pangoro.execution_layer_contract_address,
        &config.pangoro.private_key,
    )?;
    let kiln_client = KilnClient::new(&config.kiln.endpoint)?;
    let execution_layer_relay = ExecutionLayer {
        pangoro_client,
        kiln_client,
    };

    loop {
        if let Err(error) = execution_layer_relay.execution_layer_relay().await {
            tracing::error!(
                target: "pangoro-kiln",
                "Failed to relay exection payload state root: {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}

pub struct ExecutionLayer {
    pub pangoro_client: PangoroClient,
    pub kiln_client: KilnClient,
}

impl ExecutionLayer {
    pub async fn execution_layer_relay(&self) -> color_eyre::Result<()> {
        let last_relayed_header = self.pangoro_client.finalized_header().await?;
        let finalized_block = self
            .kiln_client
            .get_beacon_block(last_relayed_header.slot)
            .await?;

        let relayed_state_root = self.pangoro_client.execution_layer_state_root().await?;
        if relayed_state_root.is_zero() {
            tracing::info!(
                target: "pangoro-kiln",
                "[ExecutionLayer][Kiln => Pangoro] Try to relay execution layer state at slot: {:?}",
                last_relayed_header.slot,
            );

            let latest_execution_payload_state_root =
                H256::from_str(&finalized_block.body.execution_payload.state_root)?;
            let state_root_branch = self
                .kiln_client
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
            let tx = self
                .pangoro_client
                .execution_layer_contract
                .signed_call(
                    "import_latest_execution_payload_state_root",
                    (parameter,),
                    Options {
                        gas: Some(U256::from(10000000)),
                        gas_price: Some(U256::from(1300000000)),
                        ..Default::default()
                    },
                    &self.pangoro_client.private_key,
                )
                .await?;

            tracing::info!(
                target: "pangoro-kiln",
                "[ExecutionLayer][Kiln => Pangoro] Sending tx: {:?}",
                &tx
            );
        } else {
            tracing::info!(
                target: "pangoro-kiln",
                "[ExecutionLayer][Kiln => Pangoro] Latest execution payload state root at slot {:?} is : {:?}",
                last_relayed_header.slot,
                &relayed_state_root,
            );
        }
        Ok(())
    }
}
