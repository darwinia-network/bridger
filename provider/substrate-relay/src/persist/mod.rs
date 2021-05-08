use std::path::PathBuf;

use getset::{Getters, MutGetters, Setters};
use typed_builder::TypedBuilder;

use crate::error;

#[derive(
	Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, MutGetters, Getters, Setters,
)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Persist {
	#[serde(default)]
	generic: Generic,
	#[serde(default)]
	chains: Vec<Chain>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, MutGetters, Getters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Generic {
	#[serde(skip_deserializing)]
	config_file: PathBuf,
	#[serde(default)]
	host: String,
	#[serde(default)]
	port: u32,
}

#[derive(
	Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, MutGetters, Getters, Setters,
)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Chain {
	#[serde(default)]
	name: String,
	#[serde(default)]
	host: String,
	#[serde(default)]
	port: u32,
	#[serde(default)]
	signer: String,
}

impl Default for Generic {
	fn default() -> Self {
		Self {
			config_file: Default::default(),
			host: "127.0.0.1".to_string(),
			port: 7890,
		}
	}
}

impl Persist {
	async fn init_file(config_file: &PathBuf) -> error::Result<()> {
		if !config_file.display().to_string().ends_with(".toml") {
			return Err(error::CliError::ConfigPathNotToml)?;
		}
		if !config_file.exists() {
			if let Some(parent) = config_file.parent() {
				if !parent.exists() {
					std::fs::create_dir_all(parent)?;
				}
			}
			async_fs::write(&config_file, "").await?;
		}
		if !config_file.is_file() {
			return Err(error::CliError::ConfigPathNotFile)?;
		}
		Ok(())
	}

	pub async fn load_from_file(config_file: PathBuf) -> error::Result<Self> {
		Persist::init_file(&config_file).await?;
		let toml_config = std::fs::read_to_string(&config_file)?;
		let mut persist: Persist = toml::from_str(&toml_config)?;
		let generic: &mut Generic = persist.generic_mut();
		generic.set_config_file(config_file);
		Ok(persist)
	}

	pub async fn store(&self) -> error::Result<&Self> {
		Persist::init_file(&self.generic.config_file).await?;

		// // reminder: https://github.com/alexcrichton/toml-rs/issues/142
		// // | error: values must be emitted before tables
		// // | if not have an change
		// let toml_text = toml::to_string_pretty(&self)?;
		// std::fs::write(&self.generic.config_file, toml_text)?;

		let json = serde_json::to_string(&self)?;
		let v: toml::Value = serde_json::from_str(&json)?;
		let toml_text: String = v.to_string();
		async_fs::write(&self.generic.config_file, toml_text).await?;

		Ok(self)
	}

	pub async fn chain_add(&mut self, chain: Chain) -> error::Result<&Self> {
		let mut chains = &mut self.chains;
		if chains.iter().any(|item| item.name == chain.name) {
			return Err(error::CliError::ChainNameExists)?;
		}
		chains.push(chain);
		self.store().await
	}
}
