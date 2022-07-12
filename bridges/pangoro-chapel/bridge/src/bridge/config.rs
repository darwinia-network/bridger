use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChapelWeb3Config {
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangoroConfig {
    pub endpoint: String,
    pub bsc_address: String,
    pub private_key: String,
}

/// Bridge template config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangoroChapelConfig {
    /// Pangoro subxt config
    pub pangoro: PangoroConfig,

    /// Chapel web3 config
    pub chapel: ChapelWeb3Config,
}
