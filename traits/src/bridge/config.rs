use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use strum::AsStaticRef;

use crate::error::{BridgeResult, StandardError};

pub trait BridgeConfig {
    fn marker() -> &'static str;
    fn template() -> Self;
}

static INSTANCE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug, Deserialize, Serialize, strum::EnumString, strum::AsStaticStr)]
pub enum ConfigFormat {
    #[strum(serialize = "yml")]
    Yml,
    #[strum(serialize = "json")]
    Json,
    #[strum(serialize = "toml")]
    Toml,
}

impl ConfigFormat {
    pub fn file_extension(&self) -> &'static str {
        self.as_static()
    }
}

pub struct Config;

const DEFAULT_NAMESPACE: &str = "default";

impl Config {
    pub fn default_namespace() -> &'static str {
        DEFAULT_NAMESPACE
    }

    pub fn store<S: AsRef<str>, B: BridgeConfig + Serialize>(
        sand_name: S,
        config: B,
    ) -> BridgeResult<()> {
        Self::store_with_namespace(sand_name, config, DEFAULT_NAMESPACE)
    }

    pub fn store_with_namespace<S: AsRef<str>, B: BridgeConfig + Serialize, N: AsRef<str>>(
        sand_name: S,
        config: B,
        namespace: N,
    ) -> BridgeResult<()> {
        let config_marker = B::marker();
        let key = format!(
            "{}:{}@{}",
            sand_name.as_ref(),
            config_marker,
            namespace.as_ref()
        );

        let json = serde_json::to_string(&config).map_err(|e| {
            StandardError::Other(format!(
                "Te config cannot be serialize, lease check it. [{}] {:?}",
                key, e
            ))
        })?;
        let _mutex = INSTANCE.lock().unwrap().insert(key, json);
        Ok(())
    }

    pub fn restore<S: AsRef<str>, B: BridgeConfig + DeserializeOwned>(
        sand_name: S,
    ) -> BridgeResult<B> {
        Self::restore_with_namespace(sand_name, DEFAULT_NAMESPACE)
    }

    pub fn restore_with_namespace<
        S: AsRef<str>,
        B: BridgeConfig + DeserializeOwned,
        N: AsRef<str>,
    >(
        sand_name: S,
        namespace: N,
    ) -> BridgeResult<B> {
        let config_marker = B::marker();
        let key = format!(
            "{}:{}@{}",
            sand_name.as_ref(),
            config_marker,
            namespace.as_ref()
        );
        match INSTANCE.lock().unwrap().get(&key) {
            Some(v) => serde_json::from_str(v).map_err(|e| {
                StandardError::Other(format!(
                    "The config cannot be deserialize, please check it. [{}] {:?}",
                    key, e
                ))
            }),
            None => Err(StandardError::NotSupport(format!(
                "Not support this config, please init this config after create task -> [{}]",
                key
            ))),
        }
    }

    pub fn raw_config(config: impl Serialize, format: ConfigFormat) -> anyhow::Result<String> {
        let content = match format {
            ConfigFormat::Yml => serde_yaml::to_string(&config)?,
            ConfigFormat::Json => serde_json::to_string_pretty(&config)?,
            ConfigFormat::Toml => {
                let value = serde_json::to_value(&config)?;
                let value: toml::Value = serde_json::from_value(value)?;
                toml::to_string(&value)?
            }
        };
        Ok(content)
    }

    pub fn persist_raw(
        path_config: impl AsRef<Path>,
        config: impl AsRef<str>,
    ) -> anyhow::Result<()> {
        let path = path_config.as_ref();
        std::fs::write(path, config.as_ref())?;
        log::info!("The config [{:?}] persisted", path);
        Ok(())
    }

    pub fn persist(
        path_config: impl AsRef<Path>,
        config: impl Serialize,
        format: ConfigFormat,
    ) -> anyhow::Result<()> {
        let content = Self::raw_config(config, format)?;
        Self::persist_raw(path_config, content)
    }

    pub fn load<T>(path_config: impl AsRef<Path>) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = path_config.as_ref();
        let mut c = config::Config::default();
        c.merge(config::File::from(path))?;
        let tc = c.try_into::<T>().map_err(|e| {
            StandardError::Api(format!(
                "Failed to load task config: {:?} path: {:?}",
                e, path
            ))
        })?;
        Ok(tc)
    }
}
