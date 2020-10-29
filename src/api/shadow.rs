//! Darwinia shadow API
use crate::{result::Result, Config};
use primitives::{
    chain::ethereum::{
        EthereumReceiptProofThing, EthereumReceiptProofThingJson, EthereumRelayHeaderParcel,
        EthereumRelayProofs, EthereumRelayProofsJson, MMRRoot, MMRRootJson,
    },
    rpc::{EthereumRPC, RPC},
};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct Proposal {
    pub member: u64,
    pub target: u64,
    pub last_leaf: u64,
}

/// Shadow API
#[derive(Debug)]
pub struct Shadow {
    /// Shadow API
    pub api: String,
    /// Ethereum API
    pub eth: String,
    /// HTTP Client
    pub http: Client,
}

impl Shadow {
    /// Init Shadow API from config
    pub fn new(config: &Config) -> Shadow {
        Shadow {
            api: config.shadow.clone(),
            eth: config.eth.rpc.clone(),
            http: Client::new(),
        }
    }

    /// Get mmr
    ///
    /// ```
    /// use darwinia_bridger::api::Shadow;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new();
    ///   let shadow = Shadow {
    ///      api: "https://testnet.shadow.darwinia.network.l2me.com".to_string(),
    ///      eth: "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
    ///      http: client,
    ///   };
    ///
    ///   // Get the HeaderParcel of block 42
    ///   shadow.parcel(42).await.unwrap();
    /// }
    /// ```
    pub async fn mmr(&self, number: usize) -> Result<MMRRoot> {
        let json: MMRRootJson = self
            .http
            .get(&format!("{}/ethereum/mmr_root/{}", &self.api, number))
            .send()
            .await?
            .json()
            .await?;

        Ok(json.into())
    }

    /// Get HeaderParcel
    ///
    /// ```
    /// use darwinia_bridger::api::Shadow;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new();
    ///   let shadow = Shadow {
    ///      api: "https://testnet.shadow.darwinia.network.l2me.com".to_string(),
    ///      eth: "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
    ///      http: client,
    ///   };
    ///
    ///   // Get the HeaderParcel of block 42
    ///   shadow.parcel(42).await.unwrap();
    /// }
    /// ```
    pub async fn parcel(&self, number: usize) -> Result<EthereumRelayHeaderParcel> {
        let mmr_root = self.mmr(number).await?;
        let header = EthereumRPC::new(&self.http, &self.eth)
            .get_header_by_number(number as u64)
            .await?;

        Ok(EthereumRelayHeaderParcel {
            header,
            mmr_root: mmr_root.mmr_root,
        })
    }

    /// Get Receipt
    ///
    /// ```
    /// use darwinia_bridger::api::Shadow;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new();
    ///   let shadow = Shadow {
    ///      api: "https://testnet.shadow.darwinia.network".to_string(),
    ///      eth: "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
    ///      http: client,
    ///   };
    ///
    ///   // Get the HeaderParcel of block 42
    ///   shadow.receipt("0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a", 66666).await.unwrap();
    /// }
    /// ```
    pub async fn receipt(&self, tx: &str, last: u64) -> Result<EthereumReceiptProofThing> {
        let json: EthereumReceiptProofThingJson = self
            .http
            .get(&format!("{}/ethereum/receipt/{}/{}", &self.api, tx, last))
            .send()
            .await?
            .json()
            .await?;

        Ok(json.into())
    }

    /// Get Proposal
    ///
    /// ```
    /// use darwinia_bridger::api::Shadow;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let client = Client::new();
    ///   let shadow = Shadow {
    ///      api: "https://testnet.shadow.darwinia.network".to_string(),
    ///      eth: "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
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

        let json: EthereumRelayProofsJson = self
            .http
            .post(&format!("{}/ethereum/proof", self.api))
            .json(&map)
            .send()
            .await?
            .json()
            .await?;

        Ok(json.into())
    }
}
