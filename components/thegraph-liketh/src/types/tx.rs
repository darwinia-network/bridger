use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct QueryTransactionsVars {
    pub(crate) from: u64,
    pub(crate) first: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    /// Deposit event
    Deposit,
    /// Token event
    Token,
    /// SetAuthoritiesEvent
    SetAuthorities,
    /// RegisterErc20Token
    RegisterErc20Token,
    /// RedeemErc20Token
    RedeemErc20Token,
}

impl TransactionType {
    pub fn belong(&self) -> TransactionOrigin {
        match self {
            TransactionType::Deposit => TransactionOrigin::Bank,
            TransactionType::Token => TransactionOrigin::Issuing,
            TransactionType::SetAuthorities => TransactionOrigin::Relay,
            TransactionType::RegisterErc20Token => TransactionOrigin::Backing,
            TransactionType::RedeemErc20Token => TransactionOrigin::Backing,
        }
    }
}

/// transaction origin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionOrigin {
    Bank,
    Relay,
    Issuing,
    Backing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(rename = "txType")]
    pub tx_type: TransactionType,
}
