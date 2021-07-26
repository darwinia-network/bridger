use bridge_traits::bridge::task::TaskTerminal;

use crate::bus::DarwiniaEthereumBus;

pub async fn guard(
    bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    Ok(TaskTerminal::new("success0"))
}
