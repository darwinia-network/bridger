use gql_client::Client;
use include_dir::{include_dir, Dir};

use crate::error::SubqueryComponentError;
use crate::types::BridgeName;
use crate::SubqueryComponentResult;

#[cfg(feature = "bridge-ethv2")]
mod bridge_ethv2;
#[cfg(feature = "bridge-s2s")]
mod bridge_s2s;

/// Graphql dir
static GRAPHQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/graphql");

/// Subquery client
#[derive(Clone, Debug)]
pub struct Subquery {
    client: Client,
    bridge: BridgeName,
}

impl Subquery {
    /// Create subquery instance
    pub fn new(client: Client, bridge: BridgeName) -> Self {
        Self { client, bridge }
    }
}

impl Subquery {
    fn read_graphql(&self, file: impl AsRef<str>) -> SubqueryComponentResult<&str> {
        let file = file.as_ref();
        let dir = self.bridge.name();
        let graph = GRAPHQL_DIR
            .get_file(format!("{}/{}", dir, file))
            .or_else(|| GRAPHQL_DIR.get_file(format!("generic/{}", file)))
            .ok_or_else(|| SubqueryComponentError::GraphQL("No graphql fround".to_string()))?;
        graph.contents_utf8().ok_or_else(|| {
            SubqueryComponentError::GraphQL("Failed to read graphql file".to_string())
        })
    }
}
