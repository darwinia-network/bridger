//! Ethereum receipt
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

use codec::{Decode, Encode};
use component_shadow::types::{EthereumReceiptProofJson, EthereumReceiptProofThingJson};
use rlp::{Encodable, RlpStream};
use serde::{Deserialize, Serialize};

use support_primitives::array::{Bloom, H160, H256};

use crate::block::EthereumHeader;
use crate::error::BridgeEthereumError;
use crate::mmr::MMRProof;

/// Redeem for
#[derive(Clone, Debug, Encode, PartialEq, Eq)]
pub enum RedeemFor {
    /// Redeem for token
    Token,
    /// Redeem for deposit
    Deposit,
    /// Redeem for set authorities
    SetAuthorities,
    /// Redeem for register erc20 token
    RegisterErc20Token,
    /// Redeem for erc20 token
    RedeemErc20Token,
}

impl Default for RedeemFor {
    fn default() -> Self {
        RedeemFor::Token
    }
}

/// Ethereum Receipt Proof
#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, PartialEq, Eq)]
pub struct EthereumReceiptProof {
    /// Proof index
    pub index: u64,
    /// Receipt Proof
    pub proof: Vec<u8>,
    /// Ethereum Header Hash
    pub header_hash: [u8; 32],
}

impl TryFrom<EthereumReceiptProof> for EthereumReceiptProofJson {
    type Error = BridgeEthereumError;
    fn try_from(that: EthereumReceiptProof) -> Result<Self, Self::Error> {
        Ok(Self {
            index: format!("{:x}", that.index),
            proof: array_bytes::bytes2hex("", that.proof),
            header_hash: array_bytes::bytes2hex("", that.header_hash),
        })
    }
}

impl TryFrom<EthereumReceiptProofJson> for EthereumReceiptProof {
    type Error = BridgeEthereumError;
    fn try_from(that: EthereumReceiptProofJson) -> Result<Self, Self::Error> {
        let index = if that.index.starts_with("0x") {
            &that.index[2..]
        } else {
            "00"
        };

        let hash = if !that.header_hash.is_empty() {
            array_bytes::hex2array(that.header_hash.as_str())?
        } else {
            [0; 32]
        };

        Ok(Self {
            index: u64::from_str_radix(index, 16).unwrap_or(0),
            proof: array_bytes::hex2bytes(that.proof)?,
            header_hash: hash,
        })
    }
}

/// Ethereum ReceiptProofThing
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, Encode)]
pub struct EthereumReceiptProofThing {
    /// Ethereum Header
    pub header: EthereumHeader,
    /// Ethereum Receipt Proof
    pub receipt_proof: EthereumReceiptProof,
    /// MMR Proof
    pub mmr_proof: MMRProof,
}

impl TryFrom<EthereumReceiptProofThingJson> for EthereumReceiptProofThing {
    type Error = BridgeEthereumError;

    fn try_from(that: EthereumReceiptProofThingJson) -> Result<Self, Self::Error> {
        Ok(Self {
            header: that.header.try_into()?,
            receipt_proof: that.receipt_proof.try_into()?,
            mmr_proof: that.mmr_proof.try_into()?,
        })
    }
}

/// Ethereum receipt log entry
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct LogEntry {
    /// The address of the contract executing at the point of the `LOG` operation.
    pub address: H160,
    /// The topics associated with the `LOG` operation.
    pub topics: Vec<H256>,
    /// The data associated with the `LOG` operation.
    pub data: Vec<u8>,
}

/// Ethereum receipt transaction out come
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub enum TransactionOutcome {
    /// Status and state root are unknown under EIP-98 rules.
    Unknown,
    /// State root is known. Pre EIP-98 and EIP-658 rules.
    StateRoot(H256),
    /// Status code is known. EIP-658 rules.
    StatusCode(u8),
}

/// Ethereum Receipt
#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct EthereumReceipt {
    /// The total gas used in the block following execution of the transaction.
    pub gas_used: u64,
    /// The OR-wide combination of all logs' blooms for this transaction.
    pub log_bloom: Bloom,
    /// The logs stemming from this transaction.
    pub logs: Vec<LogEntry>,
    /// Transaction outcome.
    pub outcome: TransactionOutcome,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogJson {
    address: String,
    topics: Vec<String>,
    data: String,
}

/// Ethereum rsp response body
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthReceiptBody {
    /// the block hash
    pub block_hash: String,
    block_number: String,
    cumulative_gas_used: String,
    from: String,
    gas_used: String,
    logs: Vec<LogJson>,
    logs_bloom: String,
    #[serde(alias = "root")]
    status: String,
    to: String,
    transaction_hash: String,
    /// the transaction index
    pub transaction_index: String,
}

impl TryFrom<EthReceiptBody> for EthereumReceipt {
    type Error = BridgeEthereumError;
    fn try_from(that: EthReceiptBody) -> Result<Self, Self::Error> {
        Ok(Self {
            gas_used: u64::from_str_radix(&that.cumulative_gas_used.as_str()[2..], 16)
                .unwrap_or_default(),
            log_bloom: Bloom(array_bytes::hex2array(that.logs_bloom)?), // 256
            logs: {
                let mut rets = Vec::with_capacity(that.logs.len());
                for item in that.logs {
                    let mut topics = Vec::with_capacity(item.topics.len());
                    for topic in item.topics {
                        let bytes = H256(array_bytes::hex2array(topic)?);
                        topics.push(bytes);
                    }
                    let entry = LogEntry {
                        address: H160(array_bytes::hex2array(item.address)?), // 20
                        topics,
                        data: array_bytes::hex2bytes(item.data)?,
                    };
                    rets.push(entry);
                }
                rets
            },
            outcome: {
                if that.status.len() == 66 {
                    TransactionOutcome::StateRoot(H256(array_bytes::hex2array(that.status)?))
                // , 32
                } else {
                    TransactionOutcome::StatusCode(
                        u8::from_str_radix(&that.status.as_str()[2..], 16).unwrap_or(0),
                    )
                }
            },
        })
    }
}

impl Encodable for LogEntry {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append_list(&self.data);
    }
}

impl Encodable for EthereumReceipt {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self.outcome {
            TransactionOutcome::Unknown => {
                s.begin_list(3);
            }
            TransactionOutcome::StateRoot(ref root) => {
                s.begin_list(4);
                s.append(root);
            }
            TransactionOutcome::StatusCode(ref status_code) => {
                s.begin_list(4);
                s.append(status_code);
            }
        }
        s.append(&self.gas_used);
        s.append(&self.log_bloom);
        s.append_list(&self.logs);
    }
}
