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
    pub contract: Contract<Http>,
}

impl PosaLightClient {
    pub fn new(client: &Web3<Http>, address: Address) -> BridgeContractResult<Self> {
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
        private_key: &SecretKey,
    ) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .signed_call(
                "add_relayer",
                (relayer, threshold, signatures),
                Options::default(),
                private_key,
            )
            .await?)
    }

    pub async fn remove_relayer(
        &self,
        prev_relayer: Address,
        relayer: Address,
        threshold: U256,
        signatures: Vec<Bytes>,
        private_key: &SecretKey,
    ) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .signed_call(
                "remove_relayer",
                (prev_relayer, relayer, threshold, signatures),
                Options::default(),
                private_key,
            )
            .await?)
    }

    pub async fn swap_relayer(
        &self,
        prev_relayer: Address,
        old_relayer: Address,
        new_relayer: Address,
        signatures: Vec<Bytes>,
        private_key: &SecretKey,
    ) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .signed_call(
                "swap_relayer",
                (prev_relayer, old_relayer, new_relayer, signatures),
                Options::default(),
                private_key,
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
                Options::default(),
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

    pub async fn get_relayers(&self) -> BridgeContractResult<Vec<Address>> {
        Ok(self
            .contract
            .query("get_relayers", (), None, Options::default(), None)
            .await?)
    }

    pub async fn nonce(&self) -> BridgeContractResult<U256> {
        Ok(self
            .contract
            .query("nonce", (), None, Options::default(), None)
            .await?)
    }

    pub async fn merkle_root(&self) -> BridgeContractResult<H256> {
        Ok(self
            .contract
            .query("merkle_root", (), None, Options::default(), None)
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

    fn test_client() -> PosaLightClient {
        let transport =
            Http::new("https://eth-goerli.g.alchemy.com/v2/WerPq7On62-wy_ARssv291ZPg1TGR5vi")
                .unwrap();
        let client = web3::Web3::new(transport);
        PosaLightClient::new(
            &client,
            Address::from_str("0x82afDD48E3a06672c7C87A6742eC14d1088f6eF7").unwrap(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_query() {
        let lclient = test_client();
        let result = lclient.block_number().await.unwrap();
        dbg!(result);
    }

    #[tokio::test]
    async fn test_get_relayers() {
        let transport =
            Http::new("https://eth-goerli.g.alchemy.com/v2/WerPq7On62-wy_ARssv291ZPg1TGR5vi")
                .unwrap();
        let client = web3::Web3::new(transport);
        let lclient = PosaLightClient::new(
            &client,
            Address::from_str("0x6c74a72444048A8588dEBeb749Ee60DB842aD90f").unwrap(),
        )
        .unwrap();
        let result = lclient.get_relayers().await.unwrap();
        dbg!(result);
    }

    #[tokio::test]
    async fn test_nonce() {
        let client = test_client();
        let result = client.nonce().await.unwrap();
        dbg!(result);

        let result = client.merkle_root().await.unwrap();
        dbg!(result);

        let result = client.block_number().await.unwrap();
        dbg!(result);
    }
}
