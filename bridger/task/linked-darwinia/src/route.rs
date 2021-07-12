use bridge_traits::error::StandardError;

use crate::bus::DarwiniaLinkedBus;

pub async fn dispatch_route(
    bus: &DarwiniaLinkedBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    match &uri[..] {
        "test" => {
            let ret = (uri, param);
            let value: serde_json::Value = serde_json::to_value(ret)?;
            Ok(value)
        }
        _ => Err(StandardError::Api("Not support this route".to_string()).into()),
    }
}
