use subquery::types::BridgeName;
use subquery::{Subquery, SubqueryComponent, SubqueryConfig};

pub fn subquery(bridge_name: BridgeName) -> Subquery {
    let config = SubqueryConfig {
        endpoint: "https://subql.darwinia.network/subql-bridger-pangoro/".to_string(),
    };
    SubqueryComponent::component(config, bridge_name)
}
