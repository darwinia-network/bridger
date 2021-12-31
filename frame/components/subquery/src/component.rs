use crate::config::SubqueryConfig;
use crate::subquery::Subquery;

/// Subquery component
pub struct SubqueryComponent;

impl SubqueryComponent {
    /// Get subquery instance
    pub fn component(config: SubqueryConfig) -> color_eyre::Result<Subquery> {
        let client = gql_client::Client::new(config.endpoint);
        Ok(Subquery::new(client))
    }
}
