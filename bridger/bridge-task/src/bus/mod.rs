use lifeline::prelude::*;

pub use self::messages::*;

mod messages;

lifeline_bus!(pub struct DarwiniaEthereumBus);
