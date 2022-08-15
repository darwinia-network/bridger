use crate::service::ecdsa_relay::types::EcdsaSource;
use client_contracts::posa_light_client::Commitment;
use support_common::error::BridgerError;
use web3::signing::SecretKeyRef;
use web3::types::{H256, U256};

pub struct CollectedEnoughNewMessageRootSignaturesRunner {
    source: EcdsaSource,
}

impl CollectedEnoughNewMessageRootSignaturesRunner {
    pub fn new(source: EcdsaSource) -> Self {
        Self { source }
    }
}

impl CollectedEnoughNewMessageRootSignaturesRunner {
    pub async fn start(&self) -> color_eyre::Result<Option<u32>> {
        let client_posa = &self.source.client_posa;
        let client_pangoro_web3 = &self.source.client_pangoro_web3;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let pangoro_evm_account = &self.source.pangoro_evm_account;
        let ethereum_account = &self.source.ethereum_account;

        let cacse = subquery
            .next_collected_enough_new_message_root_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] no more new message root signatures events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");

        let signature_nodes = event.signatures.nodes;
        let signatures = signature_nodes
            .iter()
            .map(|item| item.signature.clone())
            .collect::<Vec<Vec<u8>>>();

        let mr_slice: [u8; 32] = event
            .commitment_message_root
            .try_into()
            .map_err(|e| BridgerError::Custom(format!("Wrong message root: {:?}", e)))?;
        // let n_slice: [u64; 4] = event
        //     .commitment_nonce // wrong commitment nonce types
        //     .try_into()
        //     .map_err(|e| BridgerError::Custom(format!("Wrong nonce: {:?}", e)))?;
        // let commitment = Commitment {
        //     block_number: event.commitment_block_number,
        //     message_root: H256(mr_slice),
        //     nonce: U256(n_slice),
        // };
        // let _hash = client_posa
        //     .import_message_commitment(commitment, signatures, &ethereum_account.secret_key()?)
        //     .await?;
        Ok(None)
    }
}
