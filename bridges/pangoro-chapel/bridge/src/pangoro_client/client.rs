use std::str::FromStr;

use crate::pangoro_client::types::{BSCHeader, Checkpoint, TBSCHeader};
use secp256k1::SecretKey;
use support_common::error::BridgerError;
use web3::{
    contract::{tokens::Tokenize, Contract, Options},
    ethabi::Token,
    transports::Http,
    types::{Address, H256, U256},
    Web3,
};

pub struct PangoroClient {
    pub client: Web3<Http>,
    pub bsc_light_client: Contract<Http>,
    pub private_key: Option<SecretKey>,
}

impl PangoroClient {
    pub fn new(
        endpoint: &str,
        bsc_address: &str,
        private_key: Option<&str>,
    ) -> color_eyre::Result<Self> {
        let transport = Http::new(endpoint)?;
        let client = Web3::new(transport);
        let bsc_light_client = Contract::from_json(
            client.eth(),
            Address::from_str(bsc_address)?,
            include_bytes!("BSCLightClient.json"),
        )?;
        let private_key = private_key
            .map(SecretKey::from_str)
            .transpose()?;
        Ok(Self {
            client,
            bsc_light_client,
            private_key,
        })
    }

    pub async fn get_finalized_checkpoint(&self) -> color_eyre::Result<Checkpoint> {
        let query =
            self.bsc_light_client
                .query("finalized_checkpoint", (), None, Options::default(), None);
        let checkpoint: Checkpoint = query.await?;
        Ok(checkpoint)
    }

    pub async fn get_authority_set_length(&self) -> color_eyre::Result<U256> {
        let query = self.bsc_light_client.query(
            "length_of_finalized_authorities",
            (),
            None,
            Options::default(),
            None,
        );
        let length: U256 = query.await?;
        Ok(length)
    }

    pub async fn import_finalized_epoch_header(
        &self,
        headers: Vec<BSCHeader>,
    ) -> color_eyre::Result<H256> {
        let parameter = headers
            .into_iter()
            .map(|x| Token::Tuple(TBSCHeader::from(x).into_tokens()))
            .collect::<Vec<Token>>();

        let tx = self
            .bsc_light_client
            .signed_call(
                "import_finalized_epoch_header",
                (parameter,),
                Options {
                    gas: Some(U256::from(10000000)),
                    gas_price: Some(U256::from(1300000000)),
                    ..Default::default()
                },
                &self
                    .private_key
                    .ok_or_else(|| BridgerError::Custom("Private key is not valid".into()))?,
            )
            .await?;
        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> PangoroClient {
        PangoroClient::new(
            "https://pangoro-rpc.darwinia.network",
            "0x6ac5ae3fa61b2cbea625dd24f57bdc3d952333c9",
            None,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_query_finalized_header() {
        let client = test_client();
        let checkpoint = client.get_finalized_checkpoint().await.unwrap();
        println!("Finalized checkpoint: {:?}", checkpoint);
    }
}
