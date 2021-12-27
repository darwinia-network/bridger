use gql_client::Client;

use crate::error::TheGraphLikethComponentError;
use crate::types::{QueryTransactionsVars, TheGraphResponse, TransactionEntity};

/// thegraph toolkit, query ethereum/ropsten
pub struct TheGraphLikeEth {
    client: Client,
}

impl TheGraphLikeEth {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl TheGraphLikeEth {
    /// Query transactions page
    #[allow(irrefutable_let_patterns)]
    pub async fn query_transactions(
        &self,
        from: u64,
        first: u32,
    ) -> color_eyre::Result<Vec<TransactionEntity>> {
        let query = r#"
        query TransactionPage($from: Int!, $first: Int!) {
          transactionEntities(
            where: {
              blockNumber_gt: $from
            }
            first: $first
            orderBy: blockNumber
            orderDirection: asc
          ) {
            id
            origin
            blockNumber
            blockHash
            txHash
            txIndex
            txType
          }
        }
        "#;
        let vars = QueryTransactionsVars { from, first };
        let data = self
            .client
            .query_with_vars_unwrap::<TheGraphResponse, QueryTransactionsVars>(query, vars)
            .await
            .map_err(|e| TheGraphLikethComponentError::from(e))?;
        if let TheGraphResponse::TransactionEntities(txs) = data {
            return Ok(txs);
        }
        Err(TheGraphLikethComponentError::UnknownResponse(query.to_string(), from, first).into())
    }
}
