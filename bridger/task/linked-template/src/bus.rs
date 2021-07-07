use lifeline::prelude::*;

use bridge_component::state::BridgeState;

lifeline_bus!(pub struct TemplateLinkedBus);

impl Resource<TemplateLinkedBus> for BridgeState {}
