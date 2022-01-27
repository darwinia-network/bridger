use serde::{Deserialize, Serialize};

use component_ethereum::types::web3::{Block, H256};

use crate::types::MMRRootJson;

/// Ethereum EthereumRelayHeaderParcel
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HeaderParcel {
    pub mmr_root: MMRRootJson,
    pub header: Block<H256>,
}
