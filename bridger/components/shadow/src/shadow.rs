//! Darwinia shadow API
use anyhow::Context as AnyhowContext;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use bridge_traits::error::StandardError;
use component_ethereum::error::{BizError, ComponentEthereumResult};
use component_ethereum::ethereum_rpc::EthereumRpc;
use support_ethereum::mmr::{MMRRoot, MMRRootJson};
use support_ethereum::parcel::EthereumRelayHeaderParcel;
use support_ethereum::proof::{EthereumRelayProofs, EthereumRelayProofsJson};
use support_ethereum::receipt::{EthereumReceiptProofThing, EthereumReceiptProofThingJson};

use crate::config::ShadowConfig;

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
    /// shadow config
    pub config: ShadowConfig,
    /// Ethereum RPC
    pub eth: EthereumRpc,
    /// HTTP Client
    pub http: Client,
}

impl Shadow {
    /// Init Shadow API from config
    pub fn new(config: ShadowConfig, http_client: Client, eth: EthereumRpc) -> Shadow {
        Shadow {
            config,
            eth,
            http: http_client,
        }
    }

    /// Get mmr
    pub async fn get_parent_mmr_root(
        &self,
        block_number: usize,
    ) -> ComponentEthereumResult<MMRRoot> {
        let url = &format!(
            "{}/ethereum/parent_mmr_root/{}",
            &self.config.endpoint, block_number
        );
        let resp = self.http.get(url).send().await?;
        if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
            Err(StandardError::ShadowInternalServerError(resp.text().await?).into())
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
    pub async fn parcel(
        &self,
        number: usize,
    ) -> ComponentEthereumResult<EthereumRelayHeaderParcel> {
        let mmr_root = self.get_parent_mmr_root(number).await?;
        let header = self.eth.get_header_by_number(number as u64).await?;

        Ok(EthereumRelayHeaderParcel {
            header,
            mmr_root: mmr_root.mmr_root,
        })
    }

    /// Get Receipt
    pub async fn receipt(
        &self,
        tx: &str,
        last: u64,
    ) -> ComponentEthereumResult<EthereumReceiptProofThing> {
        let resp = self
            .http
            .get(&format!(
                "{}/ethereum/receipt/{}/{}",
                &self.config.endpoint, tx, last
            ))
            .send()
            .await?;
        if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
            Err(StandardError::ShadowInternalServerError(resp.text().await?).into())
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
    ) -> ComponentEthereumResult<EthereumRelayProofs> {
        log::info!(
            "Requesting proposal - member: {}, target: {}, last_leaf: {}",
            member,
            target,
            last_leaf
        );
        let map: Value = serde_json::to_value(Proposal {
            member,
            target,
            last_leaf,
        })?;

        let resp = self
            .http
            .post(&format!("{}/ethereum/proof", self.config.endpoint))
            .json(&map)
            .send()
            .await?;

        if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
            Err(StandardError::ShadowInternalServerError(resp.text().await?).into())
        } else {
            let result: ProofResult = resp.json().await?;
            match result {
                ProofResult::Result(json) => Ok(json.into()),
                ProofResult::Error { error } => Err(BizError::Bridger(error).into()),
            }
        }
    }
}
