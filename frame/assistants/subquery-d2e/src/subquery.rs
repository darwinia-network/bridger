use std::collections::HashMap;

use gql_client::Client;
use include_dir::{include_dir, Dir};

use crate::error::SubqueryComponentError;
use crate::types::{
    AuthoritiesChangeSignedEvent, BridgeName, DataWrapper, EmptyQueryVar, MMRRootSignedEvent,
    QueryTransactionsVars, ScheduleAuthoritiesChangeEvent, ScheduleMMRRootEvent,
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
    pub async fn query_mmr_root_signed_events(
        &self,
        from: u64,
        first: u32,
    ) -> SubqueryComponentResult<Vec<MMRRootSignedEvent>> {
        let query = self.read_graphql("mmr_root_signed_events.query.graphql")?;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<MMRRootSignedEvent>>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        Ok(data
            .get("mMRRootSignedEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default())
    }

    pub async fn query_latest_schedule_mmr_root_event(
        &self,
    ) -> SubqueryComponentResult<Option<ScheduleMMRRootEvent>> {
        let query = self.read_graphql("latest_schedule_mmr_root_event.query.graphql")?;
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<ScheduleMMRRootEvent>>, EmptyQueryVar>(
                &query[..],
                EmptyQueryVar,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let rets = data
            .get("scheduleMMRRootEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(rets.first().cloned())
    }

    pub async fn query_schedule_authorities_change_event(
        &self,
        from: u64,
        first: u32,
    ) -> SubqueryComponentResult<Vec<ScheduleAuthoritiesChangeEvent>> {
        let query = self.read_graphql("schedule_authorities_change_event.query.graphql")?;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<ScheduleAuthoritiesChangeEvent>>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        Ok(data
            .get("scheduleAuthoritiesChangeEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default())
    }

    pub async fn query_authorities_change_signed_event(
        &self,
        from: u64,
        first: u32,
    ) -> SubqueryComponentResult<Vec<AuthoritiesChangeSignedEvent>> {
        let query = self.read_graphql("authorities_change_signed_event.query.graphql")?;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<AuthoritiesChangeSignedEvent>>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        Ok(data
            .get("authoritiesChangeSignedEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default())
    }
}
