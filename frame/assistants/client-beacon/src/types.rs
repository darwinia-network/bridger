use bytes::{Buf, Bytes};
use client_contracts::beacon_light_client_types::HeaderMessage as ContractHeaderMessage;
use client_contracts::beacon_light_client_types::SyncAggregate as ContractSyncAggregate;
use client_contracts::beacon_light_client_types::SyncCommittee as ContractSyncCommittee;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
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
    pub attested_header: HeaderMessage,
    pub next_sync_committee: SyncCommittee,
    pub finalized_header: HeaderMessage,
    pub finality_branch: Vec<String>,
    pub sync_aggregate: SyncAggregate,
    pub fork_version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBlockResponse {
    pub message: BlockMessage,
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
    pub attested_header: HeaderMessage,
    pub finalized_header: HeaderMessage,
    pub finality_branch: Vec<String>,
    pub sync_aggregate: SyncAggregate,
}

#[derive(Debug, Clone)]
pub struct MessagesProof {
    pub account_proof: web3::types::Bytes,
    pub lane_id_proof: web3::types::Bytes,
    pub lane_nonce_proof: web3::types::Bytes,
    pub lane_messages_proof: Vec<web3::types::Bytes>,
}

impl MessagesProof {
    pub fn get_token(&self) -> BeaconApiResult<Token> {
        Ok(Token::Tuple(
            (
                self.account_proof.clone(),
                self.lane_id_proof.clone(),
                self.lane_nonce_proof.clone(),
                Token::Array(
                    self.lane_messages_proof
                        .iter()
                        .map(|x| x.clone().into_token())
                        .collect::<Vec<Token>>(),
                ),
            )
                .into_tokens(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct MessagesConfirmationProof {
    pub account_proof: web3::types::Bytes,
    pub lane_nonce_proof: web3::types::Bytes,
    pub lane_relayers_proof: Vec<web3::types::Bytes>,
}

impl MessagesConfirmationProof {
    pub fn get_token(&self) -> BeaconApiResult<Token> {
        Ok(Token::Tuple(
            (
                self.account_proof.clone(),
                self.lane_nonce_proof.clone(),
                Token::Array(
                    self.lane_relayers_proof
                        .iter()
                        .map(|x| x.clone().into_token())
                        .collect::<Vec<Token>>(),
                ),
            )
                .into_tokens(),
        ))
    }
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
