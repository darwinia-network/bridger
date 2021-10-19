use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TheGraphLikeEthConfig {
    pub endpoint: String,
}

impl BridgeConfig for TheGraphLikeEthConfig {
    fn marker() -> &'static str {
        "component-thegraph-liketh"
    }

    fn template() -> Self {
        Self {
            endpoint: "https://api.thegraph.com/subgraphs/name/GITHUB_NAME/GRAPH_NAME".to_string(),
        }
    }
}
