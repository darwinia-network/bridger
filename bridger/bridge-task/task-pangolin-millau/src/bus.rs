use lifeline::prelude::*;

use bridge_shared::shared::SharedChannel;

lifeline_bus!(pub struct PangolinMillauBus);

impl Resource<PangolinMillauBus> for SharedChannel {}
