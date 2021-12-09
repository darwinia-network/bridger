use std::str::FromStr;

use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::{BridgeConfig, Config};
use component_subscan::SubscanConfig;

use crate::types::{ChainInfo, HexLaneId, PrometheusParamsInfo};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaCrabConfig {
    pub darwinia: ChainInfoConfig,
    pub crab: ChainInfoConfig,
    pub relay: RelayConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darwinia_subscan: Option<SubscanConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crab_subscan: Option<SubscanConfig>,
    pub task: TaskConfig,
}

impl DarwiniaCrabConfig {
    pub fn store<S: AsRef<str>>(&self, sand_name: S) -> anyhow::Result<()> {
        self.darwinia.check()?;
        self.crab.check()?;
        let name = sand_name.as_ref();
        Config::store_with_namespace(name, self.darwinia.clone(), "darwinia")?;
        Config::store_with_namespace(name, self.crab.clone(), "crab")?;
        if let Some(darwinia_subscan) = &self.darwinia_subscan {
            Config::store_with_namespace(name, darwinia_subscan.clone(), "darwinia")?;
        }
        if let Some(crab_subscan) = &self.crab_subscan {
            Config::store_with_namespace(name, crab_subscan.clone(), "crab")?;
        }
        Config::store(name, self.task.clone())?;
        Config::store(name, self.relay.clone())?;
        Ok(())
    }
    pub fn template() -> Self {
        Self {
            darwinia: ChainInfoConfig::template(),
            crab: ChainInfoConfig::template(),
            relay: RelayConfig::template(),
            darwinia_subscan: Some(SubscanConfig::template()),
            crab_subscan: Some(SubscanConfig::template()),
            task: TaskConfig::template(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskConfig {
    pub interval_update_fee: u64,
    pub update_fee_strategy: UpdateFeeStrategyType,
}

impl BridgeConfig for TaskConfig {
    fn marker() -> &'static str {
        "config-task-darwinia-crab"
    }

    fn template() -> Self {
        Self {
            interval_update_fee: 60,
            update_fee_strategy: UpdateFeeStrategyType::Nothing,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, strum::EnumString)]
pub enum UpdateFeeStrategyType {
    Nothing,
    Crazy,
    Reasonable,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelayConfig {
    /// Hex-encoded lane identifiers that should be served by the complex relay.
    pub lanes: Vec<HexLaneId>,
    pub auto_start: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_darwinia: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_crab: Option<String>,
    #[serde(default)]
    pub prometheus_params: PrometheusParamsInfo,
    /// If passed, only mandatory headers (headers that are changing the GRANDPA authorities set)
    /// are relayed.
    pub only_mandatory_headers: bool,
    /// Create relayers fund accounts on both chains, if it does not exists yet.
    pub create_relayers_fund_accounts: bool,
    /// The SURI of secret key to use when transactions are submitted to the darwinia node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darwinia_messages_pallet_owner: Option<String>,
    /// The password for the SURI of secret key to use when transactions are submitted to the darwinia node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darwinia_messages_pallet_owner_password: Option<String>,
    /// The SURI of secret key to use when transactions are submitted to the crab node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crab_messages_pallet_owner: Option<String>,
    /// The password for the SURI of secret key to use when transactions are submitted to the crab node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crab_messages_pallet_owner_password: Option<String>,
}

impl BridgeConfig for RelayConfig {
    fn marker() -> &'static str {
        "config-relay"
    }

    fn template() -> Self {
        Self {
            lanes: vec![HexLaneId::from_str("00000000").unwrap()],
            auto_start: false,
            signer_darwinia: Some("//Alice".to_string()),
            signer_crab: Some("//Alice".to_string()),
            prometheus_params: PrometheusParamsInfo {
                no_prometheus: false,
                prometheus_host: "127.0.0.1".to_string(),
                prometheus_port: 9616,
            },
            only_mandatory_headers: false,
            create_relayers_fund_accounts: false,
            darwinia_messages_pallet_owner: Some("//CrabMessagesOwner".to_string()),
            darwinia_messages_pallet_owner_password: Some("123456".to_string()),
            crab_messages_pallet_owner: Some("//DarwiniaMessagesOwner".to_string()),
            crab_messages_pallet_owner_password: Some("123456".to_string()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfoConfig {
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<String>,
    #[serde(skip)]
    pub secure: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_password: Option<String>,
    /// Transactions mortality period, in blocks. MUST be a power of two in [4; 65536] range. MAY NOT be larger than `BlockHashCount` parameter of the chain system module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions_mortality: Option<u32>,
}

impl BridgeConfig for ChainInfoConfig {
    fn marker() -> &'static str {
        "s2s-chain-info"
    }

    fn template() -> Self {
        Self {
            endpoint: "ws://127.0.0.1:9544".to_string(),
            signer: Some("//Alice".to_string()),
            secure: false,
            signer_password: None,
            transactions_mortality: None,
        }
    }
}

impl ChainInfoConfig {
    pub fn check(&self) -> anyhow::Result<()> {
        self.host_port()?;
        Ok(())
    }

    fn host_port(&self) -> anyhow::Result<(bool, String, u16)> {
        if self.endpoint.find("ws://").unwrap_or(usize::MAX) != 0
            && self.endpoint.find("wss://").unwrap_or(usize::MAX) != 0
        {
            anyhow::bail!("The entrypoint isn't websocket protocol")
        }
        let secure = self.endpoint.starts_with("wss://");
        let endpoint = self
            .endpoint
            .replace(if secure { "wss://" } else { "ws://" }, "")
            .replace('/', "")
            .replace(' ', "");
        let host_port = endpoint.split(':').collect::<Vec<&str>>();
        let host = host_port.get(0).unwrap_or(&"127.0.0.1");
        let port = host_port
            .get(1)
            .unwrap_or_else(|| if secure { &"443" } else { &"80" });
        Ok((secure, host.to_string(), port.parse::<u16>()?))
    }

    pub fn secure(&self) -> anyhow::Result<bool> {
        Ok(self.host_port()?.0)
    }

    pub fn host(&self) -> anyhow::Result<String> {
        Ok(self.host_port()?.1)
    }

    pub fn port(&self) -> anyhow::Result<u16> {
        Ok(self.host_port()?.2)
    }

    pub fn to_chain_info(&self) -> anyhow::Result<ChainInfo> {
        self.to_chain_info_with_expect_signer(None)
    }

    pub fn to_chain_info_with_expect_signer(
        &self,
        except_signer: Option<String>,
    ) -> anyhow::Result<ChainInfo> {
        let host_port = self.host_port()?;
        Ok(ChainInfo {
            secure: host_port.0,
            host: host_port.1,
            port: host_port.2,
            signer: except_signer.or_else(|| self.signer.clone()),
            signer_password: self.signer_password.clone(),
            transactions_mortality: Some(256),
        })
    }
}
