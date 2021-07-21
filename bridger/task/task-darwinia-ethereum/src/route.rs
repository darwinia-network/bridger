use bridge_traits::bridge::task::TaskTerminal;
use lifeline::{Bus, Sender};
use crate::message::{ToRelayMessage, ToDarwiniaMessage, ToEthereumMessage};
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
        "start-darwinia" => start_darwinia(bus, param).await,
        "start-ethereum" => start_ethereum(bus, param).await,
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

async fn start_darwinia(bus: &DarwiniaEthereumBus, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToDarwiniaMessage>()?;
    let block_number = param.get("block_number");
    let block_number = block_number.map(|b| {
        b.as_str().unwrap().parse::<u32>().unwrap()
    });

    if let Some(block_number) = block_number {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv();
        microkv.put("last-tracked-darwinia-block", &block_number)?;
    }

    sender.send(ToDarwiniaMessage::Start).await?;
    Ok(TaskTerminal::new("success"))
}

async fn start_ethereum(bus: &DarwiniaEthereumBus, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToEthereumMessage>()?;
    let block_number = param.get("block_number");
    let block_number = block_number.map(|b| {
        b.as_str().unwrap().parse::<u64>().unwrap()
    });

    if let Some(block_number) = block_number {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv();
        microkv.put("last-redeemed", &block_number)?;
    }

    sender.send(ToEthereumMessage::Start).await?;
    Ok(TaskTerminal::new("success"))
}
