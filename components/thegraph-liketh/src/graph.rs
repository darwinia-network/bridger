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
        from: u64,
        first: u32,
    ) -> anyhow::Result<Vec<TransactionEntity>> {
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
            .query_with_vars::<TheGraphResponse, QueryTransactionsVars>(query, vars)
            .await
            .unwrap();
        if let TheGraphResponse::TransactionEntities(txs) = data {
            return Ok(txs);
        }
        Err(StandardError::Component(format!(
            "Failed to query transaction page. query: {}, vars: [from: {}, limit: {}]",
            query, from, first,
        ))
        .into())
    }
}
