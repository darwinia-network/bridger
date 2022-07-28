use std::str::FromStr;

use futures::future;
use secp256k1::SecretKey;
use support_common::error::BridgerError;
use web3::{
    ethabi::{encode, RawLog},
    signing::keccak256,
    transports::Http,
    types::{Address, BlockNumber, Bytes, FilterBuilder, Proof as Web3Proof, H256, U256},
    Web3,
};

use crate::message_contract::{
    inbound::{Inbound, Message, OutboundLaneData, Payload, ReceiveMessagesProof},
    outbound::{MessageAccepted, Outbound},
};

use crate::kiln_client::types::MessagesProof;

const LANE_IDENTIFY_SLOT: u64 = 0u64;
const LANE_NONCE_SLOT: u64 = 1u64;
const LANE_MESSAGE_SLOT: u64 = 2u64;

pub struct MessageClient {
    pub client: Web3<Http>,
    pub inbound: Inbound,
    pub outbound: Outbound,
    pub private_key: Option<SecretKey>,
}

impl MessageClient {
    pub fn new(
        endpoint: &str,
        inbound_address: &str,
        outbound_address: &str,
        private_key: Option<&str>,
    ) -> color_eyre::Result<Self> {
        let transport = Http::new(endpoint)?;
        let client = Web3::new(transport);
        let inbound = Inbound::new(&client, inbound_address)?;
        let outbound = Outbound::new(&client, outbound_address)?;
        let private_key = private_key.map(SecretKey::from_str).transpose()?;

        Ok(Self {
            client,
            inbound,
            outbound,
            private_key,
        })
    }

    pub fn private_key(&self) -> color_eyre::Result<SecretKey> {
        Ok(self
            .private_key
            .ok_or_else(|| BridgerError::Custom("Private key not found!".into()))?)
    }

    pub async fn prepare_for_messages_delivery(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> color_eyre::Result<ReceiveMessagesProof> {
        let outbound_lane_data = self.build_messages_data(begin, end).await?;
        let proof = self
            .build_messages_proof(begin, end, block_number)
            .await?
            .get_token()?;
        let messages_proof = Bytes(encode(&[proof]));
        Ok(ReceiveMessagesProof {
            outbound_lane_data,
            messages_proof,
        })
    }

    pub async fn build_messages_data(
        &self,
        begin: u64,
        end: u64,
    ) -> color_eyre::Result<OutboundLaneData> {
        let outbound_data = self.outbound.data().await?;
        let outbound_lane_nonce = self.outbound.outbound_lane_nonce().await?;
        let (outbound_begin, _outbound_end) = (
            outbound_lane_nonce.latest_received_nonce + 1,
            outbound_lane_nonce.latest_generated_nonce,
        );
        let messages = Vec::from_iter(
            outbound_data.messages
                [(begin - outbound_begin) as usize..=(end - outbound_begin) as usize]
                .iter()
                .cloned(),
        );

        if (end - begin + 1) as usize != messages.len() {
            return Err(BridgerError::Custom("Build messages data failed".into()).into());
        }

        let accepted_events = self.query_message_accepted_events(begin, end).await?;
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

    pub async fn query_message_accepted_events(
        &self,
        begin: u64,
        end: u64,
    ) -> color_eyre::Result<Vec<MessageAccepted>> {
        let logs: Result<Vec<Option<MessageAccepted>>, _> =
            future::try_join_all((begin..=end).map(|nonce| self.query_message_accepted(nonce)))
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

    pub async fn query_message_accepted(
        &self,
        nonce: u64,
    ) -> color_eyre::Result<Option<MessageAccepted>> {
        let event = self.outbound.contract.abi().event("MessageAccepted")?;
        let mut filter = FilterBuilder::default();
        filter = filter.from_block(BlockNumber::Earliest);
        filter = filter.address(vec![self.outbound.contract.address()]);
        filter = filter.topics(
            Some(vec![event.signature()]),
            Some(vec![H256::from_low_u64_be(nonce)]),
            None,
            None,
        );
        let logs = self.client.eth().logs(filter.build()).await?;

        let events: Vec<MessageAccepted> = logs
            .into_iter()
            .map(|l| {
                let row_log = RawLog {
                    topics: l.topics.clone(),
                    data: l.data.0.clone(),
                };
                let block_number = l
                    .block_number
                    .ok_or_else(|| BridgerError::Custom("Failed toget block number".into()))?
                    .as_u64();
                MessageAccepted::from_log(event.parse_log(row_log)?, block_number)
            })
            .collect::<color_eyre::Result<Vec<MessageAccepted>>>()?;
        match events.as_slice() {
            [x] => Ok(Some(x.clone())),
            _ => Ok(None),
        }
    }

    pub async fn build_messages_proof(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> color_eyre::Result<MessagesProof> {
        let lane_id_proof = self
            .get_storage_proof(
                self.outbound.contract.address(),
                vec![U256::from(LANE_IDENTIFY_SLOT)],
                block_number,
            )
            .await?
            .ok_or_else(|| BridgerError::Custom("Failed to get lane_id_proof".into()))?;
        let lane_nonce_proof = self
            .get_storage_proof(
                self.outbound.contract.address(),
                vec![U256::from(LANE_NONCE_SLOT)],
                block_number,
            )
            .await?
            .ok_or_else(|| BridgerError::Custom("Failed to get lane_nonce_proof".into()))?;
        let message_keys = Self::build_message_storage_keys(begin, end);
        let message_proof = self
            .get_storage_proof(self.outbound.contract.address(), message_keys, block_number)
            .await?
            .ok_or_else(|| BridgerError::Custom("Failed to get message_proof".into()))?;

        let account_proof = Self::encode_proof(&lane_id_proof.account_proof);
        let lane_id_proof = Self::encode_proof(&lane_id_proof.storage_proof[0].proof);
        let lane_nonce_proof = Self::encode_proof(&lane_nonce_proof.storage_proof[0].proof);
        let lane_messages_proof = message_proof
            .storage_proof
            .iter()
            .map(|x| Self::encode_proof(&x.proof))
            .collect::<Vec<Bytes>>();

        Ok(MessagesProof {
            account_proof,
            lane_id_proof,
            lane_nonce_proof,
            lane_messages_proof,
        })
    }

    fn encode_proof(proofs: &[Bytes]) -> Bytes {
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

    pub fn build_message_storage_keys(begin: u64, end: u64) -> Vec<U256> {
        (begin..=end)
            .map(|pos| {
                let pos = U256::from(pos);
                let slot = U256::from(LANE_MESSAGE_SLOT);
                let bytes: &mut [u8] = &mut [0u8; 64];
                pos.to_big_endian(&mut bytes[..32]);
                slot.to_big_endian(&mut bytes[32..]);
                U256::from(keccak256(bytes))
            })
            .collect()
    }

    pub async fn get_storage_proof(
        &self,
        address: Address,
        storage_keys: Vec<U256>,
        block_number: Option<BlockNumber>,
    ) -> color_eyre::Result<Option<Web3Proof>> {
        Ok(self
            .client
            .eth()
            .proof(address, storage_keys, block_number)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use secp256k1::SecretKey;
    use web3::types::U64;

    use super::*;

    fn test_client() -> MessageClient {
        MessageClient::new(
            "http://localhost:8545",
            "0x588abe3F7EE935137102C5e2B8042788935f4CB0",
            "0xee4f69fc69F2C203a0572e43375f68a6e9027998",
            None,
        )
        .unwrap()
    }

    fn test_pangoro_client() -> MessageClient {
        MessageClient::new(
            "https://pangoro-rpc.darwinia.network",
            "0x6229BD8Ae2A0f97b8a1CEa47f552D0B54B402207",
            "0xEe8CA1000c0310afF74BA0D71a99EC02650798E5",
            None,
        )
        .unwrap()
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_storage_proof() {
        let client = test_client();
        let (begin, end) = (1, 2);
        let message_keys = MessageClient::build_message_storage_keys(begin, end);
        println!("Message keys: {:?}", message_keys);
        let message_proof = client
            .get_storage_proof(client.outbound.contract.address(), message_keys, None)
            .await
            .unwrap()
            .ok_or_else(|| BridgerError::Custom("Failed to get message_proof".into()))
            .unwrap();
        println!("Proof: {:?}", message_proof);
    }

    #[tokio::test]
    async fn test_build_lane_data() {
        let client = test_client();
        let outbound_lane_nonce = client.outbound.outbound_lane_nonce().await.unwrap();
        let (begin, end) = (
            outbound_lane_nonce.latest_received_nonce + 1,
            outbound_lane_nonce.latest_generated_nonce,
        );
        let lane_data = client.build_messages_data(begin, end).await.unwrap();
        println!("Lane data: {:?}", lane_data);
    }

    #[tokio::test]
    async fn test_query_message_accepted_events() {
        let client = test_client();
        let logs = client.query_message_accepted_events(1, 2).await.unwrap();
        println!("Logs: {:?}", logs);
    }

    #[tokio::test]
    async fn test_query_message_event() {
        let client = test_client();
        let event = client.query_message_accepted(2).await.unwrap();
        println!("event: {:?}", event);
    }

    #[tokio::test]
    async fn test_receive_messages_proof() {
        let kiln_client = test_client();
        let pangoro_client = test_pangoro_client();
        let private_key = SecretKey::from_str("//Alice").unwrap();
        let proof = kiln_client
            .prepare_for_messages_delivery(1, 2, Some(BlockNumber::Number(U64::from(1580730u64))))
            .await
            .unwrap();
        println!("proof: {:?}", proof);
        let tx = pangoro_client
            .inbound
            .receive_messages_proof(proof, &private_key)
            .await
            .unwrap();
        println!("tx: {:?}", tx);

        let inbound_status = pangoro_client.inbound.data().await.unwrap();
        println!("pangoro inbound: {:?}", inbound_status);
    }
}
