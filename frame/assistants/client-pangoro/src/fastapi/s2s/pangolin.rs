use abstract_client_s2s::client::S2sClientRelay;

use crate::client::PangoroClient;

#[async_trait::async_trait]
impl S2sClientRelay for PangoroClient {}
