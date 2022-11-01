use serde::Deserialize;

use crate::types::MessageAcceptedEvent;

#[derive(Debug, Deserialize)]
pub(crate) enum TheGraphResponse {
    #[serde(rename = "messageAcceptedEntities")]
    MessageAcceptedEntities(Vec<MessageAcceptedEvent>),
}
