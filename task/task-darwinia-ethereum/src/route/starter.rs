use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Sender};

use bridge_traits::bridge::task::TaskTerminal;
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::message::{ToDarwiniaMessage, ToEthereumMessage};

pub async fn start_darwinia(
    bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToDarwiniaMessage>()?;
    let block_number = param.get("block_number");
    let block_number = block_number.map(|b| b.as_str().unwrap().parse::<u32>().unwrap());

    if let Some(block_number) = block_number {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv();
        microkv.put("last-tracked-darwinia-block", &block_number)?;
    }

    sender.send(ToDarwiniaMessage::Start).await?;
    Ok(TaskTerminal::new("success"))
}

pub async fn start_ethereum(
    bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<ToEthereumMessage>()?;
    let block_number = param.get("block_number");
    let block_number = block_number.map(|b| b.as_str().unwrap().parse::<u64>().unwrap());

    if let Some(block_number) = block_number {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv();
        microkv.put("last-redeemed", &block_number)?;
    }

    sender.send(ToEthereumMessage::Start).await?;
    Ok(TaskTerminal::new("success"))
}
