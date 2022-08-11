use client_contracts::{
    chain_message_committer::types::MessageProof,
    error::BridgeContractError,
    inbound_types::{Message, OutboundLaneData, Payload},
    outbound::MessageAccepted,
    ChainMessageCommitter, Inbound, LaneMessageCommitter, Outbound,
};
use futures::future;
use support_common::error::BridgerError;
use web3::{
    ethabi::RawLog,
    transports::Http,
    types::{BlockId, BlockNumber, FilterBuilder, H256, U256},
    Web3,
};

pub async fn build_darwinia_delivery_proof(
    outbound: &Outbound,
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    block_id: Option<BlockId>,
) -> color_eyre::Result<MessageProof> {
    let (_, lane_pos, _, _) = outbound.get_lane_info().await?;
    Ok(build_darwinia_proof(
        lane_message_committer,
        chain_message_committer,
        lane_pos,
        block_id,
    )
    .await?)
}

pub async fn build_darwinia_confirm_proof(
    inbound: &Inbound,
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    block_id: Option<BlockId>,
) -> color_eyre::Result<MessageProof> {
    let (_, lane_pos, _, _) = inbound.get_lane_info().await?;
    Ok(build_darwinia_proof(
        lane_message_committer,
        chain_message_committer,
        lane_pos,
        block_id,
    )
    .await?)
}

async fn build_darwinia_proof(
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    lane_pos: u32,
    block_id: Option<BlockId>,
) -> color_eyre::Result<MessageProof> {
    let bridged_chain_pos = lane_message_committer.bridged_chain_position().await?;
    Ok(chain_message_committer
        .prove(bridged_chain_pos, U256::from(lane_pos), block_id)
        .await?)
}

pub async fn build_messages_data(
    client: &Web3<Http>,
    outbound: &Outbound,
    begin: u64,
    end: u64,
) -> color_eyre::Result<OutboundLaneData> {
    let outbound_data = outbound.data().await?;
    let outbound_lane_nonce = outbound.outbound_lane_nonce().await?;
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

    let accepted_events = query_message_accepted_events(client, outbound, begin, end).await?;
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
