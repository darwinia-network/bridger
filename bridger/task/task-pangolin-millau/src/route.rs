use std::str::FromStr;

use lifeline::{Bus, Receiver, Sender};

use bridge_traits::bridge::task::TaskTerminal;
use bridge_traits::error::StandardError;
use support_s2s::types::BridgeName;

use crate::bus::PangolinMillauBus;
use crate::message::{PangolinMillauMessageReceive, PangolinMillauMessageSend};

pub async fn dispatch_route(
    bus: &PangolinMillauBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "init-bridge" => init_bridge(bus, param).await,
        "start-relay" => relay(bus, param).await,
        _ => Ok(TaskTerminal::new("Unsupported command")),
    }
}

fn bridge_name_from_param(param: &serde_json::Value) -> anyhow::Result<BridgeName> {
    let bridge_value = param
        .get("bridge")
        .ok_or_else(|| StandardError::Api("The bridge is required".to_string()))?;
    let bridge_text = bridge_value
        .as_str()
        .ok_or_else(|| StandardError::Api("Failed to get bridge".to_string()))?;
    BridgeName::from_str(bridge_text).map_err(|_e| {
        StandardError::Api(format!("Not support this bridge: {}", bridge_text)).into()
    })
}

#[allow(clippy::never_loop)]
async fn init_bridge(
    bus: &PangolinMillauBus,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let bridge_name = bridge_name_from_param(&param)?;
    let mut sender = bus.tx::<PangolinMillauMessageSend>()?;
    let mut receiver = bus.rx::<PangolinMillauMessageReceive>()?;

    sender
        .send(PangolinMillauMessageSend::InitBridge(bridge_name.clone()))
        .await?;

    while let Some(recv) = receiver.recv().await {
        match recv {
            PangolinMillauMessageReceive::FinishedInitBridge => break,
        }
    }

    Ok(TaskTerminal::new(format!(
        "init bridge {:?} success",
        bridge_name
    )))
}

async fn relay(bus: &PangolinMillauBus, _param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<PangolinMillauMessageSend>()?;
    sender
        .send(PangolinMillauMessageSend::Relay(
            BridgeName::PangolinToMillau,
        ))
        .await?;
    // todo: there can be upgrade config to set `auto_start=true`
    Ok(TaskTerminal::new("success"))
}
