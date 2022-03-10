use serde::Deserialize;

use crate::types::TransactionEntity;

#[derive(Debug, Deserialize)]
pub(crate) enum TheGraphResponse {
    #[serde(rename = "transactionEntities")]
    TransactionEntities(Vec<TransactionEntity>),
}
