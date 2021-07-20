use once_cell::sync::{Lazy, OnceCell};

use bridge_traits::error::StandardError;
use component_state::state::BridgeState;

use crate::types::{TaskState, WebserverState};
use std::collections::HashMap;
use std::sync::Mutex;

static STATE_BRIDGE: OnceCell<BridgeState> = OnceCell::new();

pub fn set_state_bridge(state: BridgeState) -> anyhow::Result<()> {
    STATE_BRIDGE
        .set(state)
        .map_err(|_e| StandardError::Api("Failed to keep bridge state".to_string()).into())
}

pub fn get_state_bridge() -> Option<&'static BridgeState> {
    STATE_BRIDGE.get()
}

pub fn get_state_bridge_ok() -> anyhow::Result<&'static BridgeState> {
    get_state_bridge()
        .ok_or_else(|| StandardError::Api("Please set bridge state first.".to_string()).into())
}

static STATE_WEBSITE: OnceCell<WebserverState> = OnceCell::new();

pub fn set_state_website(state: WebserverState) -> anyhow::Result<()> {
    STATE_WEBSITE
        .set(state)
        .map_err(|_e| StandardError::Api("Failed to keep website state".to_string()).into())
}

pub fn get_state_website() -> Option<WebserverState> {
    STATE_WEBSITE.get().cloned()
}

pub fn get_state_website_unwrap() -> WebserverState {
    get_state_website()
        .ok_or_else(|| StandardError::Api("Please set website state first.".to_string()))
        .unwrap()
}

static STATE_TASK: Lazy<Mutex<HashMap<String, TaskState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn set_state_task(task: impl AsRef<str>, state: TaskState) -> anyhow::Result<()> {
    let mut state_task = STATE_TASK
        .lock()
        .map_err(|_e| StandardError::Api("failed to get task state".to_string()))?;
    state_task.insert(task.as_ref().to_string(), state);
    Ok(())
}

pub fn get_state_task(task: impl AsRef<str>) -> anyhow::Result<Option<TaskState>> {
    let state_task = STATE_TASK
        .lock()
        .map_err(|_e| StandardError::Api("failed to get task state".to_string()))?;
    Ok(state_task.get(task.as_ref()).cloned())
}

pub fn get_state_task_unwrap(task: impl AsRef<str>) -> anyhow::Result<TaskState> {
    match get_state_task(task) {
        Ok(v) => match v {
            Some(t) => Ok(t),
            None => Err(StandardError::Api("failed to get task state".to_string()).into()),
        },
        Err(e) => Err(e),
    }
}
