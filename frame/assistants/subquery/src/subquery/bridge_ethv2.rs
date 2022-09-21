use std::collections::HashMap;

use crate::types::{
    CollectedEnoughAuthoritiesChangeSignaturesEvent, CollectedEnoughNewMessageRootSignaturesEvent,
    CollectingAuthoritiesChangeSignaturesEvent, CollectingNewMessageRootSignaturesEvent,
    DataWrapper, QueryWithBlockNumberVars,
};
use crate::{Subquery, SubqueryComponentError, SubqueryComponentResult};

impl Subquery {
    pub async fn next_collected_enough_authorities_change_signatures_event(
        &self,
        block: u32,
    ) -> SubqueryComponentResult<Option<CollectedEnoughAuthoritiesChangeSignaturesEvent>> {
        let query = self.read_graphql(
            "bridge_ethv2_next_collected_enough_authorities_change_signatures_event.query.graphql",
        )?;
        let vars = QueryWithBlockNumberVars { block };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<CollectedEnoughAuthoritiesChangeSignaturesEvent>>, QueryWithBlockNumberVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let rets = data
            .get("collectedEnoughAuthoritiesChangeSignaturesEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(rets.first().cloned())
    }

    pub async fn next_collected_enough_new_message_root_signatures_event(
        &self,
        block: u32,
    ) -> SubqueryComponentResult<Option<CollectedEnoughNewMessageRootSignaturesEvent>> {
        let query = self.read_graphql(
            "bridge_ethv2_next_collected_enough_new_message_root_signatures_event.query.graphql",
        )?;
        let vars = QueryWithBlockNumberVars { block };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<CollectedEnoughNewMessageRootSignaturesEvent>>, QueryWithBlockNumberVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let rets = data
            .get("collectedEnoughNewMessageRootSignaturesEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(rets.first().cloned())
    }

    pub async fn next_collecting_authorities_change_signatures_event(
        &self,
        block: u32,
    ) -> SubqueryComponentResult<Option<CollectingAuthoritiesChangeSignaturesEvent>> {
        let query = self.read_graphql(
            "bridge_ethv2_next_collecting_authorities_change_signatures_event.query.graphql",
        )?;
        let vars = QueryWithBlockNumberVars { block };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<CollectingAuthoritiesChangeSignaturesEvent>>, QueryWithBlockNumberVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let rets = data
            .get("collectingAuthoritiesChangeSignaturesEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(rets.first().cloned())
    }

    pub async fn next_collecting_new_message_root_signatures_event(
        &self,
        block: u32,
    ) -> SubqueryComponentResult<Option<CollectingNewMessageRootSignaturesEvent>> {
        let query = self.read_graphql(
            "bridge_ethv2_next_collecting_new_message_root_signatures_event.query.graphql",
        )?;
        let vars = QueryWithBlockNumberVars { block };
        let data = self
            .client
            .query_with_vars_unwrap::<HashMap<String, DataWrapper<CollectingNewMessageRootSignaturesEvent>>, QueryWithBlockNumberVars>(
                query, vars,
            )
            .await
            .map_err(SubqueryComponentError::from)?;
        let rets = data
            .get("collectingNewMessageRootSignaturesEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(rets.first().cloned())
    }
}
