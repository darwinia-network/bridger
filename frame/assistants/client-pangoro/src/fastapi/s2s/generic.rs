use subxt::rpc::{Subscription, SubscriptionClientT};
use subxt::sp_core;

use crate::client::PangoroClient;
use crate::error::ClientResult;

impl PangoroClient {
    pub async fn subscribe_justification(&self) -> ClientResult<Subscription<sp_core::Bytes>> {
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
