#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use bridge_traits::error::StandardError;

static TASK_PASSWORD_MAP: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn put_task_config_password(
    task: impl AsRef<str>,
    password: impl AsRef<str>,
) -> anyhow::Result<()> {
    let mut map = TASK_PASSWORD_MAP
        .lock()
        .map_err(|_e| StandardError::Api("Failed to get task password instance".to_string()))?;
    map.insert(task.as_ref().to_string(), password.as_ref().to_string());
    Ok(())
}

pub fn remove_task_config_password(task: impl AsRef<str>) -> anyhow::Result<()> {
    let mut map = TASK_PASSWORD_MAP
        .lock()
        .map_err(|_e| StandardError::Api("Failed to get task password instance".to_string()))?;
    map.remove(task.as_ref());
    Ok(())
}

pub fn get_task_config_password(task: impl AsRef<str>) -> anyhow::Result<Option<String>> {
    let map = TASK_PASSWORD_MAP
        .lock()
        .map_err(|_e| StandardError::Api("Failed to get task password instance".to_string()))?;
    Ok(map.get(task.as_ref()).cloned())
}
