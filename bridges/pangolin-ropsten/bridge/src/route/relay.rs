use lifeline::{Bus, Sender};

use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;

use crate::bridge::PangolinRopstenBus;
use crate::bridge::ToRelayMessage;

pub async fn relay(
    bus: &PangolinRopstenBus,
    param: serde_json::Value,
) -> color_eyre::Result<TaskTerminal> {
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
