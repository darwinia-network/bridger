//! Darwinia shadow API
use crate::{result::Result, Config};
use primitives::chain::ethereum::{
    EthereumReceiptProofThing, EthereumReceiptProofThingJson, EthereumRelayHeaderParcel,
    EthereumRelayHeaderParcelJson, EthereumRelayProofs, EthereumRelayProofsJson,
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
    /// HTTP Client
    pub http: Client,
}

impl Shadow {
    /// Init Shadow API from config
    pub fn new(config: &Config) -> Shadow {
        Shadow {
            api: config.shadow.clone(),
            http: Client::new(),
        }
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
    ///      api: "https://testnet.shadow.darwinia.network".to_string(),
    ///      http: client,
    ///   };
    ///
    ///   // Get the HeaderParcel of block 42
    ///   shadow.parcel(42).await.unwrap();
    /// }
    /// ```
    pub async fn parcel(&self, number: usize) -> Result<EthereumRelayHeaderParcel> {
        let json: EthereumRelayHeaderParcelJson = self
            .http
            .get(&format!("{}/ethereum/parcel/{}", &self.api, number))
            .send()
            .await?
            .json()
            .await?;

        Ok(json.into())
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
