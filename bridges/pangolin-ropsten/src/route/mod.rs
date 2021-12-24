use bridge_traits::bridge::task::TaskTerminal;

use crate::bus::PangolinRopstenBus;

mod info;
mod redeem;
mod relay;

pub async fn dispatch_route(
    bus: &PangolinRopstenBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "relay" => relay::relay(bus, param).await,
        "redeem" => redeem::redeem(bus, param).await,
        "info-d2e" => info::d2e(bus, param).await,
        _ => Ok(TaskTerminal::new("Unsupported command")),
    }
}
