use gql_client::Client;

use bridge_traits::error::StandardError;

use crate::types::{
    AuthoritiesChangeSignedEvent, EmptyQueryVar, MMRRootSignedEvent, QueryTransactionsVars,
    ScheduleAuthoritiesChangeEvent, ScheduleMMRRootEvent, SubqueryResponse,
};

#[derive(Clone, Debug)]
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
            .query_with_vars_unwrap::<SubqueryResponse<MMRRootSignedEvent>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(|e| {
                StandardError::Component(format!(
                    "Failed to send query request to subquery: {:?}",
                    e
                ))
            })?;
        Ok(data
            .data_by_key("mMRRootSignedEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default())
    }

    pub async fn query_latest_schedule_mmr_root_event(
        &self,
    ) -> anyhow::Result<Option<ScheduleMMRRootEvent>> {
        let query = r#"
        query ScheduleMMRRootPage() {
          scheduleMMRRootEvents(
            first: 1
            orderBy: AT_BLOCK_NUMBER_DESC
          ) {
            nodes {
              id
              atBlockNumber
              eventBlockNumber
              emitted
            }
          }
        }
        "#;
        let data = self
            .client
            .query_with_vars_unwrap::<SubqueryResponse<ScheduleMMRRootEvent>, EmptyQueryVar>(
                query,
                EmptyQueryVar,
            )
            .await
            .map_err(|e| {
                StandardError::Component(format!(
                    "Failed to send query request to subquery: {:?}",
                    e
                ))
            })?;
        let rets = data
            .data_by_key("scheduleMMRRootEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default();
        Ok(rets.first().cloned())
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
            .query_with_vars_unwrap::<SubqueryResponse<ScheduleAuthoritiesChangeEvent>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(|e| {
                StandardError::Component(format!(
                    "Failed to send query request to subquery: {:?}",
                    e
                ))
            })?;
        Ok(data
            .data_by_key("scheduleAuthoritiesChangeEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default())
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
            .query_with_vars_unwrap::<SubqueryResponse<AuthoritiesChangeSignedEvent>, QueryTransactionsVars>(
                query, vars,
            )
            .await
            .map_err(|e| {
                StandardError::Component(format!(
                    "Failed to send query request to subquery: {:?}",
                    e
                ))
            })?;
        Ok(data
            .data_by_key("authoritiesChangeSignedEvents")
            .map(|item| item.nodes.clone())
            .unwrap_or_default())
    }
}
