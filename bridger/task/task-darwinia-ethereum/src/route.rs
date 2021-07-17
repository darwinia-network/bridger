use bridge_traits::bridge::task::TaskTerminal;
use lifeline::{Bus, Receiver, Sender};
use crate::message::ToRelayMessage;
use bridge_traits::error::StandardError;

use crate::bus::DarwiniaEthereumBus;

pub async fn dispatch_route(
    bus: &DarwiniaEthereumBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "relay" => relay(bus, param).await,
        _ => Ok(TaskTerminal::new("Unsupported command")),
    }
}

async fn relay(bus: &DarwiniaEthereumBus, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToRelayMessage>()?;
    let block_number = param
        .get("block_number")
        .ok_or_else(|| StandardError::Api("The block_number is required".to_string()))?;
    let block_number = block_number.as_str().unwrap_or("0");
    sender
        .send(ToRelayMessage::EthereumBlockNumber(block_number.parse::<u64>().unwrap()))
        .await?;
    // todo: there can be upgrade config to set `auto_start=true`
    Ok(TaskTerminal::new("success"))
}
