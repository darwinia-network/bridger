use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, U256},
    Web3,
};

use crate::error::BridgeContractResult;

pub struct LaneMessageCommitter {
    pub contract: Contract<Http>,
}

impl LaneMessageCommitter {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/LaneMessageCommitter.json"),
        )?;

        Ok(Self { contract })
    }

    pub async fn bridged_chain_position(&self) -> BridgeContractResult<U256> {
        Ok(self
            .contract
            .query("bridgedChainPosition", (), None, Options::default(), None)
            .await?)
    }
}

pub mod types {}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use std::str::FromStr;

    #[tokio::test]
    async fn test_bridged_chain_position() {
        let transport = Http::new("https://pangoro-rpc.darwinia.network").unwrap();
        let client = web3::Web3::new(transport);
        let c = LaneMessageCommitter::new(
            &client,
            Address::from_str("0x8c6eb3e19dA6b3808592E3D54d31999adD444605").unwrap(),
        )
        .unwrap();
        let result = c.bridged_chain_position().await.unwrap();
        dbg!(result);
    }
}
