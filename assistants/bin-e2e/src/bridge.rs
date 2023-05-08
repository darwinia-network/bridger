use component_state::state::BridgeState;
use lifeline::prelude::*;
use crate::service::header_relay::types::{DarwiniaHeader, EthereumHeader};

lifeline_bus!(pub struct BridgeBus);

impl Resource<BridgeBus> for BridgeState {}

impl Message<BridgeBus> for EthereumHeader {
    type Channel = tokio::sync::broadcast::Sender<Self>;
}
impl Message<BridgeBus> for DarwiniaHeader {
    type Channel = tokio::sync::broadcast::Sender<Self>;
}
