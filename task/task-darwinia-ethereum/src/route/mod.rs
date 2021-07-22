use bridge_traits::bridge::task::TaskTerminal;

use crate::bus::DarwiniaEthereumBus;

mod relay;
mod starter;

pub async fn dispatch_route(
    bus: &DarwiniaEthereumBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "relay" => relay::route(bus, param).await,
        "start-darwinia" => starter::start_darwinia(bus, param).await,
        "start-ethereum" => starter::start_ethereum(bus, param).await,
        _ => Ok(TaskTerminal::new("Unsupported command")),
    }
}
