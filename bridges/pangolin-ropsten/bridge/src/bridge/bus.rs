use lifeline::prelude::*;

use component_state::state::BridgeState;

lifeline_bus!(pub struct PangolinRopstenBus);

impl Resource<PangolinRopstenBus> for BridgeState {}
