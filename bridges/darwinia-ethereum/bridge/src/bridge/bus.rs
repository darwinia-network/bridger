use lifeline::prelude::*;

use component_state::state::BridgeState;

lifeline_bus!(pub struct DarwiniaEthereumBus);

impl Resource<DarwiniaEthereumBus> for BridgeState {}
