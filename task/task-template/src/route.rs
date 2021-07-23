use bridge_traits::bridge::task::TaskTerminal;

use crate::bus::TemplateTaskBus;

pub async fn dispatch_route(
    _bus: &TemplateTaskBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let value = TaskTerminal::new(format!("{} -> {:?}", uri, param));
    Ok(value)
}
