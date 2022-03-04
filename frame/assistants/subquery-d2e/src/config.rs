use serde::{Deserialize, Serialize};

use crate::types::BridgeName;

///Subquery config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubqueryConfig {
    /// The endpoint for subquery
    pub endpoint: String,
    /// Bridge type
    pub bridge: BridgeName,
}

impl Default for SubqueryConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://api.subquery.network/sq/darwinia-network/pangolin-bridger"
                .to_string(),
            bridge: BridgeName::DarwiniaEthereum,
        }
    }
}
