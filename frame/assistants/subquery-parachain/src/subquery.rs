use std::collections::HashMap;

use gql_client::Client;
use include_dir::{include_dir, Dir};

use crate::error::SubqueryComponentError;
use crate::types::{
    BridgeName, CandidateIncludedEvent, DataWrapper, QueryNextCandidateIncludedEventVars,
    QueryNextCandidateIncludedEventWithParaHeadVars,
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
    pub async fn next_candidate_included_event(
        &self,
        block_number: u32,
        para_id: u32,
    ) -> SubqueryComponentResult<Option<CandidateIncludedEvent>> {
        let query = self.read_graphql("next_candiate_included_event.query.graphql")?;
        let vars = QueryNextCandidateIncludedEventVars {
            para_id,
            block_number,
        };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<CandidateIncludedEvent>>, QueryNextCandidateIncludedEventVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let event = data
            .get("nextCandidateIncludedEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(event.get(0).cloned())
    }

    pub async fn get_block_with_para_head(
        &self,
        para_head_hash: impl AsRef<str>,
    ) -> SubqueryComponentResult<Option<CandidateIncludedEvent>> {
        let query = self.read_graphql("next_candiate_included_event.query.graphql")?;
        let vars = QueryNextCandidateIncludedEventWithParaHeadVars {
            para_head: String::from(para_head_hash.as_ref()),
        };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<CandidateIncludedEvent>>, QueryNextCandidateIncludedEventWithParaHeadVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let event = data
            .get("nextCandidateIncludedEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(event.get(0).cloned())
    }
}
