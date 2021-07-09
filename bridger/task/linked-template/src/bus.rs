use std::ops::Deref;

use lifeline::prelude::*;
use once_cell::sync::OnceCell;

use component_state::state::BridgeState;

static BUS: OnceCell<TemplateLinkedBus> = OnceCell::new();

pub(crate) fn bus() -> &'static TemplateLinkedBus {
    BUS.get_or_init(|| Default::default())
}

lifeline_bus!(pub struct TemplateLinkedBus);

impl Resource<TemplateLinkedBus> for BridgeState {}
