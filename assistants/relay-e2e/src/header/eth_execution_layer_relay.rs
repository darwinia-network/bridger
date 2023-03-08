use bridge_e2e_traits::client::EthTruthLayerLightClient;
use client_beacon::client::BeaconApiClient;
use client_contracts::execution_layer::types::{BeaconBlockBody, ExecutionPayload};
use std::time::Duration;
use tree_hash::TreeHash;
use types::{BeaconBlockMerge, ExecPayload, MainnetEthSpec};
use web3::{
    contract::Options,
    types::{H256, U256},
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
        let latest_execution_payload_state_root = finalized_block
            .body
            .execution_payload
            .execution_payload
            .state_root
            .clone();
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
            3,
        )
        .await?;
        Ok(())
    }
}

fn build_execution_layer_update(block: &BeaconBlockMerge<MainnetEthSpec>) -> BeaconBlockBody {
    BeaconBlockBody {
        randao_reveal: block.body.randao_reveal.tree_hash_root(),
        eth1_data: block.body.eth1_data.tree_hash_root(),
        graffiti: H256::from(block.body.graffiti.0),
        proposer_slashings: block.body.proposer_slashings.tree_hash_root(),
        attester_slashings: block.body.attester_slashings.tree_hash_root(),
        attestations: block.body.attestations.tree_hash_root(),
        deposits: block.body.deposits.tree_hash_root(),
        voluntary_exits: block.body.voluntary_exits.tree_hash_root(),
        sync_aggregate: block.body.sync_aggregate.tree_hash_root(),
        execution_payload: ExecutionPayload {
            parent_hash: block.body.execution_payload.parent_hash().into_root(),
            fee_recipient: block.body.execution_payload.fee_recipient(),
            state_root: block.body.execution_payload.execution_payload.state_root,
            receipts_root: block.body.execution_payload.execution_payload.receipts_root,
            logs_bloom: block
                .body
                .execution_payload
                .execution_payload
                .logs_bloom
                .tree_hash_root(),
            prev_randao: block.body.execution_payload.execution_payload.prev_randao,
            block_number: block.body.execution_payload.block_number(),
            gas_limit: block.body.execution_payload.gas_limit(),
            gas_used: block.body.execution_payload.execution_payload.gas_used,
            timestamp: block.body.execution_payload.timestamp(),
            extra_data: block
                .body
                .execution_payload
                .execution_payload
                .extra_data
                .tree_hash_root(),
            base_fee_per_gas: block
                .body
                .execution_payload
                .execution_payload
                .base_fee_per_gas,
            block_hash: block
                .body
                .execution_payload
                .execution_payload
                .block_hash
                .into_root(),
            transactions: block
                .body
                .execution_payload
                .execution_payload
                .transactions
                .tree_hash_root(),
        },
    }
}
