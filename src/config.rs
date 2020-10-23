//! Bridger Config
use crate::result::{Error, Result};
use etc::{Etc, Meta, Read, Write};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml::Serializer;

/// Ethereum Contract Tuple
#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumContractTuple {
    /// Contract Address
    pub address: String,
    /// Contract Topic
    pub topics: Vec<String>,
}

/// Ethereum Contracts
#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumContract {
    /// Ring Contract
    pub ring: EthereumContractTuple,
    /// Kton Contract
    pub kton: EthereumContractTuple,
    /// Bank Contract
    pub bank: EthereumContractTuple,
    /// Issuing Contract
    pub issuing: EthereumContractTuple,
}

/// Ethereum Config
#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumConfig {
    /// Ethereum rpc url
    pub rpc: String,
    /// Ethereum start block number
    ///
    /// Ethereum bridger will scan start from this block
    pub start: u64,
    /// Ethereum contracts
    pub contract: EthereumContract,
}

/// Service step
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    /// proxy real
    pub real: String,
}

/// Bridger Config
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Darwinia node url
    pub node: String,
    /// Darwinia relayer proxy address
    pub proxy: Proxy,
    /// Darwinia account seed
    pub seed: String,
    /// Shadow service url
    pub shadow: String,
    /// Ethereum Config
    pub eth: EthereumConfig,
    /// Service steps
    pub step: Step,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            node: "wss://crab.darwinia.network".to_string(),
            proxy: Proxy {
                real: "".to_string()
            },
            seed: "//Alice".to_string(),
            shadow: "http://localhost:3000".to_string(),
            eth: EthereumConfig {
                rpc: "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
                start: 8647036,
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
                },
            },
            step: Step {
                ethereum: 30,
                relay: 60,
                redeem: 90,
                guard: 30,
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
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let c = Etc::from(if let Some(conf) = path {
            conf
        } else if let Some(mut conf) = dirs::home_dir() {
            conf.push(".darwinia/config.toml");
            conf
        } else {
            return Err(Error::Bridger("Could not open home dir".to_string()));
        });

        if !c.real_path()?.exists() {
            Self::write(c)
        } else if let Ok(config) = toml::from_slice(&c.read()?) {
            Ok(config)
        } else {
            Self::write(c)
        }
    }
}
