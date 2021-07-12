use crate::bus::DarwiniaEthereumBus;

pub async fn dispatch_route(
    bus: &DarwiniaEthereumBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    let ret = (uri, param);
    let value: serde_json::Value = serde_json::to_value(ret)?;
    Ok(value)
}
