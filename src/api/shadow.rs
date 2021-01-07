//! Darwinia shadow API
use crate::error::Error;
use crate::{
	error::{BizError, Result},
	Config,
};
use anyhow::Context as AnyhowContext;
use primitives::{
	chain::ethereum::{
		EthereumReceiptProofThing, EthereumReceiptProofThingJson, EthereumRelayHeaderParcel,
		EthereumRelayProofs, EthereumRelayProofsJson, MMRRoot, MMRRootJson,
	},
	rpc::{EthereumRPC, RPC},
};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Serialize)]
struct Proposal {
	pub member: u64,
	pub target: u64,
	pub last_leaf: u64,
}

/// Parent mmr root result
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ParentMmrRootResult {
	Result(MMRRootJson),
	Error { error: String },
}

/// Proof result
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ProofResult {
	Result(EthereumRelayProofsJson),
	Error { error: String },
}

/// Shadow API
pub struct Shadow {
	/// Shadow API
	pub api: String,
	/// Ethereum RPC
	pub eth: EthereumRPC,
	/// HTTP Client
	pub http: Client,
}

impl Shadow {
	/// Init Shadow API from config
	pub fn new(config: &Config) -> Shadow {
		let http = reqwest::Client::builder()
			.timeout(Duration::from_secs(30))
			.build()
			.unwrap();
		Shadow {
			api: config.shadow.clone(),
			eth: EthereumRPC::new(http.clone(), vec![config.eth.rpc.clone()]),
			http,
		}
	}

	/// Get mmr
	pub async fn get_parent_mmr_root(&self, block_number: usize) -> Result<MMRRoot> {
		let url = &format!("{}/ethereum/parent_mmr_root/{}", &self.api, block_number);
		let resp = self.http.get(url).send().await?;
		if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
			Err(Error::ShadowInternalServerError(resp.text().await?).into())
		} else {
			let result: ParentMmrRootResult = resp
				.json()
				.await
				.context(format!("Fail to parse json to MMRRootJson: {}", url))?;
			match result {
				ParentMmrRootResult::Result(json) => Ok(json.into()),
				ParentMmrRootResult::Error { error } => {
					Err(BizError::BlankEthereumMmrRoot(block_number, error).into())
				}
			}
		}
	}

	/// Get HeaderParcel
	pub async fn parcel(&self, number: usize) -> Result<EthereumRelayHeaderParcel> {
		let mmr_root = self.get_parent_mmr_root(number).await?;
		let header = self.eth.get_header_by_number(number as u64).await?;

		Ok(EthereumRelayHeaderParcel {
			header,
			mmr_root: mmr_root.mmr_root,
		})
	}

	/// Get Receipt
	pub async fn receipt(&self, tx: &str, last: u64) -> Result<EthereumReceiptProofThing> {
		let resp = self
			.http
			.get(&format!("{}/ethereum/receipt/{}/{}", &self.api, tx, last))
			.send()
			.await?;
		if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
			Err(Error::ShadowInternalServerError(resp.text().await?).into())
		} else {
			let result: Value = resp.json().await?;
			if let Some(err) = result.get("error") {
				Err(BizError::Bridger(err.as_str().unwrap().to_owned()).into())
			} else {
				let json: EthereumReceiptProofThingJson = serde_json::from_value(result)?;
				Ok(json.into())
			}
		}
	}

	/// Get Proposal
	pub async fn proof(
		&self,
		member: u64,
		target: u64,
		last_leaf: u64,
	) -> Result<EthereumRelayProofs> {
		info!(
			"Requesting proposal - member: {}, target: {}, last_leaf: {}",
			member, target, last_leaf
		);
		let map: Value = serde_json::to_value(Proposal {
			member,
			target,
			last_leaf,
		})?;

		let resp = self
			.http
			.post(&format!("{}/ethereum/proof", self.api))
			.json(&map)
			.send()
			.await?;

		if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
			Err(Error::ShadowInternalServerError(resp.text().await?).into())
		} else {
			let result: ProofResult = resp.json().await?;
			match result {
				ProofResult::Result(json) => Ok(json.into()),
				ProofResult::Error { error } => Err(BizError::Bridger(error).into()),
			}
		}
	}
}
