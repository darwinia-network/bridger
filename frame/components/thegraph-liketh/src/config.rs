use serde::{Deserialize, Serialize};

/// Thegraph config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TheGraphLikeEthConfig {
    /// endpoint
    pub endpoint: String,
}

impl Default for TheGraphLikeEthConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://api.thegraph.com/subgraphs/name/GITHUB_NAME/GRAPH_NAME".to_string(),
        }
    }
}
