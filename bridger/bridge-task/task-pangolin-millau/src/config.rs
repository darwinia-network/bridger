use bridge_config::Config;
use bridge_standard::bridge::config::BridgeConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinMillauConfig {
    pangolin: ChainInfoConfig,
    millau: ChainInfoConfig,
}

impl PangolinMillauConfig {
    pub fn store<S: AsRef<str>>(&self, sand_name: S) -> anyhow::Result<()> {
        let _name = sand_name.as_ref();
        Config::store(sand_name, self.pangolin.clone())?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChainInfoConfig {
    // pub host: String,
    // pub port: u32,
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
