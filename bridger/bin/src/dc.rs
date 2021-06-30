#![allow(dead_code)]
use std::sync::Mutex;

use once_cell::sync::Lazy;

use bridge_standard::bridge::task::BridgeSand;
use std::ops::Deref;
use task_darwinia_ethereum::task::DarwiniaEthereumTask;
use task_pangolin_millau::task::PangolinMillauTask;

static AVAILABLE_TASKS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(vec![
        DarwiniaEthereumTask::NAME.to_string(),
        PangolinMillauTask::NAME.to_string(),
    ])
});

pub fn available_tasks() -> anyhow::Result<Vec<String>> {
    let tasks = AVAILABLE_TASKS.lock().unwrap();
    Ok(tasks.deref().clone())
}

pub fn init() -> anyhow::Result<()> {
    Ok(())
}
