use serde::{Deserialize, Serialize};

/// Shadow config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShadowConfig {
    /// Shadow server endpoint
    pub endpoint: String,
    /// theGraph endpoint
    pub thegraph: String,
    /// Request timeout
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}
fn default_timeout() -> u64 {
    30
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://shadow.darwinia.network".to_string(),
            thegraph: "https://api.thegraph.com/subgraphs/name/darwinia-network/ethereum-mmr"
                .to_string(),
            timeout: 30,
        }
    }
}
