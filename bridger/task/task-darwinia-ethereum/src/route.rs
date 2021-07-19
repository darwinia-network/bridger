use bridge_traits::bridge::task::TaskTerminal;
use lifeline::{Bus, Receiver, Sender};
use crate::message::{ToRelayMessage, ToDarwiniaMessage};
use bridge_traits::error::StandardError;

use crate::bus::DarwiniaEthereumBus;
use lifeline::dyn_bus::DynBus;
use component_state::state::BridgeState;

pub async fn dispatch_route(
    bus: &DarwiniaEthereumBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "relay" => relay(bus, param).await,
        "set-darwinia-start" => set_darwinia_start(bus, param).await,
        "get-darwinia-start" => get_darwinia_start(bus, param).await,
        _ => Ok(TaskTerminal::new("Unsupported command")),
    }
}

async fn relay(bus: &DarwiniaEthereumBus, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToRelayMessage>()?;
    let block_number = param
        .get("block_number")
        .ok_or_else(|| StandardError::Api("The block_number is required".to_string()))?;
    let block_number = block_number.as_str().unwrap();
    sender
        .send(ToRelayMessage::EthereumBlockNumber(block_number.parse::<u64>().unwrap()))
        .await?;
    // todo: there can be upgrade config to set `auto_start=true`
    Ok(TaskTerminal::new("success"))
}

async fn set_darwinia_start(bus: &DarwiniaEthereumBus, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
    let state = bus.storage().clone_resource::<BridgeState>()?;
    let microkv = state.microkv();

    let block_number = param
        .get("block_number")
        .ok_or_else(|| StandardError::Api("The block_number is required".to_string()))?;
    let block_number = block_number.as_str().unwrap();

    microkv.put("last-tracked-darwinia-block", &block_number.parse::<u32>().unwrap());
    Ok(TaskTerminal::new("success"))
}

async fn get_darwinia_start(bus: &DarwiniaEthereumBus, _param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
    let state = bus.storage().clone_resource::<BridgeState>()?;
    let microkv = state.microkv();
    let block_number: u32 = microkv.get("last-tracked-darwinia-block")?.unwrap();

    println!("darwinia-start: {}", block_number);
    Ok(TaskTerminal::new("success"))
}
