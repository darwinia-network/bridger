use bridge_e2e_traits::client::EthTruthLayerLightClient;
use client_beacon::client::BeaconApiClient;
use client_contracts::execution_layer::types::{BeaconBlockBody, ExecutionPayload};
use std::time::Duration;
use tree_hash::TreeHash;
use types::{ExecPayload, MainnetEthSpec, BeaconBlockCapella};
use web3::{
    contract::Options,
    types::{H256, U256, H160},
};

use crate::error::RelayResult;

pub struct ExecutionLayerRelayRunner<C: EthTruthLayerLightClient> {
    pub eth_light_client: C,
    pub beacon_api_client: BeaconApiClient,
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
        let latest_execution_payload_state_root = H256::from(finalized_block
            .body
            .execution_payload
            .execution_payload
            .state_root
            .clone().0);
        let relayed_state_root = self
            .eth_light_client
            .execution_layer()
            .merkle_root(None)
            .await?;

        if relayed_state_root == latest_execution_payload_state_root {
            tracing::info!(
                target: "relay-e2e",
                "[ExecutionLayer] Latest execution payload state root at slot {:?} is : {:?}",
                last_relayed_header.slot,
                &relayed_state_root,
            );
            return Ok(());
        }

        tracing::info!(
            target: "relay-e2e",
            "[ExecutionLayer] Try to relay execution layer state at slot: {:?}",
            last_relayed_header.slot,
        );

        let parameter = build_execution_layer_update(&finalized_block);
        let gas_price = self.eth_light_client.gas_price().await?;
        let tx = self
            .eth_light_client
            .execution_layer()
            .import_latest_execution_payload_state_root(
                parameter,
                self.eth_light_client.private_key(),
                Options {
                    gas: Some(U256::from(5000000)),
                    gas_price: Some(gas_price),
                    ..Default::default()
                },
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
            1,
        )
        .await?;
        Ok(())
    }
}

fn build_execution_layer_update(block: &BeaconBlockCapella<MainnetEthSpec>) -> BeaconBlockBody {
    BeaconBlockBody {
        randao_reveal: H256::from(block.body.randao_reveal.tree_hash_root().0),
        eth1_data: H256::from(block.body.eth1_data.tree_hash_root().0),
        graffiti: H256::from(block.body.graffiti.0),
        proposer_slashings: H256::from(block.body.proposer_slashings.tree_hash_root().0),
        attester_slashings: H256::from(block.body.attester_slashings.tree_hash_root().0),
        attestations: H256::from(block.body.attestations.tree_hash_root().0),
        deposits: H256::from(block.body.deposits.tree_hash_root().0),
        voluntary_exits: H256::from(block.body.voluntary_exits.tree_hash_root().0),
        sync_aggregate: H256::from(block.body.sync_aggregate.tree_hash_root().0),
        execution_payload: ExecutionPayload {
            parent_hash: H256::from(block.body.execution_payload.parent_hash().into_root().0),
            fee_recipient: H160::from(block.body.execution_payload.fee_recipient().0),
            state_root: H256::from(block.body.execution_payload.execution_payload.state_root.0),
            receipts_root: H256::from(block.body.execution_payload.execution_payload.receipts_root.0),
            logs_bloom: H256::from(block
                .body
                .execution_payload
                .execution_payload
                .logs_bloom
                .tree_hash_root().0),
            prev_randao: H256::from(block.body.execution_payload.execution_payload.prev_randao.0),
            block_number: block.body.execution_payload.block_number(),
            gas_limit: block.body.execution_payload.gas_limit(),
            gas_used: block.body.execution_payload.execution_payload.gas_used,
            timestamp: block.body.execution_payload.timestamp(),
            extra_data: H256::from(block
                .body
                .execution_payload
                .execution_payload
                .extra_data
                .tree_hash_root().0),
            base_fee_per_gas: U256::from(block
                .body
                .execution_payload
                .execution_payload
                .base_fee_per_gas.as_u128()),
            block_hash: H256::from(block
                .body
                .execution_payload
                .execution_payload
                .block_hash
                .into_root().0),
            transactions: H256::from(block
                .body
                .execution_payload
                .execution_payload
                .transactions
                .tree_hash_root().0),
        },
    }
}
