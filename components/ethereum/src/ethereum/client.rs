use support_ethereum::block::EthereumHeader;
use web3::transports::Http;
use web3::types::{BlockId, BlockNumber};
use web3::Web3;

use crate::config::EthereumConfig;
use crate::error::ComponentEthereumError;

pub struct EthereumClient {
    config: EthereumConfig,
    web3: Web3<Http>,
}

impl EthereumClient {
    pub fn new(config: EthereumConfig, web3: Web3<Http>) -> Self {
        Self { config, web3 }
    }
}

impl EthereumClient {
    pub async fn get_header_by_number(&self, block: u64) -> anyhow::Result<EthereumHeader> {
        let eth_block = BlockId::Number(BlockNumber::Number(block.into()));
        let block = self.web3.eth().block(eth_block).await?.ok_or_else(|| {
            ComponentEthereumError::FailToGetEthereumHeader(
                format!("The block number not found"),
                block,
            )
            .into()
        })?;
        Ok(block.into())
    }
}
