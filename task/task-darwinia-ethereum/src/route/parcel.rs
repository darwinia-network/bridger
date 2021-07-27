use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;
use component_shadow::ShadowComponent;

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;

pub async fn show(
    _bus: &DarwiniaEthereumBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let output = param["output"]
        .as_str()
        .map(|item| item.to_lowercase())
        .unwrap_or_else(|| "raw".to_string());
    let block = param["block"]
        .as_u64()
        .ok_or_else(|| StandardError::Api("The `block` parameter is required".to_string()))?;

    let component_shadow = ShadowComponent::restore::<DarwiniaEthereumTask>()?;
    let shadow = component_shadow.component().await?;

    // Get parcel
    let parcel = shadow.parcel(block as usize).await?;
    match &output[..] {
        "json" => Ok(TaskTerminal::new(serde_json::to_string(&parcel)?)),
        _ => Ok(TaskTerminal::new(format!("{}", parcel))),
    }
}
