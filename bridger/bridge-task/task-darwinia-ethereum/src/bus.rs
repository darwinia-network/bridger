use lifeline::prelude::*;

use bridge_shared::shared::SharedChannel;

lifeline_bus!(pub struct DarwiniaEthereumBus);

impl Resource<DarwiniaEthereumBus> for SharedChannel {}
