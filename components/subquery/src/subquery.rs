use gql_client::Client;

use bridge_traits::error::StandardError;

use crate::types::{
    AuthoritiesChangeSignedEvent, MMRRootSignedEvent, QueryTransactionsVars,
    ScheduleAuthoritiesChangeEvent, ScheduleMMRRootEvent, SubqueryResponse,
};

pub struct Subquery {
    client: Client,
}

impl Subquery {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Subquery {
    pub async fn query_mmr_root_signed_events(
        &self,
        from: u64,
        first: u32,
    ) -> anyhow::Result<Vec<MMRRootSignedEvent>> {
        let query = r#"
        query MMRRootPage($from: Int!, $first: Int!) {
          mMRRootSignedEvents(
            first: $first
            orderBy: AT_BLOCK_NUMBER_ASC
            filter: {
              atBlockNumber: {
                greaterThan: $from
              }
            }
          ) {
            nodes {
              atBlockNumber,
              eventBlockNumber,
              mmrRoot,
              signatures {
                nodes {
                  account,
                  relayAuthoritySignature,
                }
              }
            }
          }
        }
        "#;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars::<SubqueryResponse<MMRRootSignedEvent>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(|e| {
                StandardError::Component(format!(
                    "Failed to send query request to subquery: {:?}",
                    e
                ))
            })?;
        let wrapper = match data.data_by_key("mMRRootSignedEvents") {
            Some(v) => v,
            None => return Ok(Default::default()),
        };
        Ok(wrapper.nodes.clone())
    }

    pub async fn query_schedule_mmr_root_event(
        &self,
        from: u64,
        first: u32,
    ) -> anyhow::Result<Vec<ScheduleMMRRootEvent>> {
        let query = r#"
        query ScheduleMMRRootPage($from: Int!, $first: Int!) {
          scheduleMMRRootEvents(
            first: $first
            orderBy: AT_BLOCK_NUMBER_ASC
            filter: {
              atBlockNumber: {
                greaterThan: $from
              }
            }
          ) {
            nodes {
              atBlockNumber
              eventBlockNumber
            }
          }
        }
        "#;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars::<SubqueryResponse<ScheduleMMRRootEvent>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(|e| {
                StandardError::Component(format!(
                    "Failed to send query request to subquery: {:?}",
                    e
                ))
            })?;
        let wrapper = match data.data_by_key("scheduleMMRRootEvents") {
            Some(v) => v,
            None => return Ok(Default::default()),
        };
        Ok(wrapper.nodes.clone())
    }

    pub async fn query_schedule_authorities_change_event(
        &self,
        from: u64,
        first: u32,
    ) -> anyhow::Result<Vec<ScheduleAuthoritiesChangeEvent>> {
        let query = r#"
        query scheduleAuthoritiesChangePage($from: Int!, $first: Int!) {
          scheduleAuthoritiesChangeEvents(
            first: $first
            orderBy: AT_BLOCK_NUMBER_ASC
            filter: {
              atBlockNumber: {
                greaterThan: $from
              }
            }
          ) {
            nodes {
              atBlockNumber
              message
            }
          }
        }
        "#;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars::<SubqueryResponse<ScheduleAuthoritiesChangeEvent>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(|e| {
                StandardError::Component(format!(
                    "Failed to send query request to subquery: {:?}",
                    e
                ))
            })?;
        let wrapper = match data.data_by_key("scheduleAuthoritiesChangeEvents") {
            Some(v) => v,
            None => return Ok(Default::default()),
        };
        Ok(wrapper.nodes.clone())
    }

    pub async fn query_authorities_change_signed_event(
        &self,
        from: u64,
        first: u32,
    ) -> anyhow::Result<Vec<AuthoritiesChangeSignedEvent>> {
        let query = r#"
        query authoritiesChangeSignedPage($from: Int!, $first: Int!) {
          authoritiesChangeSignedEvents(
            first: $first
            orderBy: AT_BLOCK_NUMBER_ASC
            filter: {
              atBlockNumber: {
                greaterThan: $from
              }
            }
          ) {
            nodes {
              atBlockNumber
              term
              newAuthorities
              signatures {
                nodes {
                  account,
                  relayAuthoritySignature
                }
              }
            }
          }
        }
        "#;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars::<SubqueryResponse<AuthoritiesChangeSignedEvent>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(|e| {
                StandardError::Component(format!(
                    "Failed to send query request to subquery: {:?}",
                    e
                ))
            })?;
        let wrapper = match data.data_by_key("authoritiesChangeSignedEvents") {
            Some(v) => v,
            None => return Ok(Default::default()),
        };
        Ok(wrapper.nodes.clone())
    }
}
