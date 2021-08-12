use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Sender};

use bridge_traits::bridge::task::{BridgeSand, TaskTerminal};
use component_state::state::BridgeState;

use crate::bus::PangolinRopstenBus;
use crate::message::{ToDarwiniaMessage, ToEthereumMessage};
use crate::task::PangolinRopstenTask;

pub async fn start_pangolin(
    bus: &PangolinRopstenBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToDarwiniaMessage>()?;
    let block_number = param.get("block_number");
    let block_number = block_number.map(|b| b.as_str().unwrap().parse::<u32>().unwrap());

    if let Some(block_number) = block_number {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        microkv.put("last-tracked-pangolin-block", &block_number)?;
    }

    sender.send(ToDarwiniaMessage::Start).await?;
    Ok(TaskTerminal::new("success"))
}

pub async fn start_ropsten(
    bus: &PangolinRopstenBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToEthereumMessage>()?;
    let block_number = param.get("block_number");
    let block_number = block_number.map(|b| b.as_str().unwrap().parse::<u64>().unwrap());

    if let Some(block_number) = block_number {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        microkv.put("last-redeemed-ropsten", &block_number)?;
    }

    sender.send(ToEthereumMessage::Start).await?;
    Ok(TaskTerminal::new("success"))
}
