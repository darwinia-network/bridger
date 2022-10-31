use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAcceptedEvent {
    pub id: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub block_number: u64,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub nonce: u64,
    pub source: String,
    pub target: String,
    pub encoded: String,
}
