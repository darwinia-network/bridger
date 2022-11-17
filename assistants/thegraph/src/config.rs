use serde::{Deserialize, Serialize};

/// Thegraph config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThegraphConfig {
    /// endpoint
    pub endpoint: String,
}

impl Default for ThegraphConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://api.thegraph.com/subgraphs/name/GITHUB_NAME/GRAPH_NAME".to_string(),
        }
    }
}
