use bytes::{Buf, Bytes};
use client_contracts::beacon_light_client_types::ExecutionPayloadHeader as ContractExecutionPayloadHeader;
use client_contracts::beacon_light_client_types::HeaderMessage as ContractHeaderMessage;
use client_contracts::beacon_light_client_types::LightClientHeader;
use client_contracts::beacon_light_client_types::SyncAggregate as ContractSyncAggregate;
use client_contracts::beacon_light_client_types::SyncCommittee as ContractSyncCommittee;
use serde::{Deserialize, Serialize};
use types::ExecutionPayloadHeaderCapella;
use std::fmt::Display;
use std::str::FromStr;
use tree_hash::TreeHash;
use types::BeaconBlock;
use types::MainnetEthSpec;
use web3::types::H160;
use web3::types::U256;
use web3::{
    contract::tokens::{Tokenizable, Tokenize},
    ethabi::{ethereum_types::H32, Token},
    types::{Bytes as Web3Bytes, H256},
};

use serde::de::{self, Deserializer};

use crate::error::BeaconApiError;
use crate::error::BeaconApiResult;

fn h256_from_str(value: &str) -> BeaconApiResult<H256> {
    H256::from_str(value).or(Err(BeaconApiError::DecodeError(
        value.into(),
        "H256".into(),
    )))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "statusCode")]
    pub status_code: u64,
    pub error: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseWrapper<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetHeaderResponse {
    pub root: String,
    pub canonical: bool,
    pub header: Header,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Header {
    pub message: HeaderMessage,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BeaconHeaderMessage {
    pub beacon: HeaderMessage,
    pub execution: ExecutionPayloadHeaderCapella<MainnetEthSpec>,
    pub execution_branch: Vec<String>,
}

impl BeaconHeaderMessage {
    pub fn to_contract_type(&self) -> BeaconApiResult<LightClientHeader> {
        Ok(LightClientHeader {
            beacon: self.beacon.to_contract_type()?,
            execution: ContractExecutionPayloadHeader {
                parent_hash: H256::from(self.execution.parent_hash.into_root().0),
                fee_recipient: H160::from(self.execution.fee_recipient.0),
                state_root: H256::from(self.execution.state_root.0),
                receipts_root: H256::from(self.execution.receipts_root.0),
                logs_bloom: H256::from(self.execution.logs_bloom.tree_hash_root().0),
                prev_randao: H256::from(self.execution.prev_randao.0),
                block_number: self.execution.block_number,
                gas_limit: self.execution.gas_limit,
                gas_used: self.execution.gas_used,
                timestamp: self.execution.timestamp,
                extra_data: H256::from(self.execution.extra_data.tree_hash_root().0),
                base_fee_per_gas: U256::from(self.execution.base_fee_per_gas.as_u128()),
                block_hash: H256::from(self.execution.block_hash.into_root().0),
                transactions_root: H256::from(self.execution.transactions_root.0),
                withdrawals_root: H256::from(self.execution.withdrawals_root.0),
            },
            execution_branch: self
                .execution_branch
                .iter()
                .map(|x| h256_from_str(&x.to_string()))
                .collect::<Result<Vec<H256>, BeaconApiError>>()?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeaderMessage {
    #[serde(deserialize_with = "from_str")]
    pub slot: u64,
    #[serde(deserialize_with = "from_str")]
    pub proposer_index: u64,
    pub parent_root: String,
    pub state_root: String,
    pub body_root: String,
}

impl HeaderMessage {
    pub fn to_contract_type(&self) -> BeaconApiResult<ContractHeaderMessage> {
        Ok(ContractHeaderMessage {
            slot: self.slot,
            proposer_index: self.proposer_index,
            parent_root: h256_from_str(&self.parent_root)?,
            state_root: h256_from_str(&self.state_root)?,
            body_root: h256_from_str(&self.body_root)?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Snapshot {
    pub header: HeaderMessage,
    pub current_sync_committee: SyncCommittee,
    pub current_sync_committee_branch: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncCommittee {
    pub pubkeys: Vec<String>,
    pub aggregate_pubkey: String,
}

impl SyncCommittee {
    pub fn to_contract_type(&self) -> BeaconApiResult<ContractSyncCommittee> {
        Ok(ContractSyncCommittee {
            pubkeys: self
                .pubkeys
                .iter()
                .map(|x| hex::decode(&x.clone()[2..]))
                .collect::<Result<Vec<Vec<u8>>, _>>()?,
            aggregate_pubkey: Web3Bytes(hex::decode(&self.aggregate_pubkey.clone()[2..])?),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncCommitteePeriodUpdate {
    pub attested_header: BeaconHeaderMessage,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: Vec<String>,
    pub finalized_header: BeaconHeaderMessage,
    pub finality_branch: Vec<String>,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: String,
    // pub fork_version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBlockResponse {
    pub message: BlockMessage,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BeaconBlockWrapper {
    pub message: BeaconBlock<MainnetEthSpec>,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockMessage {
    pub slot: String,
    pub proposer_index: String,
    pub parent_root: String,
    pub state_root: String,
    pub body: BlockBody,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockBody {
    pub attestations: Vec<Attestation>,
    pub sync_aggregate: SyncAggregate,
    pub execution_payload: ExecutionPayload,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attestation {
    pub aggregation_bits: String,
    pub data: AttestationData,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttestationData {
    pub slot: String,
    pub index: String,
    pub beacon_block_root: String,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Checkpoint {
    pub epoch: String,
    pub root: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncAggregate {
    pub sync_committee_bits: String,
    pub sync_committee_signature: String,
}

impl SyncAggregate {
    pub fn to_contract_type(&self) -> BeaconApiResult<ContractSyncAggregate> {
        let mut sync_committee_bits: [H256; 2] = [H256::default(); 2];
        sync_committee_bits[0] = h256_from_str(&self.sync_committee_bits[..66])?;
        sync_committee_bits[1] = h256_from_str(&self.sync_committee_bits[66..])?;

        let sync_committee_signature =
            Web3Bytes(hex::decode(&self.sync_committee_signature.clone()[2..])?);
        Ok(ContractSyncAggregate {
            sync_committee_bits,
            sync_committee_signature,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPayload {
    pub parent_hash: String,
    pub fee_recipient: String,
    pub state_root: String,
    pub receipts_root: String,
    pub logs_bloom: String,
    pub prev_randao: String,
    pub block_number: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub timestamp: String,
    pub extra_data: String,
    pub base_fee_per_gas: String,
    pub block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finality {
    pub previous_justified: Checkpoint,
    pub current_justified: Checkpoint,
    pub finalized: Checkpoint,
}

#[derive(Debug)]
pub enum Proof {
    SingleProof {
        gindex: u16,
        leaf: H256,
        witnesses: Vec<H256>,
    },
    #[allow(dead_code)]
    TreeOffsets {
        offsets: Vec<u16>,
        leaves: Vec<H256>,
    },
}

impl TryFrom<Bytes> for Proof {
    type Error = BeaconApiError;

    fn try_from(mut x: Bytes) -> Result<Self, Self::Error> {
        match x.get_u8() {
            0u8 => Ok(Proof::SingleProof {
                gindex: x.get_u16_le(),
                leaf: {
                    let mut leaf = [0u8; 32];
                    x.copy_to_slice(&mut leaf);
                    H256::from(leaf)
                },
                witnesses: {
                    let witcount: usize = x.get_u16_le().into();
                    (0..witcount)
                        .map(|_| {
                            let mut witness = [0u8; 32];
                            x.copy_to_slice(&mut witness);
                            H256::from(witness)
                        })
                        .collect()
                },
            }),
            _ => Err(BeaconApiError::Custom("Unimplemented!".into())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkVersion {
    pub previous_version: H32,
    pub current_version: H32,
    pub epoch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalityUpdate {
    pub attested_header: BeaconHeaderMessage,
    pub finalized_header: BeaconHeaderMessage,
    pub finality_branch: Vec<String>,
    pub sync_aggregate: SyncAggregate,
}

#[derive(Debug, Clone)]
pub struct MessagesProof {
    pub account_proof: Vec<web3::types::Bytes>,
    pub lane_nonce_proof: Vec<web3::types::Bytes>,
    pub lane_messages_proof: Vec<Vec<web3::types::Bytes>>,
}

impl MessagesProof {
    pub fn get_token(&self) -> BeaconApiResult<Token> {
        Ok(Token::Tuple(
            (
                bytes_vec_to_token(self.account_proof.clone()),
                bytes_vec_to_token(self.lane_nonce_proof.clone()),
                Token::Array(
                    self.lane_messages_proof
                        .clone()
                        .into_iter()
                        .map(|x| bytes_vec_to_token(x))
                        .collect::<Vec<Token>>(),
                ),
            )
                .into_tokens(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct MessagesConfirmationProof {
    pub account_proof: Vec<web3::types::Bytes>,
    pub lane_nonce_proof: Vec<web3::types::Bytes>,
    pub lane_relayers_proof: Vec<Vec<web3::types::Bytes>>,
}

impl MessagesConfirmationProof {
    pub fn get_token(&self) -> BeaconApiResult<Token> {
        Ok(Token::Tuple(
            (
                bytes_vec_to_token(self.account_proof.clone()),
                bytes_vec_to_token(self.lane_nonce_proof.clone()),
                Token::Array(
                    self.lane_relayers_proof
                        .clone()
                        .into_iter()
                        .map(bytes_vec_to_token)
                        .collect::<Vec<Token>>(),
                ),
            )
                .into_tokens(),
        ))
    }
}

fn bytes_vec_to_token(bytes: Vec<web3::types::Bytes>) -> Token {
    Token::Array(
        bytes
            .into_iter()
            .map(|x| x.into_token())
            .collect::<Vec<Token>>(),
    )
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconBlockRoot {
    pub root: String,
}
