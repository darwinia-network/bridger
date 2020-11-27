//! Darwinia shadow API
use crate::{error::{
    Result, BizError
}, Config};
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
use crate::error::Error;

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
    ///
    /// ```
    /// use darwinia_bridger::api::Shadow;
    /// use primitives::rpc::ethereum::EthereumRPC;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let rpc = EthereumRPC::new(
    ///     Client::new(),
    ///     vec![
    ///       "https://mainnet.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
    ///       "https://mainnet.infura.io/v3/74a9b1b5816b47aa853c23fcc4f2f3b6".to_string(),
    ///     ],
    ///   );
    ///
    ///   let client = Client::new();
    ///   let shadow = Shadow {
    ///      api: "https://testnet.shadow.darwinia.network.l2me.com".to_string(),
    ///      eth: rpc,
    ///      http: client,
    ///   };
    ///
    ///   // Get the HeaderParcel of block 42
    ///   shadow.parcel(42).await.unwrap();
    /// }
    /// ```
    pub async fn get_mmr_root(&self, leaf_index: usize) -> Result<MMRRoot> {
        let block_number = leaf_index + 1;
        let url = &format!("{}/ethereum/parent_mmr_root/{}", &self.api, block_number);
        let resp = self
            .http
            .get(url)
            .send()
            .await?;
        if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
            Err(Error::ShadowInternalServerError(resp.text().await?))
        } else {
            let json: MMRRootJson = resp.json()
                .await?;
            let result = json.into();
            if result == MMRRoot::default() {
                Err(BizError::BlankEthereumMmrRoot(leaf_index).into())
            } else {
                Ok(result)
            }
        }
    }

    /// Get HeaderParcel
    ///
    /// ```
    /// use darwinia_bridger::api::Shadow;
    /// use primitives::rpc::ethereum::EthereumRPC;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let rpc = EthereumRPC::new(
    ///     Client::new(),
    ///     vec![
    ///       "https://mainnet.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
    ///       "https://mainnet.infura.io/v3/74a9b1b5816b47aa853c23fcc4f2f3b6".to_string(),
    ///     ],
    ///   );
    ///
    ///   let client = Client::new();
    ///   let shadow = Shadow {
    ///      api: "https://testnet.shadow.darwinia.network.l2me.com".to_string(),
    ///      eth: rpc,
    ///      http: client,
    ///   };
    ///
    ///   // Get the HeaderParcel of block 43 with mmr_root_of_42(which treat as an item in header of 43)
    ///   shadow.parcel(42).await.unwrap();
    /// }
    /// ```
    pub async fn parcel(&self, number: usize) -> Result<EthereumRelayHeaderParcel> {
        let mmr_root = self.get_mmr_root(number).await?;
        let header = self.eth.get_header_by_number(number as u64 + 1).await?;

        Ok(EthereumRelayHeaderParcel {
            header,
            mmr_root: mmr_root.mmr_root,
        })
    }

    /// Get Receipt
    ///
    /// ```
    /// use darwinia_bridger::api::Shadow;
    /// use primitives::rpc::ethereum::EthereumRPC;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let rpc = EthereumRPC::new(
    ///     Client::new(),
    ///     vec![
    ///       "https://mainnet.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
    ///       "https://mainnet.infura.io/v3/74a9b1b5816b47aa853c23fcc4f2f3b6".to_string(),
    ///     ],
    ///   );
    ///
    ///   let client = Client::new();
    ///   let shadow = Shadow {
    ///      api: "https://testnet.shadow.darwinia.network.l2me.com".to_string(),
    ///      eth: rpc,
    ///      http: client,
    ///   };
    ///
    ///   // Get the HeaderParcel of block 42
    ///   shadow.receipt("0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a", 66666).await.unwrap();
    /// }
    /// ```
    pub async fn receipt(&self, tx: &str, last: u64) -> Result<EthereumReceiptProofThing> {
        let resp = self
            .http
            .get(&format!("{}/ethereum/receipt/{}/{}", &self.api, tx, last))
            .send()
            .await?;
        if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
            Err(Error::ShadowInternalServerError(resp.text().await?))
        } else {
            let json: EthereumReceiptProofThingJson = resp.json().await?;
            Ok(json.into())
        }
    }

    /// Get Proposal
    ///
    /// ```
    /// use darwinia_bridger::api::Shadow;
    /// use primitives::rpc::ethereum::EthereumRPC;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let rpc = EthereumRPC::new(
    ///     Client::new(),
    ///     vec![
    ///       "https://mainnet.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
    ///       "https://mainnet.infura.io/v3/74a9b1b5816b47aa853c23fcc4f2f3b6".to_string(),
    ///     ],
    ///   );
    ///
    ///   let client = Client::new();
    ///   let shadow = Shadow {
    ///      api: "https://testnet.shadow.darwinia.network.l2me.com".to_string(),
    ///      eth: rpc,
    ///      http: client,
    ///   };
    ///
    ///   // Get the HeaderParcel of block 42
    ///   shadow.proof(10, 20, 19).await.unwrap();
    /// }
    /// ```
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
            Err(Error::ShadowInternalServerError(resp.text().await?))
        } else {
            let json: EthereumRelayProofsJson = resp.json().await?;
            Ok(json.into())
        }
    }
}
