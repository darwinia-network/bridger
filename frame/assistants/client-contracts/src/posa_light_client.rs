use secp256k1::SecretKey;
pub use types::*;
use web3::{
    contract::{Contract, Options},
    ethabi::Bytes,
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

use crate::error::BridgeContractResult;

#[derive(Debug, Clone)]
pub struct PosaLightClient {
    contract: Contract<Http>,
}

impl PosaLightClient {
    pub fn new(client: Web3<Http>, address: Address) -> BridgeContractResult<Self> {
        let contract = Contract::from_json(
            client.eth(),
            address,
            include_bytes!("abis/POSALightClient.json"),
        )?;

        Ok(Self { contract })
    }

    pub async fn add_relayer(
        &self,
        relayer: Address,
        threshold: U256,
        signatures: Vec<Bytes>,
        from: Address,
    ) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .call(
                "add_relayer",
                (relayer, threshold, signatures),
                from,
                Options::default(),
            )
            .await?)
    }

    pub async fn remove_relayer(
        &self,
        prev_relayer: Address,
        relayer: Address,
        threshold: U256,
        signatures: Vec<Bytes>,
        from: Address,
    ) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .call(
                "add_relayer",
                (prev_relayer, relayer, threshold, signatures),
                from,
                Options::default(),
            )
            .await?)
    }

    pub async fn swap_relayer(
        &self,
        prev_relayer: Address,
        old_relayer: Address,
        new_relayer: Address,
        signatures: Vec<Bytes>,
        from: Address,
    ) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .call(
                "add_relayer",
                (prev_relayer, old_relayer, new_relayer, signatures),
                from,
                Options::default(),
            )
            .await?)
    }

    pub async fn import_message_commitment(
        &self,
        commitment: Commitment,
        signature: Vec<Bytes>,
        private_key: &SecretKey,
    ) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .signed_call(
                "import_message_commitment",
                (commitment, signature),
                Options {
                    gas: Some(U256::from(10000000u64)),
                    gas_price: Some(U256::from(1300000000u64)),
                    // value: Some(fee),
                    ..Default::default()
                },
                private_key,
            )
            .await?)
    }

    pub async fn block_number(&self) -> BridgeContractResult<U256> {
        Ok(self
            .contract
            .query("block_number", (), None, Options::default(), None)
            .await?)
    }
}

pub mod types {
    use web3::{
        contract::tokens::{Detokenize, Tokenizable},
        ethabi::Token,
        types::{H256, U256},
    };

    #[derive(Debug, Clone)]
    pub struct Commitment {
        pub block_number: u32,
        pub message_root: H256,
        pub nonce: U256,
    }

    impl Tokenizable for Commitment {
        fn from_token(token: Token) -> Result<Self, web3::contract::Error>
        where
            Self: Sized,
        {
            if let Token::Tuple(tokens) = token {
                let (block_number, message_root, nonce) = Detokenize::from_tokens(tokens)?;
                Ok(Self {
                    block_number,
                    message_root,
                    nonce,
                })
            } else {
                Err(web3::contract::Error::InvalidOutputType(format!(
                    "Failed to decode from : {:?}",
                    token
                )))
            }
        }

        fn into_token(self) -> Token {
            Token::Tuple(vec![
                self.block_number.into_token(),
                self.message_root.into_token(),
                self.nonce.into_token(),
            ])
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use web3::transports::Http;

    #[tokio::test]
    async fn test_query() {
        let transport = Http::new("http://127.0.0.1:8545").unwrap();
        let client = web3::Web3::new(transport);
        let lclient = PosaLightClient::new(
            client,
            Address::from_str("0xd345Cc26e3685DA584AC9C38393F9f603B122EcF").unwrap(),
        )
        .unwrap();
        let result = lclient.block_number().await.unwrap();
        dbg!(result);
    }
}
