use crate::error::{RelayError, RelayResult};
use bridge_e2e_traits::client::EcdsaClient;
use std::time::Duration;
use subquery::types::AOperationType;
use web3::types::H160;

use super::types::EcdsaSource;

pub struct CollectedEnoughAuthoritiesChangeSignaturesRunner<T: EcdsaClient> {
    source: EcdsaSource<T>,
}

impl<T: EcdsaClient> CollectedEnoughAuthoritiesChangeSignaturesRunner<T> {
    pub fn new(source: EcdsaSource<T>) -> Self {
        Self { source }
    }
}

impl<T: EcdsaClient> CollectedEnoughAuthoritiesChangeSignaturesRunner<T> {
    pub async fn start(&self) -> RelayResult<Option<u32>> {
        let client_posa = &self.source.client_posa;
        let _client_darwinia_substrate = &self.source.client_darwinia_substrate;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let ethereum_account = &self.source.ethereum_account;

        let cacse = subquery
            .next_collected_enough_authorities_change_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "relay-e2e",
                "[Darwinia][ECDSA][collectedAuthorities] no more events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");

        let latest_relayed_block_number = self.source.client_posa.block_number().await?;
        if latest_relayed_block_number.as_u32() >= event.block_number {
            tracing::info!(
                target: "relay-e2e",
                "[Darwinia][ECDSA][collectedAuthorities] Latest relayed block number is: {:?}",
                event.block_number
            );
            return Ok(Some(event.block_number));
        }

        let mut signature_nodes = event.signatures.nodes;
        signature_nodes.sort_by(|a, b| a.address.cmp(&b.address));
        let signatures = signature_nodes
            .iter()
            .map(|item| {
                let mut new = item.signature.clone();
                let index = new.len() - 2;
                let num: u16 = u16::from_be_bytes(
                    new[index..]
                        .try_into()
                        .map_err(|e| RelayError::Custom(format!("{}", e)))?,
                ) + 27;
                new.splice((new.len() - 2).., num.to_be_bytes());
                Ok(new)
            })
            .collect::<RelayResult<Vec<Vec<u8>>>>()?;

        let threshold = event.threshold;

        let address_prev = event.operation_pre.map(H160);
        let address_new = event.operation_new.map(H160);
        let address_old = event.operation_old.map(H160);
        let hash = match event.operation_type {
            AOperationType::Add => {
                client_posa
                    .add_relayer(
                        address_new.ok_or_else(|| {
                            RelayError::Custom("not found new authority account".to_string())
                        })?,
                        threshold
                            .ok_or_else(|| {
                                RelayError::Custom("no threshold from event".to_string())
                            })?
                            .into(),
                        signatures,
                        &ethereum_account.secret_key()?,
                    )
                    .await?
            }
            AOperationType::Remove => {
                client_posa
                    .remove_relayer(
                        address_prev.ok_or_else(|| {
                            RelayError::Custom("not found previous authority account".to_string())
                        })?,
                        address_old.ok_or_else(|| {
                            RelayError::Custom("not found new authority account".to_string())
                        })?,
                        threshold
                            .ok_or_else(|| {
                                RelayError::Custom("no threshold from event".to_string())
                            })?
                            .into(),
                        signatures,
                        &ethereum_account.secret_key()?,
                    )
                    .await?
            }
            AOperationType::Swap => {
                client_posa
                    .swap_relayer(
                        address_prev.ok_or_else(|| {
                            RelayError::Custom("not found previous authority account".to_string())
                        })?,
                        address_old.ok_or_else(|| {
                            RelayError::Custom("not found old authority account".to_string())
                        })?,
                        address_new.ok_or_else(|| {
                            RelayError::Custom("not found new authority account".to_string())
                        })?,
                        signatures,
                        &ethereum_account.secret_key()?,
                    )
                    .await?
            }
        };

        tracing::info!(
            target: "relay-e2e",
            "[Darwinia][ECDSA][collectedAuthorities] authorities change submitted: {}",
            array_bytes::bytes2hex("0x", &hash.0),
        );
        support_etherscan::wait_for_transaction_confirmation(
            hash,
            self.source.client_eth_web3.transport(),
            Duration::from_secs(5),
            3,
        )
        .await?;

        Ok(Some(event.block_number))
    }
}
