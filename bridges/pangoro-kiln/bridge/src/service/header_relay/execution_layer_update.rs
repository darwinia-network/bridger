use std::{
    ops::{Add, Div},
    str::FromStr,
};

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
    contract::{
        tokens::{Tokenizable, Tokenize},
        Options,
    },
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
        let _greet = Self::try_task(&format!("execution-kiln-to-pangoro"), async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start kiln-to-pangoro execution payload state root relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
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
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

pub struct ExecutionLayer {
    pub pangoro_client: PangoroClient,
    pub kiln_client: KilnClient,
}

impl ExecutionLayer {
    pub async fn execution_layer_relay(&self) -> color_eyre::Result<()> {
        todo!()
    }
}
