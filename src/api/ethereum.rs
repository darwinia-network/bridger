//! Ethereum API
use crate::{error::Result, Config};

use web3::contract::{Contract, Options};
use web3::types::{Address, H160};
use web3::Web3;
use web3::transports::Http;
use web3::signing::SecretKeyRef;
use secp256k1::SecretKey;
use crate::api::darwinia::AccountId;
use primitives::runtime::EcdsaSignature;
use crypto::sha3::Sha3;
use crypto::digest::Digest;

/// Ethereum
pub struct Ethereum {
    web3: Web3<Http>,
    relay_contract_address: Address,
    secret_key: SecretKey,
    benefit: Option<String>,
}

impl Ethereum {
    /// new
    pub fn new(web3: Web3<Http>, config: &Config) -> Result<Self> {
        let relay_contract_address = Ethereum::build_address(&config.darwinia_to_ethereum.relay_contract_address)?;
        let private_key = hex::decode(&config.darwinia_to_ethereum.seed[2..])?;
        let secret_key = SecretKey::from_slice(&private_key)?;
        Ok(Ethereum {
            web3,
            relay_contract_address,
            secret_key,
            benefit: config.darwinia_to_ethereum.benefit.clone(),
        })
    }

    /// submit_authorities_set
    pub async fn submit_authorities_set(&self, _term: u32, message: Vec<u8>, signatures: Vec<(AccountId, EcdsaSignature)>) -> Result<()> {
        if let Some(benefit) = &self.benefit {
            let key_ref = SecretKeyRef::new(&self.secret_key);

            let contract = Contract::from_json(
                self.web3.eth(),
                self.relay_contract_address,
                include_bytes!("Relay.json"),
            )?;

            // hash
            let mut hasher = Sha3::sha3_256();
            hasher.input(&message);
            let hash: &mut [u8] = &mut [];
            hasher.result(hash);

            // signatures
            let signature_list = signatures
                .iter()
                .map(|item| item.1.0.to_vec())
                .collect::<Vec<_>>();

            // benefit account id
            let benefit = hex::decode(&benefit[2..])?;

            contract.signed_call_with_confirmations(
                "updateRelayer",
                (hash.to_vec(), message, signature_list, benefit),
                Options::default(),
                12,
                key_ref
            ).await?;
        }

        Ok(())
    }

    fn build_address(str: &str) -> Result<H160> {
        let address = hex::decode(&str[2..])?;
        let mut address_buffer = [0u8; 20];
        address_buffer.copy_from_slice(&address);
        Ok(Address::from(address_buffer))
    }
}
