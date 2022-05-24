use lifeline::{Bus, Sender};

use crate::bridge::{BridgeTask, BridgeTaskMessage};

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!("Start bridge template");
    let task = BridgeTask::new()?;
    let stack = task.stack();
    let bus = stack.bus();

    Ok(())
}
