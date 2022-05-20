use serde::{Deserialize, Serialize};

use component_ethereum::types::web3::{Block, H256};

/// Ethereum EthereumRelayHeaderParcel
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HeaderParcel {
    pub mmr_root: [u8; 32],
    pub header: Block<H256>,
}
