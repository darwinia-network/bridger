use crate::error::{TheGraphLikethComponentError, ThegraphComponentReuslt};
use crate::thegraph::Thegraph;
use crate::types::{MessageAcceptedEvent, QueryMessageEventVars, TheGraphResponse};

impl Thegraph {
    #[allow(irrefutable_let_patterns)]
    pub async fn query_message_accepted(
        &self,
        nonce: u64,
    ) -> ThegraphComponentReuslt<Option<MessageAcceptedEvent>> {
        let query = self.read_graphql("message_accepted_event.query.graphql")?;
        let vars = QueryMessageEventVars { nonce };
        let data = self
            .client
            .query_with_vars_unwrap::<TheGraphResponse, QueryMessageEventVars>(query, vars)
            .await
            .map_err(TheGraphLikethComponentError::from)?;
        if let TheGraphResponse::MessageAcceptedEntities(events) = data {
            if events.len() == 1 {
                return Ok(Some(events[0].clone()));
            } else {
                return Ok(None);
            }
        }

        Err(TheGraphLikethComponentError::UnknownResponse(format!(
            "QUERY: {}, VARS: {}",
            query, nonce
        ))
        .into())
    }
}
