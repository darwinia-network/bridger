use client_contracts::posa_light_client::Commitment;
use web3::types::H256;

use support_common::error::BridgerError;

use crate::service::ecdsa_relay::types::EcdsaSource;

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
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let ethereum_account = &self.source.ethereum_account;

        let cacse = subquery
            .next_collected_enough_new_message_root_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "pangoro-goerli",
                "[pangoro] [ecdsa] no more new message root signatures events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");

        let signature_nodes = event.signatures.nodes;
        let signatures = signature_nodes
            .iter()
            .map(|item| {
                let mut new = item.signature.clone();
                let index = new.len() - 2;
                let num: u16 = u16::from_be_bytes(new[index..].try_into()?) + 27;
                new.splice((new.len() - 2).., num.to_be_bytes());
                Ok(new)
            })
            .collect::<color_eyre::Result<Vec<Vec<u8>>>>()?;

        let mr_slice: [u8; 32] = event
            .commitment_message_root
            .try_into()
            .map_err(|e| BridgerError::Custom(format!("Wrong message root: {:?}", e)))?;
        let commitment = Commitment {
            block_number: event.commitment_block_number,
            message_root: H256(mr_slice),
            nonce: event.commitment_nonce.into(),
        };
        let hash = client_posa
            .import_message_commitment(commitment, signatures, &ethereum_account.secret_key()?)
            .await?;
        tracing::info!(
            target: "pangoro-goerli",
            "[pangoro] [ecdsa] submitted collected enouth new message root signature: {}",
            array_bytes::bytes2hex("0x", &hash.0),
        );
        Ok(Some(event.block_number))
    }
}
