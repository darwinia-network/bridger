//! Darwinia shadow API
use crate::{result::Result, Config};
use primitives::chain::eth::{
    EthereumReceiptProofThing, EthereumReceiptProofThingJson, HeaderStuff, HeaderStuffJson,
    HeaderThing, HeaderThingWithConfirmationJson,
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

    /// Get HeaderThing
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
    ///      client: client,
    ///   };
    ///
    ///   // Get the HeaderThing of block 42
    ///   shadow.header_thing(42).await.unwrap();
    /// }
    /// ```
    pub async fn header_thing(&self, number: usize) -> Result<HeaderThing> {
        let json: HeaderThingWithConfirmationJson = self
            .http
            .get(&format!("{}/eth/header/{}", &self.api, number))
            .send()
            .await?
            .json()
            .await?;

        Ok(json.header_thing.into())
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
    ///      client: client,
    ///   };
    ///
    ///   // Get the HeaderThing of block 42
    ///   shadow.receipt("0x3b82a55f5e752c23359d5c3c4c3360455ce0e485ed37e1faabe9ea10d5db3e7a/66666").await.unwrap();
    /// }
    /// ```
    pub async fn receipt(&self, tx: &str, last: u64) -> Result<EthereumReceiptProofThing> {
        let json: EthereumReceiptProofThingJson = self
            .http
            .get(&format!("{}/eth/receipt/{}/{}", &self.api, tx, last))
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
    ///      client: client,
    ///   };
    ///
    ///   // Get the HeaderThing of block 42
    ///   shadow.proposal(10, 20, 19).await.unwrap();
    /// }
    /// ```
    pub async fn proposal(&self, member: u64, target: u64, last_leaf: u64) -> Result<HeaderStuff> {
        info!(
            "Requesting proposal - member: {}, target: {}, last_leaf: {}",
            member, target, last_leaf
        );
        let map: Value = serde_json::to_value(Proposal {
            member,
            target,
            last_leaf,
        })?;

        let json: HeaderStuffJson = self
            .http
            .post(&format!("{}/eth/proposal", self.api))
            .json(&map)
            .send()
            .await?
            .json()
            .await?;

        Ok(json.into())
    }
}
