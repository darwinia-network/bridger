use std::str::FromStr;

use lifeline::{Bus, Receiver, Sender};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{BridgeSand, TaskTerminal};
use bridge_traits::error::StandardError;
use support_s2s::types::BridgeName;

use crate::bus::PangolinMillauBus;
use crate::config::PangolinMillauConfig;
use crate::message::{PangolinMillauMessageReceive, PangolinMillauMessageSend};
use crate::task::PangolinMillauTask;

pub async fn dispatch_route(
    bus: &PangolinMillauBus,
    uri: String,
    param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    match &uri[..] {
        "init-bridge" => init_bridge(bus, param).await,
        "start-relay" => start_relay(bus, param).await,
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

async fn start_relay(
    bus: &PangolinMillauBus,
    _param: serde_json::Value,
) -> anyhow::Result<TaskTerminal> {
    let mut sender = bus.tx::<PangolinMillauMessageSend>()?;
    sender
        .send(PangolinMillauMessageSend::Relay(
            BridgeName::PangolinToMillau,
        ))
        .await?;

    let state_task = support_keep::state::get_state_task_unwrap(PangolinMillauTask::NAME)?;
    let mut config_task: PangolinMillauConfig = Config::load(state_task.config_path.clone())?;
    let mut config_relay = config_task.relay;
    config_relay.auto_start = true;
    config_task.relay = config_relay;
    Config::persist(
        &state_task.config_path,
        config_task,
        state_task.config_format.clone(),
    )?;

    Ok(TaskTerminal::new("success"))
}
