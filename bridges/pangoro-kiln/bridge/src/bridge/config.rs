use serde::{Deserialize, Serialize};
use subquery::types::BridgeName;
use subquery::{Subquery, SubqueryComponent, SubqueryConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub pangoro: PangoroConfig,
    pub kiln: ChainInfoConfig,
    pub index: IndexConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfoConfig {
    pub endpoint: String,
    pub execution_layer_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    pub inbound_address: String,
    pub outbound_address: String,
    pub fee_market_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PangoroConfig {
    pub endpoint: String,
    pub contract_address: String,
    pub execution_layer_contract_address: String,
    pub account: String,
    pub private_key: String,
    pub inbound_address: String,
    pub outbound_address: String,
    pub fee_market_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub pangoro: SubqueryConfig,
}

impl IndexConfig {
    pub fn to_pangoro_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.pangoro.clone(), BridgeName::PangoroGoerli)
    }
}
