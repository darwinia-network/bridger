use std::{str::FromStr, time::Duration};

use bridge_e2e_traits::client::EthTruthLayerLightClient;
use client_beacon::{client::BeaconApiClient, types::Proof};
use web3::{
    contract::{tokens::Tokenize, Options},
    ethabi::Token,
    types::{H256, U256},
};

use crate::error::{RelayError, RelayResult};

pub struct ExecutionLayerRelayRunner<C: EthTruthLayerLightClient> {
    eth_light_client: C,
    beacon_api_client: BeaconApiClient,
}

impl<C: EthTruthLayerLightClient> ExecutionLayerRelayRunner<C> {
    pub async fn start(&mut self) -> RelayResult<()> {
        loop {
            self.run().await?;
            tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        }
    }

    pub async fn run(&mut self) -> RelayResult<()> {
        let last_relayed_header = self
            .eth_light_client
            .beacon_light_client()
            .finalized_header()
            .await?;
        let finalized_block = self
            .beacon_api_client
            .get_beacon_block(last_relayed_header.slot)
            .await?;
        let latest_execution_payload_state_root =
            H256::from_str(&finalized_block.body.execution_payload.state_root)
                .map_err(|e| RelayError::Custom(format!("{}", e)))?;
        let relayed_state_root = self
            .eth_light_client
            .execution_layer()
            .merkle_root(None)
            .await?;

        if relayed_state_root != latest_execution_payload_state_root {
            tracing::info!(
                target: "relay-e2e",
                "[ExecutionLayer] Try to relay execution layer state at slot: {:?}",
                last_relayed_header.slot,
            );

            let state_root_branch = self
                .beacon_api_client
                .get_latest_execution_payload_state_root_branch(last_relayed_header.slot)
                .await?;
            let witnesses = match state_root_branch {
                Proof::SingleProof {
                    gindex: _,
                    leaf: _,
                    witnesses,
                } => witnesses,
                _ => return Err(RelayError::Custom("Not implemented!".to_string()).into()),
            };
            let parameter =
                Token::Tuple((latest_execution_payload_state_root, witnesses).into_tokens());

            let gas_price = self.eth_light_client.gas_price().await?;
            let tx = self
                .eth_light_client
                .execution_layer()
                .contract
                .signed_call(
                    "import_latest_execution_payload_state_root",
                    (parameter,),
                    Options {
                        gas: Some(U256::from(10000000)),
                        gas_price: Some(gas_price),
                        ..Default::default()
                    },
                    self.eth_light_client.private_key(),
                )
                .await?;
            tracing::info!(
                target: "relay-e2e",
                "[ExecutionLayer] Sending tx: {:?}",
                &tx
            );
            support_etherscan::wait_for_transaction_confirmation(
                tx,
                self.eth_light_client.get_web3().transport(),
                Duration::from_secs(5),
                3,
            )
            .await?;
        } else {
            tracing::info!(
                target: "relay-e2e",
                "[ExecutionLayer] Latest execution payload state root at slot {:?} is : {:?}",
                last_relayed_header.slot,
                &relayed_state_root,
            );
        }
        Ok(())
    }
}
