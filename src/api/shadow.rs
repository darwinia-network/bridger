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
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct Proposal {
	pub member: u64,
	pub target: u64,
	pub last_leaf: u64,
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
		let http = Client::new();
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
			let json: MMRRootJson = resp
				.json()
				.await
				.context(format!("Fail to parse json to MMRRootJson: {}", url))?;
			let result = json.into();
			if result == MMRRoot::default() {
				Err(BizError::BlankEthereumMmrRoot(block_number).into())
			} else {
				Ok(result)
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
			let json: EthereumReceiptProofThingJson = resp.json().await?;
			Ok(json.into())
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
			let json: EthereumRelayProofsJson = resp.json().await?;
			Ok(json.into())
		}
	}
}
