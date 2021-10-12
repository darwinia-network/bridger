use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct QueryTransactionsVars {
    pub(crate) first: u32,
    pub(crate) skip: u32,
}

#[derive(Debug, Deserialize)]
pub struct TransactionEntity {
    pub id: String,
    pub origin: TransactionOrigin,
    #[serde(rename = "blockNumber")]
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub block_number: u64,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    #[serde(rename = "txIndex")]
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub tx_index: u64,
}
