use crate::config::SubqueryConfig;
use crate::subquery::Subquery;
use crate::types::BridgeName;

/// Subquery component
pub struct SubqueryComponent;

impl SubqueryComponent {
    /// Get subquery instance
    pub fn component(config: SubqueryConfig, bridge: BridgeName) -> Subquery {
        let client = gql_client::Client::new(config.endpoint);
        Subquery::new(client, bridge)
    }
}
