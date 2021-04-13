//! Ethereum receipt
use crate::{
	array::{Bloom, H160, H256},
	bytes,
	chain::ethereum::{EthereumHeader, EthereumHeaderJson, MMRProof, MMRProofJson},
	hex,
};
use codec::{Decode, Encode};
use rlp::{Encodable, RlpStream};
use serde::Deserialize;
use std::fmt::Debug;

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
#[derive(Clone, Debug, Default, Encode, PartialEq, Eq)]
pub struct EthereumReceiptProof {
	/// Proof index
	pub index: u64,
	/// Receipt Proof
	pub proof: Vec<u8>,
	/// Ethereum Header Hash
	pub header_hash: [u8; 32],
}

/// Ethereum Receipt Proof Json
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthereumReceiptProofJson {
	/// Proof index
	pub index: String,
	/// Receipt Proof
	pub proof: String,
	/// Ethereum Header Hash
	pub header_hash: String,
}

impl Into<EthereumReceiptProofJson> for EthereumReceiptProof {
	fn into(self) -> EthereumReceiptProofJson {
		EthereumReceiptProofJson {
			index: format!("{:x}", self.index),
			proof: hex!(self.proof),
			header_hash: hex!(self.header_hash.to_vec()),
		}
	}
}

impl Into<EthereumReceiptProof> for EthereumReceiptProofJson {
	fn into(self) -> EthereumReceiptProof {
		let index = if self.index.starts_with("0x") {
			&self.index[2..]
		} else {
			"00"
		};

		let hash = if !self.header_hash.is_empty() {
			bytes!(self.header_hash.as_str(), 32)
		} else {
			[0; 32]
		};

		EthereumReceiptProof {
			index: u64::from_str_radix(index, 16).unwrap_or(0),
			proof: bytes!(self.proof.as_str()),
			header_hash: hash,
		}
	}
}

/// Ethereum ReceiptProofThing
#[derive(Clone, Debug, Default, PartialEq, Eq, Encode)]
pub struct EthereumReceiptProofThing {
	/// Ethereum Header
	pub header: EthereumHeader,
	/// Ethereum Receipt Proof
	pub receipt_proof: EthereumReceiptProof,
	/// MMR Proof
	pub mmr_proof: MMRProof,
}

/// Ethereum ReceiptProofThing Json
#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumReceiptProofThingJson {
	/// Ethereum Header
	pub header: EthereumHeaderJson,
	/// Ethereum Receipt Proof
	pub receipt_proof: EthereumReceiptProofJson,
	/// MMR Proof
	pub mmr_proof: MMRProofJson,
}

impl Into<EthereumReceiptProofThing> for EthereumReceiptProofThingJson {
	fn into(self) -> EthereumReceiptProofThing {
		EthereumReceiptProofThing {
			header: self.header.into(),
			receipt_proof: self.receipt_proof.into(),
			mmr_proof: self.mmr_proof.into(),
		}
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

impl Into<EthereumReceipt> for EthReceiptBody {
	fn into(self) -> EthereumReceipt {
		EthereumReceipt {
			gas_used: u64::from_str_radix(&self.cumulative_gas_used.as_str()[2..], 16)
				.unwrap_or_default(),
			log_bloom: Bloom(bytes!(self.logs_bloom.as_str(), 256)),
			logs: self
				.logs
				.iter()
				.map(|l| -> LogEntry {
					LogEntry {
						address: H160(bytes!(l.address.as_str(), 20)),
						topics: l
							.topics
							.iter()
							.map(|t| H256(bytes!(t.as_str(), 32)))
							.collect(),
						data: bytes!(l.data.as_str()),
					}
				})
				.collect(),
			outcome: {
				if self.status.len() == 66 {
					TransactionOutcome::StateRoot(H256(bytes!(self.status.as_str(), 32)))
				} else {
					TransactionOutcome::StatusCode(
						u8::from_str_radix(&self.status.as_str()[2..], 16).unwrap_or(0),
					)
				}
			},
		}
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
