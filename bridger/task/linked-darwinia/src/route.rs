use lifeline::Bus;

use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;

use crate::bus::DarwiniaLinkedBus;
use crate::message::DarwiniaLinkedMessage;

pub async fn dispatch_route(
    bus: &DarwiniaLinkedBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "test" => {
            let _sender = bus.tx::<DarwiniaLinkedMessage>()?;
            let _receiver = bus.rx::<DarwiniaLinkedMessage>()?;
            let value = TaskTerminal::new(format!("{} -> {:?}", uri, param));
            Ok(value)
        }
        _ => Err(StandardError::Api("Not support this route".to_string()).into()),
    }
}
