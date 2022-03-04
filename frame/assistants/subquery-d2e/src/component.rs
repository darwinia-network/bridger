use crate::config::SubqueryConfig;
use crate::subquery::Subquery;

/// Subquery component
pub struct SubqueryComponent;

impl SubqueryComponent {
    /// Get subquery instance
    pub fn component(config: SubqueryConfig) -> Subquery {
        let client = gql_client::Client::new(config.endpoint);
        Subquery::new(client, config.bridge)
    }
}
