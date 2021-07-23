#![allow(dead_code)]

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use once_cell::sync::{Lazy, OnceCell};

use bridge_traits::bridge::task::BridgeTaskKeep;
use bridge_traits::error::StandardError;

static AVAILABLE_TASKS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));

static mut RUNNING_TASKS: OnceCell<HashMap<String, Box<dyn BridgeTaskKeep + Send + Sync>>> =
    OnceCell::new();

pub fn add_available_tasks(names: Vec<impl AsRef<str>>) -> anyhow::Result<()> {
    for item in names {
        add_available_task(item)?;
    }
    Ok(())
}

pub fn add_available_task(name: impl AsRef<str>) -> anyhow::Result<()> {
    let mut tasks = AVAILABLE_TASKS
        .lock()
        .map_err(|_e| StandardError::Api("failed to get available task".to_string()))?;
    tasks.push(name.as_ref().to_string());
    Ok(())
}

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
    task: Box<dyn BridgeTaskKeep + Send + Sync>,
) -> anyhow::Result<()> {
    unsafe {
        if let Some(running) = RUNNING_TASKS.get_mut() {
            running.insert(name.as_ref().to_string(), task);
            Ok(())
        } else {
            let mut map = HashMap::new();
            map.insert(name.as_ref().to_string(), task);
            RUNNING_TASKS
                .set(map)
                .map_err(|_m| StandardError::Api("failed to init running task".to_string()))?;
            Ok(())
        }
    }
}

pub fn stop_task<N: AsRef<str>>(name: N) -> anyhow::Result<()> {
    let name = name.as_ref();
    unsafe {
        if let Some(runing) = RUNNING_TASKS.get_mut() {
            runing.remove(name).ok_or_else(|| {
                StandardError::Api(format!(
                    "not found this task: [{}]. maybe this task not started yet",
                    name
                ))
            })?;
        }
        Ok(())
    }
}

pub fn task_is_running<N: AsRef<str>>(name: N) -> bool {
    unsafe {
        RUNNING_TASKS
            .get()
            .map(|running| running.contains_key(name.as_ref()))
            .unwrap_or(false)
    }
}

#[allow(clippy::borrowed_box)]
pub fn running_task(
    name: impl AsRef<str>,
) -> Option<&'static Box<dyn BridgeTaskKeep + Send + Sync>> {
    let name = name.as_ref();
    unsafe {
        if let Some(running) = RUNNING_TASKS.get() {
            running.get(&name.to_string())
        } else {
            None
        }
    }
}

pub fn running_task_downcast_ref<T: 'static + BridgeTaskKeep>(
    name: impl AsRef<str>,
) -> anyhow::Result<&'static T> {
    let name = name.as_ref();
    unsafe {
        if let Some(running) = RUNNING_TASKS.get() {
            if let Some(tk) = running.get(&name.to_string()) {
                return match tk.as_any().downcast_ref::<T>() {
                    Some(b) => Ok(b),
                    None => {
                        Err(StandardError::Api(format!("can't downcast task [{}]", name)).into())
                    }
                };
            }
        }
        Err(StandardError::Api(format!("the task [{}] isn't started", name)).into())
    }
}

pub fn running_task_downcast_mut<T: 'static + BridgeTaskKeep>(
    name: impl AsRef<str>,
) -> anyhow::Result<&'static mut T> {
    let name = name.as_ref();
    unsafe {
        if let Some(running) = RUNNING_TASKS.get_mut() {
            if let Some(tk) = running.get_mut(&name.to_string()) {
                return match tk.as_any_mut().downcast_mut::<T>() {
                    Some(b) => Ok(b),
                    None => {
                        Err(StandardError::Api(format!("can't downcast task [{}]", name)).into())
                    }
                };
            }
        }
        Err(StandardError::Api(format!("the task [{}] isn't started", name)).into())
    }
}
