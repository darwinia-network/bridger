use std::str::FromStr;

use client_contracts::{
    error::BridgeContractError,
    inbound_types::{Message, OutboundLaneData, Payload},
    outbound::MessageAccepted,
    ChainMessageCommitter, Inbound, LaneMessageCommitter, Outbound,
};
use futures::future;
use support_common::error::BridgerError;
use thegraph_liketh::graph::TheGraphLikeEth;
use web3::{
    contract::tokens::Tokenizable,
    ethabi::{encode, RawLog},
    signing::keccak256,
    transports::Http,
    types::{Address, BlockId, BlockNumber, Bytes, FilterBuilder, Proof as Web3Proof, H256, U256},
    Web3,
};

use crate::ethereum_client::types::MessagesConfirmationProof;

use super::message_client::{LANE_IDENTIFY_SLOT, LANE_MESSAGE_SLOT, LANE_NONCE_SLOT};

pub async fn build_darwinia_delivery_proof(
    outbound: &Outbound,
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    block_id: Option<BlockId>,
) -> color_eyre::Result<Bytes> {
    let (_, lane_pos, _, _) = outbound.get_lane_info().await?;

    build_darwinia_proof(
        lane_message_committer,
        chain_message_committer,
        lane_pos,
        block_id,
    )
    .await
}

pub async fn build_darwinia_confirmation_proof(
    inbound: &Inbound,
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    block_id: Option<BlockId>,
) -> color_eyre::Result<Bytes> {
    let (_, lane_pos, _, _) = inbound.get_lane_info(block_id).await?;
    build_darwinia_proof(
        lane_message_committer,
        chain_message_committer,
        lane_pos,
        block_id,
    )
    .await
}

async fn build_darwinia_proof(
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    lane_pos: u32,
    block_id: Option<BlockId>,
) -> color_eyre::Result<Bytes> {
    let bridged_chain_pos = lane_message_committer.bridged_chain_position().await?;
    let proof = chain_message_committer
        .prove(bridged_chain_pos, U256::from(lane_pos), block_id)
        .await?
        .into_token();

    Ok(Bytes(encode(&[proof])))
}

pub async fn build_eth_confirmation_proof(
    client: &Web3<Http>,
    inbound: &Inbound,
    begin: u64,
    end: u64,
    block_number: Option<BlockNumber>,
) -> color_eyre::Result<Bytes> {
    let lane_id_proof = build_eth_proof(
        client,
        inbound.contract.address(),
        vec![U256::from(LANE_IDENTIFY_SLOT)],
        block_number,
    )
    .await?
    .ok_or_else(|| BridgerError::Custom("Failed to get lane_id_proof".into()))?;
    let lane_nonce_proof = build_eth_proof(
        client,
        inbound.contract.address(),
        vec![U256::from(LANE_NONCE_SLOT)],
        block_number,
    )
    .await?
    .ok_or_else(|| BridgerError::Custom("Failed to get lane_nonce_proof".into()))?;
    let relayer_keys = build_relayer_keys(begin, end)?;
    let lane_relayers_proof = build_eth_proof(
        client,
        inbound.contract.address(),
        relayer_keys,
        block_number,
    )
    .await?
    .ok_or_else(|| BridgerError::Custom("Failed to get lane_nonce_proof".into()))?;

    let proof = MessagesConfirmationProof {
        account_proof: encode_proof(&lane_id_proof.account_proof),
        lane_nonce_proof: encode_proof(&lane_nonce_proof.storage_proof[0].proof),
        lane_relayers_proof: lane_relayers_proof
            .storage_proof
            .iter()
            .map(|x| encode_proof(&x.proof))
            .collect(),
    };
    Ok(Bytes(encode(&[proof.get_token()?])))
}

async fn build_eth_proof(
    client: &Web3<Http>,
    address: Address,
    storage_keys: Vec<U256>,
    block_number: Option<BlockNumber>,
) -> color_eyre::Result<Option<Web3Proof>> {
    Ok(client
        .eth()
        .proof(address, storage_keys, block_number)
        .await?)
}

pub fn build_relayer_keys(begin: u64, end: u64) -> color_eyre::Result<Vec<U256>> {
    let mut result: Vec<U256> = Vec::new();
    for pos in begin..=end {
        let pos = U256::from(pos);
        let slot = U256::from(LANE_MESSAGE_SLOT);
        let bytes: &mut [u8] = &mut [0u8; 64];
        pos.to_big_endian(&mut bytes[..32]);
        slot.to_big_endian(&mut bytes[32..]);
        let key1 = U256::from(keccak256(bytes));
        let key2 = key1
            .checked_add(U256::from(1u64))
            .ok_or_else(|| BridgerError::Custom("Failed to build relayer keys".into()))?;
        result.push(key1);
        result.push(key2);
    }
    Ok(result)
}

pub fn encode_proof(proofs: &[Bytes]) -> Bytes {
    Bytes::from(
        &rlp::encode_list::<Vec<u8>, _>(
            proofs
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<Vec<u8>>>()
                .as_slice(),
        )[..],
    )
}

pub async fn build_messages_data(
    indexer: &TheGraphLikeEth,
    outbound: &Outbound,
    begin: u64,
    end: u64,
    at_block: Option<BlockNumber>,
) -> color_eyre::Result<OutboundLaneData> {
    let outbound_data = outbound.data(at_block.map(BlockId::from)).await?;
    let outbound_lane_nonce = outbound
        .outbound_lane_nonce(at_block.map(BlockId::from))
        .await?;
    let (outbound_begin, _outbound_end) = (
        outbound_lane_nonce.latest_received_nonce + 1,
        outbound_lane_nonce.latest_generated_nonce,
    );
    let messages = Vec::from_iter(
        outbound_data.messages[(begin - outbound_begin) as usize..=(end - outbound_begin) as usize]
            .iter()
            .cloned(),
    );

    if (end - begin + 1) as usize != messages.len() {
        return Err(BridgerError::Custom("Build messages data failed".into()).into());
    }

    let accepted_events = query_message_accepted_events_thegraph(indexer, begin, end).await?;
    let messages: Vec<Message> = std::iter::zip(messages, accepted_events)
        .into_iter()
        .map(|(message, event)| Message {
            encoded_key: message.encoded_key,
            payload: Payload {
                source: event.source,
                target: event.target,
                encoded: event.encoded,
            },
        })
        .collect();

    Ok(OutboundLaneData {
        latest_received_nonce: outbound_data.latest_received_nonce,
        messages,
    })
}

#[allow(dead_code)]
pub async fn query_message_accepted(
    client: &Web3<Http>,
    outbound: &Outbound,
    nonce: u64,
) -> color_eyre::Result<Option<MessageAccepted>> {
    let event = outbound.contract.abi().event("MessageAccepted")?;
    let mut filter = FilterBuilder::default();
    filter = filter.from_block(BlockNumber::Earliest);
    filter = filter.address(vec![outbound.contract.address()]);
    filter = filter.topics(
        Some(vec![event.signature()]),
        Some(vec![H256::from_low_u64_be(nonce)]),
        None,
        None,
    );
    let logs = client.eth().logs(filter.build()).await?;

    let events: Vec<MessageAccepted> = logs
        .into_iter()
        .map(|l| {
            let row_log = RawLog {
                topics: l.topics.clone(),
                data: l.data.0.clone(),
            };
            let block_number = l
                .block_number
                .ok_or_else(|| BridgeContractError::Custom("Failed toget block number".into()))?
                .as_u64();
            MessageAccepted::from_log(event.parse_log(row_log)?, block_number)
        })
        .collect::<Result<Vec<MessageAccepted>, BridgeContractError>>()?;
    match events.as_slice() {
        [x] => Ok(Some(x.clone())),
        _ => Ok(None),
    }
}

#[allow(dead_code)]
pub async fn query_message_accepted_events(
    client: &Web3<Http>,
    outbound: &Outbound,
    begin: u64,
    end: u64,
) -> color_eyre::Result<Vec<MessageAccepted>> {
    let logs: Result<Vec<Option<MessageAccepted>>, _> = future::try_join_all(
        (begin..=end).map(|nonce| query_message_accepted(client, outbound, nonce)),
    )
    .await;
    if let Some(logs) = logs?.into_iter().collect::<Option<Vec<_>>>() {
        Ok(logs)
    } else {
        Err(BridgerError::Custom(format!(
            "Failed to get message events from {:?} to {:?}",
            begin, end
        ))
        .into())
    }
}

pub async fn query_message_accepted_thegraph(
    thegraph_client: &TheGraphLikeEth,
    nonce: u64,
) -> color_eyre::Result<Option<MessageAccepted>> {
    thegraph_client
        .query_message_accepted(nonce)
        .await?
        .map(|x| -> color_eyre::Result<MessageAccepted> {
            Ok(MessageAccepted {
                nonce: x.nonce,
                source: Address::from_str(&x.source)?,
                target: Address::from_str(&x.target)?,
                encoded: Bytes(hex::decode(&x.encoded[2..])?),
                block_number: x.block_number,
            })
        })
        .transpose()
}

pub async fn query_message_accepted_events_thegraph(
    client: &TheGraphLikeEth,
    begin: u64,
    end: u64,
) -> color_eyre::Result<Vec<MessageAccepted>> {
    let logs: Result<Vec<Option<MessageAccepted>>, _> = future::try_join_all(
        (begin..=end).map(|nonce| query_message_accepted_thegraph(client, nonce)),
    )
    .await;
    if let Some(logs) = logs?.into_iter().collect::<Option<Vec<_>>>() {
        Ok(logs)
    } else {
        Err(BridgerError::Custom(format!(
            "Failed to get message events from {:?} to {:?}",
            begin, end
        ))
        .into())
    }
}
