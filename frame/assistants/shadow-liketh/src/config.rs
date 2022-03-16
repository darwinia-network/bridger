use serde::{Deserialize, Serialize};

/// Shadow config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShadowConfig {
    /// Shadow server endpoint
    pub endpoint: String,
    /// theGraph endpoint
    pub thegraph: String,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://shadow.darwinia.network".to_string(),
            thegraph: "https://api.thegraph.com/subgraphs/name/darwinia-network/ethereum-mmr"
                .to_string(),
        }
    }
}
