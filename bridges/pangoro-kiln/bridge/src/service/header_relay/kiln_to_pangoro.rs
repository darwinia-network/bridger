use std::{
    ops::{Add, Div},
    str::FromStr,
};

use crate::{
    bridge::{BridgeConfig, PangoroKilnBus},
    kiln_client::{client::KilnClient, types::Proof},
    pangoro_client::client::PangoroClient,
};
use lifeline::{Lifeline, Service, Task};
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use web3::{
    contract::{
        tokens::{Tokenizable, Tokenize},
        Options,
    },
    ethabi::Token,
    types::{H256, U256},
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
        let _greet = Self::try_task("header-kiln-to-pangoro", async move {
            while let Err(error) = start().await {
                tracing::error!(
                    target: "pangoro-kiln",
                    "Failed to start kiln-to-pangoro header relay service, restart after some seconds: {:?}",
                    error
                );
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    let config: BridgeConfig = Config::restore(Names::BridgePangoroKiln)?;
    println!("{:?}", &config);
    let pangoro_client = PangoroClient::new(
        &config.pangoro.endpoint,
        &config.pangoro.contract_address,
        &config.pangoro.execution_layer_contract_address,
        &config.pangoro.private_key,
    )?;
    let kiln_client = KilnClient::new(&config.kiln.endpoint)?;
    let header_relay = HeaderRelay {
        pangoro_client,
        kiln_client,
    };

    loop {
        if let Err(error) = header_relay.header_relay().await {
            tracing::error!(
                target: "pangoro-kiln",
                "Failed relay header : {:?}",
                error
            );
            return Err(error);
        }
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

pub struct HeaderRelay {
    pub pangoro_client: PangoroClient,
    pub kiln_client: KilnClient,
}

impl HeaderRelay {
    pub async fn header_relay(&self) -> color_eyre::Result<()> {
        let kiln_current_head = self.kiln_client.get_header("head").await?;
        let kiln_current_slot = kiln_current_head.header.message.slot.parse::<u64>()?;
        let old_finalized_header = self.pangoro_client.finalized_header().await?;
        tracing::info!(
            target: "pangoro-kiln",
            "[Header][Kiln => Pangoro] Latest kiln header on pangoro: {:?}, And current kiln header: {:?}",
            &old_finalized_header.slot,
            kiln_current_slot,
        );
        let _old_period = old_finalized_header.slot.div(32).div(256);

        let attested_header_slot = old_finalized_header.slot.add(96);
        if attested_header_slot >= kiln_current_slot {
            tracing::info!(
                target: "pangoro-kiln",
                "[Header][Kiln => Pangoro] Current slot is {:?}, wait for finality",
                &kiln_current_slot,
            );
            return Ok(());
        }

        let (slot, sync_aggregate_slot, attested_header, sync_aggregate_block, finalized_header) =
            match self
                .kiln_client
                .find_valid_attested_header(
                    kiln_current_slot,
                    attested_header_slot,
                    old_finalized_header.slot,
                )
                .await?
            {
                None => {
                    tracing::info!(
                        target: "pangoro-kiln",
                        "[Header][Kiln => Pangoro] Wait for valid attested header",
                    );
                    return Ok(());
                }
                Some((
                    slot,
                    sync_aggregate_slot,
                    attested_header,
                    sync_aggregate_block,
                    finalized_header,
                )) => (
                    slot,
                    sync_aggregate_slot,
                    attested_header,
                    sync_aggregate_block,
                    finalized_header,
                ),
            };

        tracing::info!(
            target: "pangoro-kiln",
            "[Header][Kiln => Pangoro] Next attested header: {:?}, sync aggregate header: {:?}",
            slot,
            sync_aggregate_slot,
        );

        let new_period = sync_aggregate_slot.div(32).div(256);
        let snapshot = self
            .kiln_client
            .find_valid_snapshot_in_period(new_period)
            .await?;
        let current_sync_committee = snapshot.current_sync_committee;

        let sync_aggregate = sync_aggregate_block.body.sync_aggregate;
        let mut sync_aggregate_bits: Vec<Token> = Vec::new();

        let bits = sync_aggregate.sync_committee_bits;
        sync_aggregate_bits.push(H256::from_str(&bits[..66])?.into_token());
        sync_aggregate_bits.push(H256::from_str(&bits[66..])?.into_token());

        tracing::info!(
            target: "pangoro-kiln",
            "[Header][Kiln => Pangoro] Finalized header to relay: {:?}",
            &finalized_header.header.message.slot,
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
        let attested_header = Token::Tuple(
            (
                header_message.slot.parse::<u64>()?,
                header_message.proposer_index.parse::<u64>()?,
                H256::from_str(&header_message.parent_root)?,
                H256::from_str(&header_message.state_root)?,
                H256::from_str(&header_message.body_root)?,
            )
                .into_tokens(),
        );
        let signature_sync_committee = Token::Tuple(
            (
                Token::FixedArray(
                    current_sync_committee
                        .pubkeys
                        .iter()
                        .map(|s| hex::decode(&s.clone()[2..]))
                        .collect::<Result<Vec<Vec<u8>>, _>>()?
                        .iter()
                        .map(|s| Token::Bytes(s.to_vec()))
                        .collect::<Vec<Token>>(),
                ),
                hex::decode(&current_sync_committee.aggregate_pubkey.clone()[2..])?,
            )
                .into_tokens(),
        );

        let message = finalized_header.header.message;
        let finalized_header = Token::Tuple(
            (
                message.slot.parse::<u64>()?,
                message.proposer_index.parse::<u64>()?,
                H256::from_str(&message.parent_root)?,
                H256::from_str(&message.state_root)?,
                H256::from_str(&message.body_root)?,
            )
                .into_tokens(),
        );

        let sync_aggregate = Token::Tuple(
            (
                Token::FixedArray(sync_aggregate_bits),
                hex::decode(&sync_aggregate.sync_committee_signature.clone()[2..])?,
            )
                .into_tokens(),
        );
        let parameter = Token::Tuple(
            (
                attested_header,
                signature_sync_committee,
                finalized_header,
                witnesses,
                sync_aggregate,
                Token::FixedBytes(fork_version.current_version.as_bytes().to_vec()),
                sync_aggregate_slot,
            )
                .into_tokens(),
        );

        let tx = self
            .pangoro_client
            .contract
            .signed_call(
                "import_finalized_header",
                (parameter,),
                Options {
                    gas: Some(U256::from(10000000)),
                    gas_price: Some(U256::from(1300000000)),
                    ..Default::default()
                },
                &self.pangoro_client.private_key,
            )
            .await?;

        tracing::info!(
            target: "pangoro-kiln",
            "[Header][Kiln => Pangoro] Sending tx: {:?}",
            &tx
        );
        Ok(())
    }
}
