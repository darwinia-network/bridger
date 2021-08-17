use bridge_traits::bridge::task::TaskTerminal;

use crate::bus::PangolinRopstenBus;

mod info;
mod redeem;
mod relay;
mod starter;

pub async fn dispatch_route(
    bus: &PangolinRopstenBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "relay" => relay::relay(bus, param).await,
        "redeem" => redeem::redeem(bus, param).await,
        "start-ropsten" => starter::start_ropsten(bus, param).await,
        "info-d2e" => info::d2e(bus, param).await,
        _ => Ok(TaskTerminal::new("Unsupported command")),
    }
}
