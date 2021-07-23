use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;
use component_shadow::ShadowComponent;

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;

pub async fn affirm(
    _bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let block = param["block"]
        .as_u64()
        .ok_or_else(|| StandardError::Api("The `block` parameter is required".to_string()))?;
    let password = param["password"].as_str();

    let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;
    Ok(TaskTerminal::new("success"))
}
