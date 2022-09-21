use gql_client::Client;
use include_dir::{include_dir, Dir};

use crate::error::{TheGraphLikethComponentError, TheGraphLikethComponentReuslt};
use crate::types::{
    EmptyQueryVar, LikethChain, MessageAcceptedEvent, QueryMessageEventVars, QueryTransactionsVars,
    TheGraphResponse, TransactionEntity,
};

/// Graphql dir
static GRAPHQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/graphql");

/// thegraph toolkit, query ethereum/ropsten
pub struct TheGraphLikeEth {
    client: Client,
    chain: LikethChain,
}

impl TheGraphLikeEth {
    pub fn new(client: Client, chain: LikethChain) -> Self {
        Self { client, chain }
    }
}

impl TheGraphLikeEth {
    fn read_graphql(&self, file: impl AsRef<str>) -> TheGraphLikethComponentReuslt<&str> {
        let file = file.as_ref();
        let dir = self.chain.directory();
        let graph = GRAPHQL_DIR
            .get_file(format!("{}/{}", dir, file))
            .or_else(|| GRAPHQL_DIR.get_file(format!("generic/{}", file)))
            .ok_or_else(|| {
                TheGraphLikethComponentError::GraphQL("No graphql fround".to_string())
            })?;
        graph.contents_utf8().ok_or_else(|| {
            TheGraphLikethComponentError::GraphQL("Failed to read graphql file".to_string())
        })
    }
}

impl TheGraphLikeEth {
    #[allow(irrefutable_let_patterns)]
    pub async fn last_transaction(
        &self,
    ) -> TheGraphLikethComponentReuslt<Option<TransactionEntity>> {
        let query = self.read_graphql("transactions_last.query.graphql")?;
        let vars = EmptyQueryVar;
        let data = self
            .client
            .query_with_vars_unwrap::<TheGraphResponse, EmptyQueryVar>(query, vars)
            .await
            .map_err(TheGraphLikethComponentError::from)?;
        if let TheGraphResponse::TransactionEntities(txs) = data {
            return Ok(txs.get(0).cloned());
        }
        Err(TheGraphLikethComponentError::UnknownResponse(format!("QUERY: {}", query,)).into())
    }

    /// Query transactions page
    #[allow(irrefutable_let_patterns)]
    pub async fn query_transactions(
        &self,
        from: u64,
        first: u32,
        equals_from: bool,
    ) -> TheGraphLikethComponentReuslt<Vec<TransactionEntity>> {
        let query = self.read_graphql(if equals_from {
            "transactions_page_gte_from.query.graphql"
        } else {
            "transactions_page_gt_from.query.graphql"
        })?;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars_unwrap::<TheGraphResponse, QueryTransactionsVars>(query, vars)
            .await
            .map_err(TheGraphLikethComponentError::from)?;
        if let TheGraphResponse::TransactionEntities(txs) = data {
            return Ok(txs);
        }
        Err(TheGraphLikethComponentError::UnknownResponse(format!(
            "QUERY: {}, VARS: [{}, {}]",
            query, from, first
        ))
        .into())
    }

    pub async fn query_message_accepted(
        &self,
        nonce: u64,
    ) -> TheGraphLikethComponentReuslt<Option<MessageAcceptedEvent>> {
        let query = self.read_graphql("message_accepted_event.query.graphql")?;
        let vars = QueryMessageEventVars { nonce };
        let data = self
            .client
            .query_with_vars_unwrap::<TheGraphResponse, QueryMessageEventVars>(query, vars)
            .await
            .map_err(TheGraphLikethComponentError::from)?;
        if let TheGraphResponse::MessageAcceptedEntities(events) = data {
            if events.len() == 1 {
                return Ok(Some(events[0].clone()));
            } else {
                return Ok(None);
            }
        }

        Err(TheGraphLikethComponentError::UnknownResponse(format!(
            "QUERY: {}, VARS: {}",
            query, nonce
        ))
        .into())
    }
}
