use secp256k1::SecretKey;
use web3::signing::{Key, SecretKeyRef};
use web3::types::Address;

use crate::error::RelayResult;

/// fast ethereum account, provide a text seed
#[derive(Clone)]
pub struct FastEthereumAccount {
    seed: String,
}

impl FastEthereumAccount {
    pub fn new(seed: impl AsRef<str>) -> Self {
        let seed = seed.as_ref();
        let mut seed = seed.to_string();
        if seed.contains("0x") {
            seed = seed.replace("0x", "");
        }
        Self { seed }
    }
}

impl FastEthereumAccount {
    /// get secret key
    pub fn secret_key(&self) -> RelayResult<SecretKey> {
        let private_key = array_bytes::hex2bytes(&self.seed)?;
        let secret_key = SecretKey::from_slice(&private_key)?;
        Ok(secret_key)
    }

    /// ethereum address
    pub fn address(&self) -> RelayResult<Address> {
        let secret_key = self.secret_key()?;
        let secret_key_ref = SecretKeyRef::new(&secret_key);
        let address = secret_key_ref.address();
        Ok(address)
    }
}
