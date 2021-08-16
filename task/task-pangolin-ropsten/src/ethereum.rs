//! Ethereum API
use std::time::Duration;

use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::types::{Address, H160, H256, U256};
use web3::Web3;

use component_pangolin_subxt::types::EcdsaSignature;

use crate::{error::Error, error::Result};

#[derive(Debug, Serialize, Deserialize)]
struct GasPrice {
    code: i32,
    data: GasPriceData,
}

#[derive(Debug, Serialize, Deserialize)]
struct GasPriceData {
    rapid: u64,
    fast: u64,
    slow: u64,
    standard: u64,
    timestamp: u64,
}

/// Ethereum
pub struct Ethereum {
    web3: Web3<Http>,
    relay_contract_address: Address,
    /// secret_key to send ethereum tx
    pub secret_key: Option<SecretKey>,
    beneficiary: Option<String>,
}

impl Ethereum {
    /// new
    pub fn new(
        web3: Web3<Http>,
        relay_contract: String,
        relayer_private_key: Option<String>,
        beneficiary_darwinia_account: Option<String>,
    ) -> Result<Self> {
        let relay_contract_address = Ethereum::build_address(&relay_contract)?;
        let secret_key = if let Some(seed) = relayer_private_key {
            let private_key = array_bytes::hex2bytes(&seed[2..])
                .map_err(|_| Error::Hex2Bytes("seed[2..]".into()))?;
            Some(SecretKey::from_slice(&private_key)?)
        } else {
            None
        };

        Ok(Ethereum {
            web3,
            relay_contract_address,
            secret_key,
            beneficiary: beneficiary_darwinia_account,
        })
    }

    /// submit_authorities_set
    pub async fn submit_authorities_set(
        &self,
        message: Vec<u8>,
        signatures: Vec<EcdsaSignature>,
    ) -> Result<H256> {
        if let Some(beneficiary) = &self.beneficiary {
            if let Some(secret_key) = &self.secret_key {
                let key_ref = SecretKeyRef::new(secret_key);

                let contract = Contract::from_json(
                    self.web3.eth(),
                    self.relay_contract_address,
                    include_bytes!("Relay.json"),
                )?;

                // signatures
                let signature_list = signatures
                    .iter()
                    .map(|item| item.0.to_vec())
                    .collect::<Vec<_>>();

                // beneficiary account id
                let beneficiary = array_bytes::hex2bytes(&beneficiary[2..])
                    .map_err(|_| Error::Hex2Bytes("beneficiary[2..]".into()))?;
                let mut beneficiary_buffer = [0u8; 32];
                beneficiary_buffer.copy_from_slice(&beneficiary);

                // debug
                debug!("message: {}", array_bytes::bytes2hex("0x", message.clone()));
                for (i, signature) in signature_list.clone().iter().enumerate() {
                    debug!(
                        "signature {}: {}",
                        i + 1,
                        array_bytes::bytes2hex("0x", signature)
                    );
                }
                debug!(
                    "beneficiary: {}",
                    array_bytes::bytes2hex("0x", beneficiary_buffer)
                );

                // gas price
                // TODO: do not need to get gas_price if ropsten
                let gas_price = Ethereum::fast_gas_price().await.ok();

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
            } else {
                anyhow::bail!("You have no ethereum private key configured.")
            }
        } else {
            anyhow::bail!("You have no beneficiary configured.")
        }
    }

    fn build_address(str: &str) -> Result<H160> {
        let address =
            array_bytes::hex2bytes(&str[2..]).map_err(|_| Error::Hex2Bytes("str[2..]".into()))?;
        let mut address_buffer = [0u8; 20];
        address_buffer.copy_from_slice(&address);
        Ok(Address::from(address_buffer))
    }

    async fn fast_gas_price() -> Result<U256> {
        let gasnow_url = "https://gasnow.sparkpool.com/api/v3/gas/price?utm_source=DarwiniaBridger";
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()?;
        let gas_price: GasPrice = client.get(gasnow_url).send().await?.json().await?;
        Ok(gas_price.data.fast.into())
    }
}
