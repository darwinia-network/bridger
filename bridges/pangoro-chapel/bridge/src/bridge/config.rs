use serde::{Deserialize, Serialize};

use client_pangoro::config::ClientConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChapelWeb3Config {
    pub endpoint: String,
}

/// Bridge template config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangoroChapelConfig {
    /// Pangoro subxt config
    pub pangoro: ClientConfig,

    /// Chapel web3 config
    pub chapel: ChapelWeb3Config,
}
