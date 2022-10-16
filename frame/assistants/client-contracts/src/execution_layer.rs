use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, BlockId, H256},
    Web3,
};

use crate::error::BridgeContractResult;

pub struct ExecutionLayer {
    pub contract: Contract<Http>,
}

impl ExecutionLayer {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/ExecutionLayer.json"),
        )?;

        Ok(Self { contract })
    }

    pub async fn execution_layer_state_root(
        &self,
        at_block: Option<BlockId>,
    ) -> BridgeContractResult<H256> {
        let query = self
            .contract
            .query("merkle_root", (), None, Options::default(), at_block);
        Ok(query.await?)
    }
}
