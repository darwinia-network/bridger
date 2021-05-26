//! Bridger Settings
use crate::encrypt_key;

use super::crypto::{Crypto, EncryptPrivateKey};
use crate::error::{BizError, Result};
use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

// - Ethereum --------------------------------
/// Ethereum Settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumSettings {
	/// Ethereum rpc url
	pub rpc: String,
	/// Ethereum contracts
	pub contract: EthereumContract,
	/// Ethereum Relayer
	pub relayer: Option<EthereumRelayer>,
	/// Ethereum Authority
	pub authority: Option<EthereumAuthority>,
}

/// Ethereum Contract Tuple
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumContractTuple {
	/// Contract Address
	pub address: String,
	/// Contract Topic
	pub topics: Option<Vec<String>>,
}

/// Ethereum Contracts
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumContract {
	/// Ring Contract
	pub ring: EthereumContractTuple,
	/// Kton Contract
	pub kton: EthereumContractTuple,
	/// Bank Contract
	pub bank: EthereumContractTuple,
	/// Issuing Contract
	pub issuing: EthereumContractTuple,
	/// Relay Contract
	pub relay: EthereumContractTuple,
	/// Backing Contract
	pub backing: EthereumContractTuple,
}

/// Ethereum Relayer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumRelayer {
	/// Private key
	pub private_key: String,
	/// ethereum.relayer's beneficiary account public key
	pub beneficiary_darwinia_account: String,
}
encrypt_key!(EthereumRelayer);

/// Ethereum Authority
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumAuthority {
	/// Private key to sign ecdsa signature
	pub private_key: String,
}
encrypt_key!(EthereumAuthority);

// - Darwinia --------------------------------
/// Darwinia Settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DarwiniaSettings {
	/// Darwinia rpc url
	pub rpc: String,
	/// Darwinia Relayer
	pub relayer: DarwiniaRelayer,
}

/// Darwinia Relayer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DarwiniaRelayer {
	/// Private key
	pub private_key: String,
	/// Real Account public key
	pub real_account: Option<String>,
}
encrypt_key!(DarwiniaRelayer);

// - Shadow --------------------------------
/// Shadow Settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShadowSettings {
	/// Shadow endpoint
	pub endpoint: String,
}

// - Services --------------------------------
/// Services Settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServicesSettings {
	/// Ethereum
	pub ethereum: ServicesEthereum,
	/// Relay
	pub relay: ServicesRelay,
	/// Redeem
	pub redeem: ServicesRedeem,
	/// Guard
	pub guard: ServicesGuard,
}

/// Services ethereum settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServicesEthereum {
	/// step
	pub step: u64,
}

/// Services relay settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServicesRelay {
	/// step
	pub step: u64,
}

/// Services redeem settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServicesRedeem {
	/// step
	pub step: u64,
}

/// Services guard settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServicesGuard {
	/// step
	pub step: u64,
}

/// Bridger Settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
	/// Encrypted
	pub encrypted: bool,

	/// Ethereum Settings
	pub ethereum: EthereumSettings,

	/// Darwinia Settings
	pub darwinia: DarwiniaSettings,

	/// Shadow Settings
	pub shadow: ShadowSettings,

	/// services
	pub services: ServicesSettings,
}

impl Settings {
	/// New settings from pathbuf
	pub fn new(data_dir: &Path) -> Result<Self> {
		let mut config_file = data_dir.to_path_buf();
		config_file.push("config.yml");

		let mut settings = Config::default();
		settings.merge(File::from(config_file))?;
		Ok(settings.try_into()?)
	}

	/// default_data_dir
	pub fn default_data_dir() -> Result<PathBuf> {
		let mut dir = dirs::home_dir()
			.ok_or_else(|| BizError::Bridger("Could not open home dir".to_string()))?;
		dir.push(".bridger");

		Ok(dir)
	}

	/// encrypt configure file
	pub fn encrypt(&mut self, passwd: &str) -> Result<()> {
		if !self.encrypted {
			let crypto = Crypto { salt: [0; 16] };
			if let Some(relayer) = &mut self.ethereum.relayer {
				relayer.encrypt(&crypto, &passwd)?;
			}
			if let Some(authority) = &mut self.ethereum.authority {
				authority.encrypt(&crypto, &passwd)?;
			}
			self.darwinia.relayer.encrypt(&crypto, &passwd)?;
			self.encrypted = true;
		}
		Ok(())
	}

	/// decrypt configure file
	pub fn decrypt(&mut self, passwd: &str) -> Result<()> {
		if self.encrypted {
			let crypto = Crypto { salt: [0; 16] };
			if let Some(relayer) = &mut self.ethereum.relayer {
				relayer.decrypt(&crypto, &passwd)?;
			}
			if let Some(authority) = &mut self.ethereum.authority {
				authority.decrypt(&crypto, &passwd)?;
			}
			self.darwinia.relayer.decrypt(&crypto, &passwd)?;
			self.encrypted = false;
		}
		Ok(())
	}
}

impl Default for Settings {
	fn default() -> Self {
		let config_file = PathBuf::from(".maintain/config/ropsten_pangolin.sample.yml");
		let mut settings = Config::default();
		settings.merge(File::from(config_file)).unwrap();
		settings.try_into().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_yaml_config() {
		let settings = Settings::default();
		println!("{:?}", settings.ethereum);
	}
}
