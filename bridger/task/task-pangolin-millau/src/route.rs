use bridge_traits::bridge::task::TaskTerminal;

use crate::bus::PangolinMillauBus;

pub async fn dispatch_route(
    _bus: &PangolinMillauBus,
    _uri: String,
    _param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let value = TaskTerminal::new("Not have any command");
    Ok(value)
}
