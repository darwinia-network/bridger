use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use support_common::error::BridgerError;

/// Config helpers. store config to file or restore from file
#[derive(Debug)]
pub struct Config {
    base_path: PathBuf,
}

/// Config format
#[derive(Clone, Debug, Deserialize, Serialize, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum ConfigFormat {
    /// yml
    #[serde(rename = "yml")]
    Yml,
    /// json
    #[serde(rename = "json")]
    Json,
    /// toml
    #[serde(rename = "toml")]
    Toml,
}

impl ConfigFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            ConfigFormat::Yml => "yml",
            ConfigFormat::Json => "json",
            ConfigFormat::Toml => "toml",
        }
    }
}

impl Config {
    fn new() -> Self {
        let basic_path = dirs::home_dir()
            .or(std::env::current_exe().ok())
            .unwrap_or(std::env::temp_dir());
        let base_path = basic_path.join(".bridger");
        Self { base_path }
    }
}

impl Config {
    fn raw_config(config: impl Serialize, format: &ConfigFormat) -> color_eyre::Result<String> {
        let content = match format {
            ConfigFormat::Yml => serde_yaml::to_string(&config)?,
            ConfigFormat::Json => serde_json::to_string_pretty(&config)?,
            ConfigFormat::Toml => {
                let value = serde_json::to_value(&config)?;
                let value: toml::Value = serde_json::from_value(value)?;
                toml::to_string(&value)?
            }
        };
        tracing::trace!(target = "config", "raw config:\n {}", content);
        Ok(content)
    }
}

impl Config {
    /// Store config to file, the name argument is file name
    pub fn store(
        name: impl AsRef<str>,
        config: impl Serialize,
        format: ConfigFormat,
    ) -> color_eyre::Result<()> {
        let raw_config = Self::raw_config(config, &format)?;
        Self::new().persist(name, raw_config, format)
    }

    /// Restore config from file by name
    pub fn restore<T: DeserializeOwned>(name: impl AsRef<str>) -> color_eyre::Result<()> {
        Self::new().load(name)
    }
}

impl Config {
    fn persist(
        &self,
        name: impl AsRef<str>,
        config: String,
        format: ConfigFormat,
    ) -> color_eyre::Result<()> {
        if !self.base_path.exists() {
            std::fs::create_dir_all(&self.base_path)?;
        }
        let path = self
            .base_path
            .join(format!("{}.{}", name.as_ref(), format.extension()));
        std::fs::write(path, config)?;
        Ok(())
    }

    fn load<T: DeserializeOwned>(&self, name: impl AsRef<str>) -> color_eyre::Result<T> {
        if !self.base_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("The config path {:?} not found", &self.base_path),
            )
            .into());
        }

        let mut config_file = None;
        let read_dir = std::fs::read_dir(&self.base_path)?;
        for path in read_dir {
            let file = path?.path();
            if !file.is_file() {
                continue;
            }
            let file_name = match file.file_name() {
                Some(v) => match v.to_str() {
                    Some(z) => z.to_string(),
                    None => continue,
                },
                None => continue,
            };
            if file_name.starts_with(name.as_ref()) {
                config_file = Some(file);
                break;
            }
        }
        let path = config_file.ok_or(BridgerError::Config(format!(
            "Not found config file for name: {}",
            name.as_ref()
        )))?;

        let mut c = config::Config::default();
        c.merge(config::File::from(path.clone()))?;
        let tc = c.try_into::<T>().map_err(|e| {
            BridgerError::Config(format!(
                "Failed to load config: {:?} in path: {:?} for name {}",
                e,
                path,
                name.as_ref()
            ))
        })?;
        Ok(tc)
    }
}
