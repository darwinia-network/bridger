#![allow(dead_code)]

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use once_cell::sync::{Lazy, OnceCell};

use bridge_shared::shared::BridgeShared;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask};
use bridge_standard::error::StandardError;
use task_darwinia_ethereum::task::DarwiniaEthereumTask;
use task_pangolin_millau::task::PangolinMillauTask;

static AVAILABLE_TASKS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(vec![
        DarwiniaEthereumTask::NAME.to_string(),
        PangolinMillauTask::NAME.to_string(),
    ])
});

static RUNNING_TASKS: Lazy<Mutex<HashMap<String, Box<dyn BridgeTask + Send>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static RUNNING_SHARED: OnceCell<BridgeShared> = OnceCell::new();

pub fn available_tasks() -> anyhow::Result<Vec<String>> {
    let tasks = AVAILABLE_TASKS
        .lock()
        .map_err(|_e| StandardError::Other("failed to get available task".to_string()))?;
    Ok(tasks.deref().clone())
}

pub fn keep_task<N: AsRef<str>>(name: N, task: Box<dyn BridgeTask + Send>) -> anyhow::Result<()> {
    let mut running = RUNNING_TASKS
        .lock()
        .map_err(|_e| StandardError::Other("failed to get running task".to_string()))?;
    running.insert(name.as_ref().to_string(), task);
    Ok(())
}

pub fn stop_task<N: AsRef<str>>(name: N) -> anyhow::Result<()> {
    let mut running = RUNNING_TASKS
        .lock()
        .map_err(|_e| StandardError::Other("failed to get running task".to_string()))?;
    let name = name.as_ref();
    running.remove(name).ok_or_else(|| {
        StandardError::Other(format!(
            "not found this task: [{}]. maybe this task not started yet",
            name
        ))
    })?;
    Ok(())
}

pub fn task_is_running<N: AsRef<str>>(name: N) -> anyhow::Result<bool> {
    let running = RUNNING_TASKS
        .lock()
        .map_err(|_e| StandardError::Other("failed to get running task".to_string()))?;
    Ok(running.contains_key(name.as_ref()))
}

pub fn set_shared(shared: BridgeShared) -> anyhow::Result<()> {
    RUNNING_SHARED
        .set(shared)
        .map_err(|_e| StandardError::Other("failed to save shared".to_string()))?;
    Ok(())
}

pub fn get_shared() -> Option<&'static BridgeShared> {
    RUNNING_SHARED.get()
}
