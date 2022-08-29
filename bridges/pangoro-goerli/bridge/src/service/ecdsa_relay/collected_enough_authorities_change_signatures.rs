use web3::types::H160;

use subquery::types::AOperationType;
use support_common::error::BridgerError;

use crate::service::ecdsa_relay::types::EcdsaSource;

pub struct CollectedEnoughAuthoritiesChangeSignaturesRunner {
    source: EcdsaSource,
}

impl CollectedEnoughAuthoritiesChangeSignaturesRunner {
    pub fn new(source: EcdsaSource) -> Self {
        Self { source }
    }
}

impl CollectedEnoughAuthoritiesChangeSignaturesRunner {
    pub async fn start(&self) -> color_eyre::Result<Option<u32>> {
        let client_posa = &self.source.client_posa;
        let _client_pangoro_substrate = &self.source.client_pangoro_substrate;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let ethereum_account = &self.source.ethereum_account;

        let cacse = subquery
            .next_collected_enough_authorities_change_signatures_event(from_block)
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

        let threshold = event.threshold;

        let address_prev = event.operation_pre.map(H160);
        let address_new = event.operation_new.map(H160);
        let address_old = event.operation_old.map(H160);
        let hash = match event.operation_type {
            AOperationType::Add => {
                client_posa
                    .add_relayer(
                        address_new.ok_or_else(|| {
                            BridgerError::Custom("not found new authority account".to_string())
                        })?,
                        threshold
                            .ok_or_else(|| {
                                BridgerError::Custom("no threshold from event".to_string())
                            })?
                            .into(),
                        signatures,
                        ethereum_account.address()?,
                    )
                    .await?
            }
            AOperationType::Remove => {
                client_posa
                    .remove_relayer(
                        address_prev.ok_or_else(|| {
                            BridgerError::Custom("not found previous authority account".to_string())
                        })?,
                        address_new.ok_or_else(|| {
                            BridgerError::Custom("not found new authority account".to_string())
                        })?,
                        threshold
                            .ok_or_else(|| {
                                BridgerError::Custom("no threshold from event".to_string())
                            })?
                            .into(),
                        signatures,
                        ethereum_account.address()?,
                    )
                    .await?
            }
            AOperationType::Swap => {
                client_posa
                    .swap_relayer(
                        address_prev.ok_or_else(|| {
                            BridgerError::Custom("not found previous authority account".to_string())
                        })?,
                        address_old.ok_or_else(|| {
                            BridgerError::Custom("not found old authority account".to_string())
                        })?,
                        address_new.ok_or_else(|| {
                            BridgerError::Custom("not found new authority account".to_string())
                        })?,
                        signatures,
                        &ethereum_account.secret_key()?,
                    )
                    .await?
            }
        };

        tracing::info!(
            target: "pangoro-goerli",
            "[pangoro] [ecdsa] authorities change submitted: {}",
            array_bytes::bytes2hex("0x", &hash.0),
        );

        Ok(Some(event.block_number))
    }
}
