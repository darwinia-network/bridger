use lifeline::{Bus, Sender};

use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;

use crate::bus::DarwiniaEthereumBus;
use crate::message::ToRelayMessage;

pub async fn route(
    bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToRelayMessage>()?;
    let block_number = param
        .get("block_number")
        .ok_or_else(|| StandardError::Api("The block_number is required".to_string()))?;
    let block_number = block_number.as_str().unwrap();
    sender
        .send(ToRelayMessage::EthereumBlockNumber(
            block_number.parse::<u64>().unwrap(),
        ))
        .await?;

    Ok(TaskTerminal::new("success"))
}
