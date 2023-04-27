use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use support_types::constants;

use crate::error::BridgerError;

/// The config names
#[derive(Clone, Debug)]
pub enum Names {
    /// Bridger
    Bridger,
    /// Bridge tempalte
    BridgeTemplate,
    /// Bridge darwinia-ethereum
    BridgeDarwiniaEthereum,
    /// Bridge pangolin-pangoro
    BridgePangolinPangoro,
    /// bridge darwinia-crab
    BridgeDarwiniaCrab,
    /// bridge pangoro-chapel
    BridgePangoroChapel,
    /// bridge pangolin-goerli
    BridgePangolinGoerli,
}

impl Names {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bridger => "bridger",
            Self::BridgeTemplate => "bridge-template",
            Self::BridgeDarwiniaEthereum => "bridge-darwinia-ethereum",
            Self::BridgePangolinPangoro => "bridge-pangolin-pangoro",
            Self::BridgeDarwiniaCrab => "bridge-darwinia-crab",
            Self::BridgePangoroChapel => "bridge-pangoro-chapel",
            Self::BridgePangolinGoerli => "bridge-pangolin-goerli",
        }
    }
}

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
        let base_path = constants::bridger_home();
        Self { base_path }
    }
}

impl Config {
    /// Store without file format, if the config is exists will be replace it.
    /// If not choose toml default.
    pub fn store(name: Names, config: impl Serialize) -> Result<(), BridgerError> {
        Self::new().persist(name.name(), config, None)
    }

    /// Store config to file, the name argument is file name
    pub fn store_with_format(
        name: Names,
        config: impl Serialize,
        format: ConfigFormat,
    ) -> Result<(), BridgerError> {
        Self::new().persist(name.name(), config, Some(format))
    }

    /// Restore config from file by name
    pub fn restore<T: DeserializeOwned>(name: Names) -> Result<T, BridgerError> {
        Self::new().load(name.name())
    }

    /// The config file is exists
    pub fn exists(name: Names) -> bool {
        Self::new()
            .find_config_file(name.name())
            .unwrap_or_default()
            .is_some()
    }
}

impl Config {
    fn raw_config(
        &self,
        config: impl Serialize,
        format: &ConfigFormat,
    ) -> Result<String, BridgerError> {
        let content = match format {
            ConfigFormat::Yml => serde_yaml::to_string(&config)?,
            ConfigFormat::Json => serde_json::to_string_pretty(&config)?,
            ConfigFormat::Toml => {
                let value = serde_json::to_value(&config)?;
                let value: toml::Value = serde_json::from_value(value)?;
                toml::to_string(&value)?
            }
        };
        // This is danger log output
        // tracing::trace!(target: "config", "raw config: \n{}", content);
        Ok(content)
    }

    fn find_config_file(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<(PathBuf, ConfigFormat)>, BridgerError> {
        let mut config_file = None;
        if !self.base_path.exists() {
            tracing::warn!(target: "bridger", "The base_path ({}) is not found.", self.base_path.display());
            return Ok(None);
        }
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
        match config_file {
            Some(v) => {
                let extension = v.extension().and_then(|v| v.to_str()).and_then(|s| {
                    match &s.to_lowercase()[..] {
                        "toml" => Some(ConfigFormat::Toml),
                        "json" => Some(ConfigFormat::Json),
                        "yml" => Some(ConfigFormat::Yml),
                        _ => None,
                    }
                });
                match extension {
                    Some(e) => Ok(Some((v, e))),
                    None => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    fn persist(
        &self,
        name: impl AsRef<str>,
        config: impl Serialize,
        format: Option<ConfigFormat>,
    ) -> Result<(), BridgerError> {
        if !self.base_path.exists() {
            std::fs::create_dir_all(&self.base_path)?;
        }
        let format = format.unwrap_or(
            self.find_config_file(name.as_ref())?
                .map(|(_, format)| format)
                .unwrap_or(ConfigFormat::Toml),
        );

        let config = self.raw_config(config, &format)?;
        let path = self
            .base_path
            .join(format!("{}.{}", name.as_ref(), format.extension()));
        std::fs::write(path, config)?;
        Ok(())
    }

    fn load<T: DeserializeOwned>(&self, name: impl AsRef<str>) -> Result<T, BridgerError> {
        if !self.base_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("The config path {:?} not found", &self.base_path),
            )
            .into());
        }

        let (path, _) = self.find_config_file(name.as_ref())?.ok_or_else(|| {
            BridgerError::Config(format!(
                "Not found config file for name: {} in path: {}",
                name.as_ref(),
                self.base_path.display()
            ))
        })?;
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
