use serde::Deserialize;

#[cfg(feature = "bridge-ethv2")]
use crate::types::MessageAcceptedEvent;

#[derive(Debug, Deserialize)]
pub(crate) enum TheGraphResponse {
    #[cfg(feature = "bridge-ethv2")]
    #[serde(rename = "messageAcceptedEntities")]
    MessageAcceptedEntities(Vec<MessageAcceptedEvent>),
}
