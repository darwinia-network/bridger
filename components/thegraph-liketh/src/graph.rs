use bridge_traits::error::StandardError;
use gql_client::Client;

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
        first: u32,
        skip: u32,
    ) -> anyhow::Result<Vec<TransactionEntity>> {
        let query = r#"
        query TransactionPage($first: Int!, $skip: Int!) {
          transactionEntities(
            skip: $skip
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
        let vars = QueryTransactionsVars { first, skip };
        let data = self
            .client
            .query_with_vars::<TheGraphResponse, QueryTransactionsVars>(query, vars)
            .await
            .unwrap();
        if let TheGraphResponse::TransactionEntities(txs) = data {
            return Ok(txs);
        }
        Err(StandardError::Component(format!(
            "Failed to query transaction page. query: {}, vars: [first: {}, skip: {}]",
            query, first, skip,
        ))
        .into())
    }
}
