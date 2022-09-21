use serde::Deserialize;

use crate::types::{MessageAcceptedEvent, TransactionEntity};

#[derive(Debug, Deserialize)]
pub(crate) enum TheGraphResponse {
    #[serde(rename = "transactionEntities")]
    TransactionEntities(Vec<TransactionEntity>),
    #[serde(rename = "messageAcceptedEntities")]
    MessageAcceptedEntities(Vec<MessageAcceptedEvent>),
}
