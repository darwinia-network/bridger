use std::convert::TryInto;

use secp256k1::SecretKey;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::types::{Address, BlockId, BlockNumber, H256, U256};
use web3::Web3;

use support_ethereum::block::EthereumHeader;

use crate::errors::EthereumComponentError;
use crate::ethereum::types::GasPrice;
use crate::ethereum::EthereumConfig;

/// Ethereum client
pub struct EthereumClient {
    config: EthereumConfig,
    web3: Web3<Http>,
}

impl EthereumClient {
    /// Create an ethereum client
    pub fn new(config: EthereumConfig, web3: Web3<Http>) -> Self {
        Self { config, web3 }
    }
}

impl EthereumClient {
    fn build_address(str: &str) -> color_eyre::Result<Address> {
        let address = array_bytes::hex2bytes(&str[2..]).map_err(|_| {
            EthereumComponentError::WrongKey("str[2..]".to_string(), str.to_string())
        })?;
        let mut address_buffer = [0u8; 20];
        address_buffer.copy_from_slice(&address);
        Ok(Address::from(address_buffer))
    }

    async fn fast_gas_price() -> color_eyre::Result<U256> {
        let gasnow_url = "https://gasnow.sparkpool.com/api/v3/gas/price?utm_source=DarwiniaBridger";
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()?;
        let gas_price: GasPrice = client.get(gasnow_url).send().await?.json().await?;
        Ok(gas_price.data.fast.into())
    }
}

impl EthereumClient {
    /// Get web3 reference
    pub fn web3(&self) -> &Web3<Http> {
        &self.web3
    }

    /// Get ethereum header by block number
    pub async fn get_header_by_number(&self, block: u64) -> color_eyre::Result<EthereumHeader> {
        let eth_block = BlockId::Number(BlockNumber::Number(block.into()));
        match self.web3.eth().block(eth_block).await? {
            Some(block) => Ok(block.try_into()?),
            None => Err(EthereumComponentError::BlockNotFound(block).into()),
        }
    }

    /// submit_authorities_set
    pub async fn submit_authorities_set(
        &self,
        message: Vec<u8>,
        signatures: Vec<[u8; 65]>,
    ) -> color_eyre::Result<H256> {
        let relay_contract_address = Self::build_address(&self.config.subscribe_relay_address)?;
        let beneficiary = self
            .config
            .relayer_beneficiary_darwinia_account
            .clone()
            .ok_or_else(|| EthereumComponentError::NoBeneficiaryAccount)?;
        let secret_key = match &self.config.relayer_private_key {
            Some(seed) => {
                let private_key = array_bytes::hex2bytes(&seed[2..]).map_err(|_| {
                    EthereumComponentError::WrongKey("seed[2..]".to_string(), seed.clone())
                })?;
                Some(SecretKey::from_slice(&private_key)?)
            }
            None => None,
        }
        .ok_or_else(|| EthereumComponentError::NoPrivateKey)?;

        let key_ref = SecretKeyRef::new(&secret_key);
        let contract = Contract::from_json(
            self.web3.eth(),
            relay_contract_address,
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/abi/relay.json")),
        )?;

        // signatures
        let signature_list = signatures
            .iter()
            .map(|item| item.to_vec())
            .collect::<Vec<_>>();

        // beneficiary account id
        let beneficiary = array_bytes::hex2bytes(&beneficiary[2..]).map_err(|_| {
            EthereumComponentError::WrongKey(
                "beneficiary[2..]".to_string(),
                beneficiary.to_string(),
            )
        })?;
        let mut beneficiary_buffer = [0u8; 32];
        beneficiary_buffer.copy_from_slice(&beneficiary);

        // debug
        tracing::debug!(target: "component-ethereum", "message: {}", array_bytes::bytes2hex("0x", message.clone()));
        for (i, signature) in signature_list.clone().iter().enumerate() {
            tracing::debug!(
                target: "component-ethereum",
                "signature {}: {}",
                i + 1,
                array_bytes::bytes2hex("0x", signature)
            );
        }
        tracing::debug!(
            target: "component-ethereum",
            "beneficiary: {}",
            array_bytes::bytes2hex("0x", beneficiary_buffer)
        );

        // gas price
        // TODO: do not need to get gas_price if ropsten
        let gas_price = Self::fast_gas_price().await.ok();

        let input = (message, signature_list, beneficiary_buffer);
        let txhash = contract
            .signed_call(
                "updateRelayer",
                input,
                Options::with(|options| {
                    options.gas = Some(150_000.into());
                    options.gas_price = gas_price;
                }),
                key_ref,
            )
            .await?;
        Ok(txhash)
    }
}
