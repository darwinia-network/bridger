use std::{
    ops::{Add, Div},
    str::FromStr,
};

use crate::{
    bridge::PangoroKilnBus,
    kiln_client::{client::KilnClient, types::Proof},
    pangoro_client::client::PangoroClient,
};
use lifeline::{Lifeline, Service, Task};
use reqwest::header;
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use web3::{
    contract::{
        tokens::{Tokenizable, TokenizableItem, Tokenize},
        Options,
    },
    ethabi::{ethereum_types::H32, Token},
    types::{Bytes, H256},
};

#[derive(Debug)]
pub struct KilnToPangoroHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for KilnToPangoroHeaderRelayService {}

impl Service for KilnToPangoroHeaderRelayService {
    type Bus = PangoroKilnBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(&format!("header-kiln-to-pangoro"), async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start kiln-to-pangoro header relay service, restart after some seconds: {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    todo!()
}

pub struct HeaderRelay {
    pub pangoro_client: PangoroClient,
    pub kiln_client: KilnClient,
}

impl HeaderRelay {
    pub async fn header_relay(&self) -> color_eyre::Result<()> {
        tracing::info!(
            target: "pangoro-kiln",
            "Start to relay headers: Kiln => Pangoro",
        );
        let old_finalized_header = self.pangoro_client.finalized_header().await?;
        tracing::info!(
            target: "pangoro-kiln",
            "Lastest relayed header: {:?}",
            &old_finalized_header,
        );
        let old_finalized_header_root = self
            .kiln_client
            .get_beacon_block_root(old_finalized_header.slot)
            .await?;
        let snapshot = self
            .kiln_client
            .get_light_client_snapshot(&old_finalized_header_root)
            .await?;
        let current_sync_committee = snapshot.current_sync_committee;
        let _old_period = old_finalized_header.slot.div(32).div(256);

        let attested_header_slot = old_finalized_header.slot.add(96);
        let (slot, attested_header) = self
            .kiln_client
            .find_valid_header_since(attested_header_slot)
            .await?;
        tracing::info!(
            target: "pangoro-kiln",
            "Next attested header: {:?}, {:?}",
            &slot,
            &attested_header
        );
        let sync_aggregate_slot = slot.add(1);
        let (sync_aggregate_slot, _sync_aggregate_header) = self
            .kiln_client
            .find_valid_header_since(sync_aggregate_slot)
            .await?;
        tracing::info!(
            target: "pangoro-kiln",
            "Next sync aggregate header: {:?}, {:?}",
            &sync_aggregate_slot,
            &_sync_aggregate_header
        );
        let sync_aggregate_block = self
            .kiln_client
            .get_beacon_block(sync_aggregate_slot)
            .await?;
        let _new_period = sync_aggregate_slot.div(32).div(256);

        let sync_aggregate = sync_aggregate_block.body.sync_aggregate;
        let mut sync_aggregate_bits: Vec<H256> = Vec::new();

        let bits = sync_aggregate.sync_committee_bits;
        sync_aggregate_bits.push(H256::from_str(&bits[..66])?);
        sync_aggregate_bits.push(H256::from_str(&bits[66..])?);

        let checkpoint = self.kiln_client.get_checkpoint(slot).await?;
        let finalized_header_root = checkpoint.finalized.root;
        let finalized_header = self.kiln_client.get_header(finalized_header_root).await?;
        tracing::info!(
            target: "pangoro-kiln",
            "Finalized header to relay : {:?}",
            &finalized_header,
        );
        let finality_branch = self.kiln_client.get_finality_branch(slot).await?;
        let witnesses = match finality_branch {
            Proof::SingleProof {
                gindex: _,
                leaf: _,
                witnesses,
            } => witnesses,
            _ => return Err(BridgerError::Custom("Not implemented!".to_string()).into()),
        };
        let fork_version = self
            .kiln_client
            .get_fork_version(sync_aggregate_slot)
            .await?;

        let header_message = attested_header.header.message;
        let attested_header = (
            header_message.slot.parse::<u64>()?,
            header_message.proposer_index.parse::<u64>()?,
            H256::from_str(&header_message.parent_root)?,
            H256::from_str(&header_message.state_root)?,
            H256::from_str(&header_message.body_root)?,
        )
            .into_tokens();
        let signature_sync_committee = (
            current_sync_committee
                .pubkeys
                .iter()
                .map(|s| hex::decode(&s.clone()[2..]))
                .collect::<Result<Vec<Vec<u8>>, _>>()?,
            hex::decode(&current_sync_committee.aggregate_pubkey.clone()[2..])?,
        )
            .into_tokens();

        let message = finalized_header.header.message;
        let finalized_header = (
            message.slot,
            message.proposer_index,
            H256::from_str(&message.parent_root)?,
            H256::from_str(&message.state_root)?,
            H256::from_str(&message.body_root)?,
        )
            .into_tokens();

        let sync_aggregate = (
            sync_aggregate_bits,
            hex::decode(&sync_aggregate.sync_committee_signature.clone()[2..])?,
        )
            .into_tokens();
        let tx = self
            .pangoro_client
            .contract
            .signed_call(
                "import_finalized_header",
                (
                    attested_header,
                    signature_sync_committee,
                    finalized_header,
                    witnesses,
                    sync_aggregate,
                    fork_version.current_version.as_bytes().to_vec(),
                    sync_aggregate_slot,
                ),
                Options::default(),
                &self.pangoro_client.private_key,
            )
            .await?;

        tracing::info!(
            target: "pangoro-kiln",
            "Sending tx: {:?}",
            &tx
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> (KilnClient, PangoroClient) {
        (
            KilnClient::new("http://localhost:5052").unwrap(),
            PangoroClient::new(
                "https://pangoro-rpc.darwinia.network",
                "/Users/furoxr/Projects/bridger/frame/abstract/bridge-s2e/src/BeaconLightClient_abi.json",
                "0xedD0683d354b2d2c209Ac8c574ef88E85bdBEa70",
                "03454001267e888193ea585845b6634d8977f56040199a55ba3c8654776efed8"
            ).unwrap()
        )
    }

    #[tokio::test]
    async fn test_header_relay() {
        let (kiln_client, pangoro_client) = test_client();
        let header_relay_service = HeaderRelay {pangoro_client, kiln_client};
        let result = header_relay_service.header_relay().await;
    }
}
