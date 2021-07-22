use lifeline::prelude::*;

use component_state::state::BridgeState;

lifeline_bus!(pub struct TemplateLinkedBus);

impl Resource<TemplateLinkedBus> for BridgeState {}
