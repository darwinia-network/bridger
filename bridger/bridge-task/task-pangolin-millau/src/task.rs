use lifeline::dyn_bus::DynBus;

use bridge_shared::shared::SharedChannel;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask};
use serde::{Deserialize, Serialize};

use crate::bus::PangolinMillauBus;

#[derive(Debug)]
pub struct PangolinMillauTask {}

impl BridgeTask for PangolinMillauTask {}

impl BridgeSand for PangolinMillauTask {
    const NAME: &'static str = "task-pangolin-millau";
}

impl PangolinMillauTask {
    pub async fn new(config: PangolinMillauConfig, channel: SharedChannel) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = PangolinMillauBus::default();
        bus.store_resource::<SharedChannel>(channel);
        // todo: millau <-> pangolin start
        debug!("create task-pangolin-millau");
        Ok(Self {})
    }
}

// -- config --

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinMillauConfig {}

impl PangolinMillauConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        let _name = cell_name.as_ref();
        Ok(())
    }
}
