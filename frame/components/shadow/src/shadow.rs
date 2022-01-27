//! Darwinia shadow API
use std::convert::TryInto;

use color_eyre::eyre::Context;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use component_ethereum::errors::BizError;
use component_ethereum::ethereum::client::EthereumClient;

use crate::config::ShadowConfig;
use crate::error::ShadowComponentError;
use crate::types::MMRRootJson;

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
    pub eth: EthereumClient,
    /// HTTP Client
    pub http: Client,
}

impl Shadow {
    /// Init Shadow API from config
    pub fn new(config: ShadowConfig, http_client: Client, eth: EthereumClient) -> Shadow {
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
    ) -> color_eyre::Result<MMRRootJson> {
        let url = &format!(
            "{}/ethereum/parent_mmr_root/{}",
            &self.config.endpoint, block_number
        );
        let resp = self.http.get(url).send().await?;
        if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
            return Err(ShadowComponentError::InternalServer(resp.text().await?).into());
        }

        let result: ParentMmrRootResult = resp
            .json()
            .await
            .context(format!("Fail to parse json to MMRRootJson: {}", url))?;
        match result {
            ParentMmrRootResult::Result(json) => Ok(json),
            ParentMmrRootResult::Error { error } => {
                Err(BizError::BlankEthereumMmrRoot(block_number, error).into())
            }
        }
    }

    /// Get HeaderParcel
    pub async fn parcel(&self, number: usize) -> color_eyre::Result<EthereumRelayHeaderParcel> {
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
        tx: impl AsRef<str>,
        last: u64,
    ) -> color_eyre::Result<EthereumReceiptProofThing> {
        let resp = self
            .http
            .get(&format!(
                "{}/ethereum/receipt/{}/{}",
                &self.config.endpoint,
                tx.as_ref(),
                last
            ))
            .send()
            .await?;
        if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
            return Err(ShadowComponentError::InternalServer(resp.text().await?).into());
        }
        let result: Value = resp.json().await?;
        if let Some(err) = result.get("error") {
            let msg = err
                .as_str()
                .ok_or_else(|| BizError::Other("Failed parse error message".to_string()))?;
            Err(BizError::Other(msg.to_owned()).into())
        } else {
            let json: EthereumReceiptProofThingJson = serde_json::from_value(result)?;
            Ok(json.try_into()?)
        }
    }

    /// Get Proposal
    pub async fn proof(
        &self,
        member: u64,
        target: u64,
        last_leaf: u64,
    ) -> color_eyre::Result<EthereumRelayProofs> {
        tracing::info!(
            target: "component-shadow",
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
            return Err(ShadowComponentError::InternalServer(resp.text().await?).into());
        }
        let result: ProofResult = resp.json().await?;
        match result {
            ProofResult::Result(json) => Ok(json.try_into()?),
            ProofResult::Error { error } => Err(BizError::Other(error).into()),
        }
    }
}
