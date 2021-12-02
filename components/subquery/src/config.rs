use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SubqueryConfig {
    pub endpoint: String,
}

impl BridgeConfig for SubqueryConfig {
    fn marker() -> &'static str {
        "component-subquery"
    }

    fn template() -> Self {
        Self {
            endpoint: "https://api.subquery.network/sq/darwinia-network/pangolin-bridger"
                .to_string(),
        }
    }
}
