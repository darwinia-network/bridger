use bridge_e2e_traits::client::EthTruthLayerLightClient;
use client_beacon::client::BeaconApiClient;
use client_contracts::execution_layer::types::{
    BeaconBlockBodyBellatrix, BeaconBlockBodyCapella, ExecutionPayload, ExecutionPayloadCapella,
};
use std::time::Duration;
use tree_hash::TreeHash;
use types::{
    BeaconBlockBodyCapella as RawBeaconBlockBodyCapella, BeaconBlockBodyMerge, MainnetEthSpec,
};
use web3::{
    contract::Options,
    types::{H160, H256, U256},
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
        let is_capella = self.eth_light_client.execution_layer().is_capella().await?;
        dbg!(is_capella);
        let finalized_block = self
            .beacon_api_client
            .get_beacon_block(last_relayed_header.slot)
            .await?;
        let latest_state_root = match is_capella {
            true => finalized_block
                .body()
                .execution_payload_capella()?
                .execution_payload
                .state_root
                .clone(),
            false => finalized_block
                .body()
                .execution_payload_merge()?
                .execution_payload
                .state_root
                .clone(),
        };
        let relayed_state_root = self
            .eth_light_client
            .execution_layer()
            .merkle_root(None)
            .await?;

        if relayed_state_root == H256::from(latest_state_root.0) {
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

        let gas_price = self.eth_light_client.gas_price().await?;
        let options = Options {
            gas_price: Some(gas_price),
            ..Default::default()
        };

        let tx = match is_capella {
            true => {
                let parameter =
                    build_execution_layer_update_capella(finalized_block.body_capella()?);
                self.eth_light_client
                    .execution_layer()
                    .import_block_body_capella(
                        parameter,
                        self.eth_light_client.private_key(),
                        options,
                    )
                    .await?
            }
            false => {
                let parameter =
                    build_execution_layer_update_bellatrix(finalized_block.body_merge()?);
                self.eth_light_client
                    .execution_layer()
                    .import_block_body_bellatrix(
                        parameter,
                        self.eth_light_client.private_key(),
                        options,
                    )
                    .await?
            }
        };

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

fn build_execution_layer_update_bellatrix(
    block: &BeaconBlockBodyMerge<MainnetEthSpec>,
) -> BeaconBlockBodyBellatrix {
    let execution_payload = &block.execution_payload.execution_payload;
    BeaconBlockBodyBellatrix {
        randao_reveal: H256::from(block.randao_reveal.tree_hash_root().0),
        eth1_data: H256::from(block.eth1_data.tree_hash_root().0),
        graffiti: H256::from(block.graffiti.0),
        proposer_slashings: H256::from(block.proposer_slashings.tree_hash_root().0),
        attester_slashings: H256::from(block.attester_slashings.tree_hash_root().0),
        attestations: H256::from(block.attestations.tree_hash_root().0),
        deposits: H256::from(block.deposits.tree_hash_root().0),
        voluntary_exits: H256::from(block.voluntary_exits.tree_hash_root().0),
        sync_aggregate: H256::from(block.sync_aggregate.tree_hash_root().0),
        execution_payload: ExecutionPayload {
            parent_hash: H256::from(execution_payload.parent_hash.into_root().0),
            fee_recipient: H160::from(execution_payload.fee_recipient.0),
            state_root: H256::from(execution_payload.state_root.0),
            receipts_root: H256::from(execution_payload.receipts_root.0),
            logs_bloom: H256::from(execution_payload.logs_bloom.tree_hash_root().0),
            prev_randao: H256::from(execution_payload.prev_randao.0),
            block_number: execution_payload.block_number,
            gas_limit: execution_payload.gas_limit,
            gas_used: execution_payload.gas_used,
            timestamp: execution_payload.timestamp,
            extra_data: H256::from(execution_payload.extra_data.tree_hash_root().0),
            base_fee_per_gas: U256::from(execution_payload.base_fee_per_gas.as_u128()),
            block_hash: H256::from(execution_payload.block_hash.into_root().0),
            transactions: H256::from(execution_payload.transactions.tree_hash_root().0),
        },
    }
}

fn build_execution_layer_update_capella(
    block: &RawBeaconBlockBodyCapella<MainnetEthSpec>,
) -> BeaconBlockBodyCapella {
    let execution_payload = &block.execution_payload.execution_payload;
    BeaconBlockBodyCapella {
        randao_reveal: H256::from(block.randao_reveal.tree_hash_root().0),
        eth1_data: H256::from(block.eth1_data.tree_hash_root().0),
        graffiti: H256::from(block.graffiti.0),
        proposer_slashings: H256::from(block.proposer_slashings.tree_hash_root().0),
        attester_slashings: H256::from(block.attester_slashings.tree_hash_root().0),
        attestations: H256::from(block.attestations.tree_hash_root().0),
        deposits: H256::from(block.deposits.tree_hash_root().0),
        voluntary_exits: H256::from(block.voluntary_exits.tree_hash_root().0),
        sync_aggregate: H256::from(block.sync_aggregate.tree_hash_root().0),
        execution_payload: ExecutionPayloadCapella {
            parent_hash: H256::from(execution_payload.parent_hash.into_root().0),
            fee_recipient: H160::from(execution_payload.fee_recipient.0),
            state_root: H256::from(execution_payload.state_root.0),
            receipts_root: H256::from(execution_payload.receipts_root.0),
            logs_bloom: H256::from(execution_payload.logs_bloom.tree_hash_root().0),
            prev_randao: H256::from(execution_payload.prev_randao.0),
            block_number: execution_payload.block_number,
            gas_limit: execution_payload.gas_limit,
            gas_used: execution_payload.gas_used,
            timestamp: execution_payload.timestamp,
            extra_data: H256::from(execution_payload.extra_data.tree_hash_root().0),
            base_fee_per_gas: U256::from(execution_payload.base_fee_per_gas.as_u128()),
            block_hash: H256::from(execution_payload.block_hash.into_root().0),
            transactions_root: H256::from(execution_payload.transactions.tree_hash_root().0),
            withdrawals_root: H256::from(execution_payload.withdrawals.tree_hash_root().0),
        },
        bls_to_execution_changes: H256::from(block.bls_to_execution_changes.tree_hash_root().0),
    }
}
