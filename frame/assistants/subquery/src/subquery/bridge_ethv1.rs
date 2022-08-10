use std::collections::HashMap;

use crate::types::{
    AuthoritiesChangeSignedEvent, DataWrapper, EmptyQueryVar, MMRRootSignedEvent,
    QueryTransactionsVars, ScheduleAuthoritiesChangeEvent, ScheduleMMRRootEvent,
};
use crate::{Subquery, SubqueryComponentError, SubqueryComponentResult};

impl Subquery {
    pub async fn query_mmr_root_signed_events(
        &self,
        from: u64,
        first: u32,
    ) -> SubqueryComponentResult<Vec<MMRRootSignedEvent>> {
        let query = self.read_graphql("bridge_ethv1_mmr_root_signed_events.query.graphql")?;
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
        let query =
            self.read_graphql("bridge_ethv1_latest_schedule_mmr_root_event.query.graphql")?;
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<ScheduleMMRRootEvent>>, EmptyQueryVar>(
                query,
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
        let query =
            self.read_graphql("bridge_ethv1_schedule_authorities_change_event.query.graphql")?;
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
        let query =
            self.read_graphql("bridge_ethv1_authorities_change_signed_event.query.graphql")?;
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
