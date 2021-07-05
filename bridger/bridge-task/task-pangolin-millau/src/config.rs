use serde::{Deserialize, Serialize};

use bridge_standard::bridge::config::{BridgeConfig, Config};
use external_s2s::traits::CliChain;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinMillauConfig {
    pangolin: ChainInfoConfig,
    millau: ChainInfoConfig,
}

impl PangolinMillauConfig {
    pub fn store<S: AsRef<str>>(&self, sand_name: S) -> anyhow::Result<()> {
        self.pangolin.check()?;
        self.millau.check()?;
        let name = sand_name.as_ref();
        Config::store_with_namespace(name, self.pangolin.clone(), "pangolin")?;
        Config::store_with_namespace(name, self.millau.clone(), "millau")?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChainInfoConfig {
    pub endpoint: String,
    pub signer: Option<String>,
    pub secure: bool,
    pub signer_password: Option<String>,
}

impl BridgeConfig for ChainInfoConfig {
    fn marker() -> &'static str {
        "s2s-chain-info"
    }
}

impl ChainInfoConfig {
    pub fn check(&self) -> anyhow::Result<()> {
        self.host_port()?;
        Ok(())
    }

    fn host_port(&self) -> anyhow::Result<(String, u16)> {
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
        Ok((host.to_string(), port.parse::<u16>()?))
    }

    pub fn host(&self) -> anyhow::Result<String> {
        Ok(self.host_port()?.0)
    }

    pub fn port(&self) -> anyhow::Result<u16> {
        Ok(self.host_port()?.1)
    }

    /// Convert connection params into Substrate client.
    pub async fn to_substrate_relay_chain<C: CliChain>(
        &self,
    ) -> anyhow::Result<relay_substrate_client::Client<C>> {
        Ok(
            relay_substrate_client::Client::new(relay_substrate_client::ConnectionParams {
                host: self.host()?,
                port: self.port()?,
                secure: self.secure,
            })
            .await,
        )
    }

    /// Parse signing params into chain-specific KeyPair.
    pub fn to_keypair<C: CliChain>(&self) -> anyhow::Result<C::KeyPair> {
        use sp_core::crypto::Pair;

        let signer = match self.signer.clone() {
            Some(v) => v,
            None => anyhow::bail!(
                "The chain [{}:{}] not set signer",
                self.host()?,
                self.port()?
            ),
        };
        C::KeyPair::from_string(&signer, self.signer_password.as_deref())
            .map_err(|e| anyhow::format_err!("{:?}", e))
    }
}
