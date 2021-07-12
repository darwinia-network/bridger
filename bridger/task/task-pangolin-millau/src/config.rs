use std::convert::TryFrom;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::{BridgeConfig, Config};
use support_s2s::types::{ChainInfo, HexLaneId, PrometheusParamsInfo};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinMillauConfig {
    pangolin: ChainInfoConfig,
    millau: ChainInfoConfig,
    relay: RelayConfig,
}

impl PangolinMillauConfig {
    pub fn store<S: AsRef<str>>(&self, sand_name: S) -> anyhow::Result<()> {
        self.pangolin.check()?;
        self.millau.check()?;
        let name = sand_name.as_ref();
        Config::store_with_namespace(name, self.pangolin.clone(), "pangolin")?;
        Config::store_with_namespace(name, self.millau.clone(), "millau")?;
        Config::store(name, self.relay.clone())?;
        Ok(())
    }
    pub fn template() -> Self {
        Self {
            pangolin: ChainInfoConfig::template(),
            millau: ChainInfoConfig::template(),
            relay: RelayConfig::template(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelayConfig {
    pub lanes: Vec<HexLaneId>,
    #[serde(default)]
    pub prometheus_params: PrometheusParamsInfo,
}

impl BridgeConfig for RelayConfig {
    fn marker() -> &'static str {
        "config-relay"
    }

    fn template() -> Self {
        Self {
            lanes: vec![HexLaneId::from_str("00000000").unwrap()],
            prometheus_params: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfoConfig {
    pub endpoint: String,
    pub signer: Option<String>,
    #[serde(skip)]
    pub secure: bool,
    pub signer_password: Option<String>,
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
            signer_password: Some("".to_string()),
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
            .replace("/", "")
            .replace(" ", "");
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
}

impl TryFrom<ChainInfoConfig> for ChainInfo {
    type Error = anyhow::Error;

    fn try_from(from: ChainInfoConfig) -> Result<Self, Self::Error> {
        let host_port = from.host_port()?;
        Ok(Self {
            secure: host_port.0,
            host: host_port.1,
            port: host_port.2,
            signer: from.signer,
            signer_password: from.signer_password,
        })
    }
}
