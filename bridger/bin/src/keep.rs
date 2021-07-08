#![allow(dead_code)]

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use once_cell::sync::{Lazy, OnceCell};

use bridge_traits::bridge::task::{BridgeSand, BridgeTaskKeep};
use bridge_traits::error::StandardError;
use component_state::state::BridgeState;
use linked_darwinia::task::DarwiniaLinked;
use task_darwinia_ethereum::task::DarwiniaEthereumTask;
use task_pangolin_millau::task::PangolinMillauTask;

static AVAILABLE_TASKS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(vec![
        DarwiniaEthereumTask::NAME.to_string(),
        PangolinMillauTask::NAME.to_string(),
        DarwiniaLinked::NAME.to_string(),
    ])
});

static RUNNING_TASKS: Lazy<Mutex<HashMap<String, Box<dyn BridgeTaskKeep + Send>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static BRIDGE_STATE: OnceCell<BridgeState> = OnceCell::new();

pub fn available_tasks() -> anyhow::Result<Vec<String>> {
    let tasks = AVAILABLE_TASKS
        .lock()
        .map_err(|_e| StandardError::Api("failed to get available task".to_string()))?;
    Ok(tasks.deref().clone())
}

pub fn is_available_task<S: AsRef<str>>(name: S) -> bool {
    match available_tasks() {
        Ok(tasks) => tasks.contains(&(name.as_ref().to_string())),
        Err(_) => false,
    }
}

pub fn keep_task<N: AsRef<str>>(
    name: N,
    task: Box<dyn BridgeTaskKeep + Send>,
) -> anyhow::Result<()> {
    let mut running = RUNNING_TASKS
        .lock()
        .map_err(|_e| StandardError::Api("failed to get running task".to_string()))?;
    running.insert(name.as_ref().to_string(), task);
    Ok(())
}

pub fn stop_task<N: AsRef<str>>(name: N) -> anyhow::Result<()> {
    let mut running = RUNNING_TASKS
        .lock()
        .map_err(|_e| StandardError::Api("failed to get running task".to_string()))?;
    let name = name.as_ref();
    running.remove(name).ok_or_else(|| {
        StandardError::Api(format!(
            "not found this task: [{}]. maybe this task not started yet",
            name
        ))
    })?;
    Ok(())
}

pub fn task_is_running<N: AsRef<str>>(name: N) -> bool {
    match RUNNING_TASKS.lock() {
        Ok(running) => running.contains_key(name.as_ref()),
        Err(_) => false,
    }
}

pub fn run_with_running_task<T, F>(name: &str, fnc: F) -> anyhow::Result<()>
where
    T: 'static + BridgeTaskKeep,
    F: FnOnce(&T) -> anyhow::Result<()>,
{
    let running = RUNNING_TASKS
        .lock()
        .map_err(|_e| StandardError::Api("failed to get running task".to_string()))?;
    if let Some(tk) = running.get(&name.to_string()) {
        return match tk.as_any().downcast_ref::<T>() {
            Some(b) => fnc(b),
            None => Err(StandardError::Api(format!("can't downcast task [{}]", name)).into()),
        };
    }
    Err(StandardError::Api(format!("the task [{}] isn't started", name)).into())
}

pub fn set_state(state: BridgeState) -> anyhow::Result<()> {
    BRIDGE_STATE
        .set(state)
        .map_err(|_e| StandardError::Api("Failed to keep bridge state".to_string()).into())
}

pub fn get_state() -> Option<BridgeState> {
    BRIDGE_STATE.get().cloned()
}
