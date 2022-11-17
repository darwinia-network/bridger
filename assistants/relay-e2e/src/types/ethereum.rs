use secp256k1::SecretKey;
use web3::signing::{Key, SecretKeyRef};
use web3::types::Address;

use crate::error::{RelayError, RelayResult};

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

    /// sign message
    pub fn sign(&self, message: &[u8]) -> RelayResult<Vec<u8>> {
        let secret_key = self.secret_key()?;
        let secret_key_ref = SecretKeyRef::new(&secret_key);
        let signature = secret_key_ref
            .sign_message(message)
            .map_err(|e| RelayError::Custom(format!("Sign message error: {:?}", e)))?;
        let v = signature
            .v
            .try_into()
            .expect("signature recovery in electrum notation always fits in a u8");

        let mut bytes = Vec::with_capacity(65);
        bytes.extend_from_slice(signature.r.as_bytes());
        bytes.extend_from_slice(signature.s.as_bytes());
        bytes.push(v);
        Ok(bytes)
    }
}
