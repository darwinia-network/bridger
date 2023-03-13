use bridge_e2e_traits::client::EcdsaClient;
use client_contracts::posa_light_client::Commitment;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use web3::types::H256;

use crate::error::{RelayError, RelayResult};

use super::types::EcdsaSource;

pub struct CollectedEnoughNewMessageRootSignaturesRunner<T: EcdsaClient> {
    source: EcdsaSource<T>,
    interval: u64,
    last_relay_time: u64,
}

impl<T: EcdsaClient> CollectedEnoughNewMessageRootSignaturesRunner<T> {
    pub fn new(source: EcdsaSource<T>, interval: u64) -> Self {
        Self {
            source,
            interval,
            last_relay_time: u64::MIN,
        }
    }
}

impl<T: EcdsaClient> CollectedEnoughNewMessageRootSignaturesRunner<T> {
    pub async fn start(&mut self) -> RelayResult<Option<u32>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RelayError::Custom(format!("{}", e)))?
            .as_secs();
        if now - self.last_relay_time <= self.interval {
            tracing::info!(
                target: "relay-e2e",
                "[Darwinia][ECDSA][collectedMessages] Last relaying time is {:?}, wait for {} seconds to scan new message root",
                self.last_relay_time,
                self.interval - (now - self.last_relay_time)
            );
            return Ok(None);
        }

        let client_posa = &self.source.client_posa;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let ethereum_account = &self.source.ethereum_account;

        let cacse = subquery
            .next_collected_enough_new_message_root_signatures_event(from_block)
            .await?;

        if cacse.is_none() {
            tracing::debug!(
                target: "relay-e2e",
                "[Darwinia][ECDSA][collectedMessages] no more events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");
        let latest_relayed_block_number = self.source.client_posa.block_number().await?;
        if latest_relayed_block_number.as_u32() >= event.commitment_block_number {
            tracing::info!(
                target: "relay-e2e",
                "[Darwinia][ECDSA][collectedMessages] Latest relayed block number is: {:?}",
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

        let mr_slice: [u8; 32] = event
            .commitment_message_root
            .try_into()
            .map_err(|e| RelayError::Custom(format!("Wrong message root: {:?}", e)))?;
        let commitment = Commitment {
            block_number: event.commitment_block_number,
            message_root: H256(mr_slice),
            nonce: event.commitment_nonce.into(),
        };
        dbg!(&commitment);
        dbg!(&signatures);
        let nonce = client_posa.nonce().await?;
        dbg!(nonce);
        let block_number = client_posa.block_number().await?;
        dbg!(block_number);
        let hash = client_posa
            .import_message_commitment(commitment, signatures, &ethereum_account.secret_key()?)
            .await?;
        tracing::info!(
            target: "relay-e2e",
            "[Darwinia][ECDSA][collectedMessages] submitted collected enouth new message root signature: {}",
            array_bytes::bytes2hex("0x", &hash.0),
        );
        support_etherscan::wait_for_transaction_confirmation(
            hash,
            self.source.client_eth_web3.transport(),
            Duration::from_secs(5),
            3,
        )
        .await?;
        self.last_relay_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RelayError::Custom(format!("{}", e)))?
            .as_secs();
        Ok(Some(event.block_number))
    }
}
