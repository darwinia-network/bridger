use bridge_traits::bridge::task::TaskTerminal;

use crate::bus::PangolinMillauBus;

pub async fn dispatch_route(
    _bus: &PangolinMillauBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let value = TaskTerminal::new(format!("{} -> {:?}", uri, param));
    Ok(value)
}
