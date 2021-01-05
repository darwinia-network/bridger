//! Bridger Config
use crate::error::{BizError, Result};
use etc::{Etc, Meta, Read, Write};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml::Serializer;

/// Ethereum Contract Tuple
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumContractTuple {
	/// Contract Address
	pub address: String,
	/// Contract Topic
	pub topics: Vec<String>,
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
}

/// Ethereum Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumConfig {
	/// Ethereum rpc url
	pub rpc: String,
	/// Ethereum contracts
	pub contract: EthereumContract,
}

/// Service step
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Step {
	/// Ethereum step
	pub ethereum: u64,
	/// Relay Step
	pub relay: u64,
	/// Redeem Step
	pub redeem: u64,
	/// Guard Step
	pub guard: u64,
}

/// Proxy config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Proxy {
	/// proxy real
	pub real: String,
}

/// Darwinia to ethereum config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DarwiniaToEthereum {
	/// ethereum seed
	pub seed: Option<String>,

	/// the darwinia account id who will get the reward
	/// Do not do submit authorities if None
	pub beneficiary: Option<String>,
}

/// Bridger Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
	/// Darwinia node url
	pub node: String,
	/// Darwinia account seed
	pub seed: String,
	/// Shadow service url
	pub shadow: String,
	/// Ethereum Config
	pub eth: EthereumConfig,
	/// Service steps
	pub step: Step,
	/// Darwinia relayer proxy address
	pub proxy: Option<Proxy>,
	/// darwinia_to_ethereum
	pub darwinia_to_ethereum: DarwiniaToEthereum,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			node: "wss://crab.darwinia.network".to_string(),
			seed: "//Alice".to_string(),
			shadow: "http://localhost:3000".to_string(),
			eth: EthereumConfig {
				rpc: "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
				contract: EthereumContract {
					ring: EthereumContractTuple {
						address: "0xb52FBE2B925ab79a821b261C82c5Ba0814AAA5e0".to_string(),
						topics: vec![
							"0xc9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10"
								.to_string(),
						],
					},
					kton: EthereumContractTuple {
						address: "0x1994100c58753793D52c6f457f189aa3ce9cEe94".to_string(),
						topics: vec![
							"0xc9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10"
								.to_string(),
						],
					},
					bank: EthereumContractTuple {
						address: "0x6EF538314829EfA8386Fc43386cB13B4e0A67D1e".to_string(),
						topics: vec![
							"0xe77bf2fa8a25e63c1e5e29e1b2fcb6586d673931e020c4e3ffede453b830fb12"
								.to_string(),
						],
					},
					issuing: EthereumContractTuple {
						address: "0x49262B932E439271d05634c32978294C7Ea15d0C".to_string(),
						topics: vec![],
					},
					relay: EthereumContractTuple {
						address: "0xc8d6c331030886716f6e323ACB795077eB530E36".to_string(),
						topics: vec![
							"0x91d6d149c7e5354d1c671fe15a5a3332c47a38e15e8ac0339b24af3c1090690f"
								.to_string(),
						],
					},
				},
			},
			step: Step {
				ethereum: 30,
				relay: 60,
				redeem: 90,
				guard: 30,
			},
			proxy: None,
			darwinia_to_ethereum: DarwiniaToEthereum {
				seed: None,
				beneficiary: None,
			},
		}
	}
}

impl Config {
	fn write(c: Etc) -> Result<Config> {
		let config = Config::default();
		let mut dst = String::with_capacity(128);
		config.serialize(Serializer::pretty(&mut dst).pretty_array(true))?;
		c.write(dst)?;
		Ok(config)
	}

	/// New config from pathbuf
	pub fn new(data_dir: &PathBuf) -> Result<Self> {
		let mut config_file = data_dir.clone();
		config_file.push("config.toml");
		let c = Etc::from(config_file);

		// if file exist
		if c.real_path()?.exists() {
			// if read fail, do not overwrite the exist one
			let mut config: Config = toml::from_slice(&c.read()?)?;

			// proxy real's length check
			if let Some(proxy) = config.clone().proxy {
				if proxy.real.len() != 64 && proxy.real.len() != 66 {
					return Err(BizError::Bridger(
						"Config proxy real's length is wrong".to_string(),
					)
					.into());
				}
				if proxy.real.len() == 64 {
					config.proxy = Some(Proxy {
						real: format!("0x{}", proxy.real),
					});
				}
			}

			Ok(config)
		} else {
			Self::write(c)
		}
	}

	/// default_data_dir
	pub fn default_data_dir() -> Result<PathBuf> {
		let mut dir = dirs::home_dir()
			.ok_or_else(|| BizError::Bridger("Could not open home dir".to_string()))?;
		dir.push(".bridger");

		Ok(dir)
	}
}
