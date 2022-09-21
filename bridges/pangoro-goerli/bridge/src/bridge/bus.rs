use component_state::state::BridgeState;
use lifeline::prelude::*;

lifeline_bus!(pub struct BridgeBus);

impl Resource<BridgeBus> for BridgeState {}
