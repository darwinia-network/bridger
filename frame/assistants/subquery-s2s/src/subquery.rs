use std::collections::HashMap;

use gql_client::Client;
use include_dir::{include_dir, Dir};

use crate::error::SubqueryComponentError;
use crate::types::{
    BridgeName, DataWrapper, EmptyQueryVar, JustificationMapping, NeedRelayBlock, QueryBlockVars,
    QueryTransactionsVars,
};
use crate::SubqueryComponentResult;

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
        let dir = self.bridge.directory();
        let graph = GRAPHQL_DIR
            .get_file(format!("{}/{}", dir, file))
            .or_else(|| GRAPHQL_DIR.get_file(format!("generic/{}", file)))
            .ok_or_else(|| SubqueryComponentError::GraphQL("No graphql fround".to_string()))?;
        graph.contents_utf8().ok_or_else(|| {
            SubqueryComponentError::GraphQL("Failed to read graphql file".to_string())
        })
    }
}

impl Subquery {
    pub async fn query_headers_need_to_relay(
        &self,
        from: u64,
        first: u32,
    ) -> SubqueryComponentResult<Vec<NeedRelayBlock>> {
        let query = self.read_graphql("query_next_header.query.graphql")?;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<NeedRelayBlock>>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        Ok(data
            .get("needRelayBlocks")
            .map(|item| item.nodes.clone())
            .unwrap_or_default())
    }

    pub async fn query_justification(
        &self,
        block_number: u32,
    ) -> SubqueryComponentResult<Option<JustificationMapping>> {
        let query = self.read_graphql("query_justification.query.graphql")?;
        let vars = QueryBlockVars { block_number };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<JustificationMapping>>, QueryBlockVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        Ok(data
            .get("justificationMappings")
            .map(|item| item.nodes.last().cloned())
            .unwrap_or_default())
    }

    pub async fn query_latest_justification(
        &self,
    ) -> SubqueryComponentResult<Option<JustificationMapping>> {
        let query = self.read_graphql("query_latest_justification.query.graphql")?;
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<JustificationMapping>>, EmptyQueryVar>(
                query, EmptyQueryVar,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        Ok(data
            .get("justificationMappings")
            .map(|item| item.nodes.last().cloned())
            .unwrap_or_default())
    }
}
