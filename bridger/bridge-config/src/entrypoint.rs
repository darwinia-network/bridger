use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::{Lazy, OnceCell};

use bridge_standard::bridge::config::BridgeConfig;
use bridge_standard::error::{BridgeResult, StandardError};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::sync::Arc;

static INSTANCE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub struct Config;

impl Config {
    pub fn store<S: AsRef<str>, B: BridgeConfig + Serialize>(
        task_name: S,
        config: B,
    ) -> BridgeResult<()> {
        let config_marker = B::marker();
        let key = format!("{}:{}", task_name.as_ref(), config_marker);

        let json = serde_json::to_string(&config).map_err(|e| {
            StandardError::Other(format!(
                "Te config cannot be serialize, lease check it. {:?}",
                e
            ))
        })?;
        let _mutex = INSTANCE.lock().unwrap().insert(key, json);
        Ok(())
    }

    pub fn restore<S: AsRef<str>, B: BridgeConfig + DeserializeOwned>(
        task_name: S,
    ) -> BridgeResult<B> {
        let config_marker = B::marker();
        let key = format!("{}:{}", task_name.as_ref(), config_marker);
        match INSTANCE.lock().unwrap().get(&key) {
            Some(v) => serde_json::from_str(v).map_err(|e| {
                StandardError::Other(format!(
                    "The config cannot be deserialize, please check it. {:?}",
                    e
                ))
            }),
            None => Err(StandardError::NotSupport(
                "Not support this config, please init this config after create task".to_string(),
            )),
        }
    }
}
