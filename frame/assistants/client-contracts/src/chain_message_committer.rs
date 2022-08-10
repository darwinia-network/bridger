use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, H256},
    Web3,
};

use crate::error::BridgeContractResult;

pub struct ChainMessageCommitter {
    pub contract: Contract<Http>,
}

impl ChainMessageCommitter {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/ChainMessageCommitter.json"),
        )?;

        Ok(Self { contract })
    }

    pub async fn commitment(&self) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .query("commitment", (), None, Options::default(), None)
            .await?)
    }
}

mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_query_commitment() {
        let transport = Http::new("https://pangoro-rpc.darwinia.network").unwrap();
        let client = web3::Web3::new(transport);
        let c = ChainMessageCommitter::new(
            &client,
            Address::from_str("0x492b0E386ddC970395B3A506E2E56DfFaf49947D").unwrap(),
        )
        .unwrap();
        let result = c.commitment().await.unwrap();
        dbg!(result);
    }
}
