use subxt::rpc::{Subscription, SubscriptionClientT};

use crate::client::PangolinClient;
use crate::error::ClientResult;

impl PangolinClient {
    pub async fn subscribe_justification(&self) -> ClientResult<Subscription<Vec<u8>>> {
        Ok(self
            .subxt()
            .rpc()
            .client
            .subscribe(
                "grandpa_subscribeJustifications",
                None,
                "grandpa_unsubscribeJustifications",
            )
            .await?)
    }
}
