use bytes::{Buf, Bytes};
use serde::{Deserialize, Serialize};
use web3::{ethabi::ethereum_types::H32, types::H256};

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
    pub slot: String,
    pub proposer_index: String,
    pub parent_root: String,
    pub state_root: String,
    pub body_root: String,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncCommitteePeriodUpdate {
    pub next_sync_committee: SyncCommittee,
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

#[derive(Debug, Clone)]
pub enum Proof {
    SingleProof {
        gindex: u16,
        leaf: H256,
        witnesses: Vec<H256>,
    },
    TreeOffsets {
        offsets: Vec<u16>,
        leaves: Vec<H256>,
    },
}

impl From<Bytes> for Proof {
    fn from(mut x: Bytes) -> Self {
        match x.get_u8() {
            0u8 => Proof::SingleProof {
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
            },
            _ => {
                unimplemented!();
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkVersion {
    pub previous_version: H32,
    pub current_version: H32,
    pub epoch: String,
}
