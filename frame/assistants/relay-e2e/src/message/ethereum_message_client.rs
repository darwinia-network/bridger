use bridge_e2e_traits::{
    client::{GasPriceOracle, MessageClient, Web3Client},
    error::{E2EClientError, E2EClientResult},
    strategy::RelayStrategy,
};
use client_beacon::types::{MessagesConfirmationProof, MessagesProof};
use client_contracts::{
    error::BridgeContractError,
    inbound_types::{Message, OutboundLaneData, Payload, ReceiveMessagesProof},
    outbound_types::{MessageAccepted, ReceiveMessagesDeliveryProof},
    Inbound, Outbound, PosaLightClient,
};
use secp256k1::SecretKey;
use support_etherscan::EtherscanClient;
use web3::{
    ethabi::{encode, RawLog},
    signing::keccak256,
    transports::Http,
    types::{Address, BlockId, BlockNumber, Bytes, FilterBuilder, Proof as Web3Proof, H256, U256},
    Web3,
};

pub const LANE_IDENTIFY_SLOT: u64 = 0u64;
pub const LANE_NONCE_SLOT: u64 = 1u64;
pub const LANE_MESSAGE_SLOT: u64 = 2u64;

pub struct EthMessageClient<T: RelayStrategy> {
    pub client: Web3<Http>,
    pub inbound: Inbound,
    pub outbound: Outbound,
    pub darwinia_light_client: PosaLightClient,
    pub strategy: T,
    pub private_key: SecretKey,
    pub max_gas_price: U256,
    pub etherscan_client: EtherscanClient,
}

impl<T: RelayStrategy> Web3Client for EthMessageClient<T> {
    fn get_web3(&self) -> &Web3<Http> {
        &self.client
    }
}

impl<T: RelayStrategy> GasPriceOracle for EthMessageClient<T> {
    fn get_etherscan_client(&self) -> Option<&EtherscanClient> {
        Some(&self.etherscan_client)
    }

    fn max_gas_price(&self) -> U256 {
        self.max_gas_price
    }
}

#[async_trait::async_trait]
impl<T: RelayStrategy> MessageClient for EthMessageClient<T> {
    fn inbound(&self) -> &Inbound {
        &self.inbound
    }

    fn outbound(&self) -> &Outbound {
        &self.outbound
    }

    fn private_key(&self) -> &SecretKey {
        &self.private_key
    }

    async fn decide(&mut self, encoded_key: U256) -> E2EClientResult<bool> {
        self.strategy.decide(encoded_key).await
    }

    async fn prepare_for_delivery(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<ReceiveMessagesProof> {
        let outbound_lane_data = self
            .build_messages_data(begin, end, block_number.map(BlockId::from))
            .await?;
        let proof = self
            .build_messages_proof(begin, end, block_number)
            .await?
            .get_token()
            .map_err(|e| E2EClientError::Custom(format!("{}", e)))?;
        let messages_proof = Bytes(encode(&[proof]));
        Ok(ReceiveMessagesProof {
            outbound_lane_data,
            messages_proof,
        })
    }

    fn delivery_gas_unit(&self) -> E2EClientResult<U256> {
        Ok(U256::from_dec_str("300000").map_err(|e| E2EClientError::Custom(format!("{}", e)))?)
    }

    async fn prepare_for_confirmation(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<ReceiveMessagesDeliveryProof> {
        let at_block = block_number.map(BlockId::Number);
        let inbound_lane_data = self.inbound.data(at_block).await?;
        let messages_proof = self
            .build_eth_confirmation_proof(begin, end, block_number)
            .await?;
        Ok(ReceiveMessagesDeliveryProof {
            inbound_lane_data,
            messages_proof,
        })
    }

    fn confirmation_gas_unit(&self) -> E2EClientResult<U256> {
        Ok(U256::from_dec_str("2000000").map_err(|e| E2EClientError::Custom(format!("{}", e)))?)
    }

    async fn latest_light_client_block_number(&self) -> E2EClientResult<Option<u64>> {
        Ok(Some(
            self.darwinia_light_client.block_number().await?.as_u64(),
        ))
    }
}

impl<T: RelayStrategy> EthMessageClient<T> {
    pub async fn build_messages_data(
        &self,
        begin: u64,
        end: u64,
        at_block: Option<BlockId>,
    ) -> E2EClientResult<OutboundLaneData> {
        let outbound_data = self.outbound.data(at_block).await?;
        let outbound_lane_nonce = self.outbound.outbound_lane_nonce(at_block).await?;
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
            return Err(E2EClientError::Custom("Build messages data failed".into()).into());
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

    pub async fn build_messages_proof(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<MessagesProof> {
        let lane_id_proof = self
            .get_storage_proof_with_retry(
                self.outbound.contract.address(),
                vec![U256::from(LANE_IDENTIFY_SLOT)],
                block_number,
            )
            .await?
            .ok_or_else(|| E2EClientError::Custom("Failed to get lane_id_proof".into()))?;
        let lane_nonce_proof = self
            .get_storage_proof_with_retry(
                self.outbound.contract.address(),
                vec![U256::from(LANE_NONCE_SLOT)],
                block_number,
            )
            .await?
            .ok_or_else(|| E2EClientError::Custom("Failed to get lane_nonce_proof".into()))?;
        let message_keys = Self::build_message_storage_keys(begin, end);
        let message_proof = self
            .get_storage_proof_with_retry(
                self.outbound.contract.address(),
                message_keys,
                block_number,
            )
            .await?
            .ok_or_else(|| E2EClientError::Custom("Failed to get message_proof".into()))?;

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

    pub async fn build_eth_confirmation_proof(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<Bytes> {
        let lane_id_proof = self
            .get_storage_proof_with_retry(
                self.inbound.contract.address(),
                vec![U256::from(LANE_IDENTIFY_SLOT)],
                block_number,
            )
            .await?
            .ok_or_else(|| E2EClientError::Custom("Failed to get lane_id_proof".into()))?;
        let lane_nonce_proof = self
            .get_storage_proof_with_retry(
                self.inbound.contract.address(),
                vec![U256::from(LANE_NONCE_SLOT)],
                block_number,
            )
            .await?
            .ok_or_else(|| E2EClientError::Custom("Failed to get lane_nonce_proof".into()))?;
        let relayer_keys = Self::build_relayer_keys(begin, end)?;
        let lane_relayers_proof = self
            .get_storage_proof_with_retry(
                self.inbound.contract.address(),
                relayer_keys,
                block_number,
            )
            .await?
            .ok_or_else(|| E2EClientError::Custom("Failed to get lane_nonce_proof".into()))?;

        let proof = MessagesConfirmationProof {
            account_proof: Self::encode_proof(&lane_id_proof.account_proof),
            lane_nonce_proof: Self::encode_proof(&lane_nonce_proof.storage_proof[0].proof),
            lane_relayers_proof: lane_relayers_proof
                .storage_proof
                .iter()
                .map(|x| Self::encode_proof(&x.proof))
                .collect(),
        };
        Ok(Bytes(encode(&[proof
            .get_token()
            .map_err(|e| E2EClientError::Custom(format!("{}", e)))?])))
    }

    pub fn build_relayer_keys(begin: u64, end: u64) -> E2EClientResult<Vec<U256>> {
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
                .ok_or_else(|| E2EClientError::Custom("Failed to build relayer keys".into()))?;
            result.push(key1);
            result.push(key2);
        }
        Ok(result)
    }

    pub async fn query_message_accepted_events(
        &self,
        begin: u64,
        end: u64,
    ) -> E2EClientResult<Vec<MessageAccepted>> {
        let logs: Result<Vec<Option<MessageAccepted>>, _> = futures::future::try_join_all(
            (begin..=end).map(|nonce| self.query_message_accepted_with_retry(nonce)),
        )
        .await;
        if let Some(logs) = logs?.into_iter().collect::<Option<Vec<_>>>() {
            Ok(logs)
        } else {
            Err(E2EClientError::Custom(format!(
                "Failed to get message events from {:?} to {:?}",
                begin, end
            ))
            .into())
        }
    }

    pub async fn query_message_accepted_with_retry(
        &self,
        nonce: u64,
    ) -> E2EClientResult<Option<MessageAccepted>> {
        let mut count = 0;
        loop {
            match self.query_message_accepted(nonce).await {
                Ok(v) => return Ok(v),
                Err(error) => {
                    if count > 3 {
                        return Err(error);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(count)).await;
                    count += 1;
                }
            }
        }
    }

    pub async fn query_message_accepted(
        &self,
        nonce: u64,
    ) -> E2EClientResult<Option<MessageAccepted>> {
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

    pub async fn get_storage_proof_with_retry(
        &self,
        address: Address,
        storage_keys: Vec<U256>,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<Option<Web3Proof>> {
        let mut count = 0;
        loop {
            match self
                .get_storage_proof(address, storage_keys.clone(), block_number)
                .await
            {
                Ok(v) => return Ok(v),
                Err(error) => {
                    if count > 3 {
                        return Err(error);
                    }
                    count += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(count)).await;
                }
            }
        }
    }

    pub async fn get_storage_proof(
        &self,
        address: Address,
        storage_keys: Vec<U256>,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<Option<Web3Proof>> {
        Ok(self
            .client
            .eth()
            .proof(address, storage_keys, block_number)
            .await?)
    }
}
