use secp256k1::SecretKey;
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::Web3;

use crate::account::DarwiniaAccount;
use crate::types::EcdsaSignature;
use crate::{error::Error, error::Result};

#[derive(Clone)]
pub struct EthereumAccount {
    /// ethereum url
    pub ethereum_url: String,
    /// authority signer raw ethereum seed
    pub ethereum_seed: Option<String>,
}

/// Account
#[derive(Clone)]
pub struct Account(pub DarwiniaAccount, pub EthereumAccount);

impl Account {
    /// Create a new Account
    pub fn new(
        darwinia_account: DarwiniaAccount,
        ethereum_seed: Option<String>,
        ethereum_url: String,
    ) -> Account {
        Account(
            darwinia_account,
            EthereumAccount {
                ethereum_url,
                ethereum_seed,
            },
        )
    }

    /// sign
    pub fn ecdsa_sign(&self, message: &[u8]) -> Result<EcdsaSignature> {
        let web3 = Web3::new(Http::new(&self.1.ethereum_url)?);
        if let Some(ethereum_seed) = &self.1.ethereum_seed {
            let private_key = array_bytes::hex2bytes(&ethereum_seed[2..])
                .map_err(|_| Error::Hex2Bytes("ethereum_seed[2..]".into()))?;
            let secret_key = SecretKey::from_slice(&private_key)?;
            let signature = web3
                .accounts()
                .sign(message, SecretKeyRef::new(&secret_key))
                .signature;
            let mut buffer = [0u8; 65];
            buffer.copy_from_slice(signature.0.as_slice());
            Ok(EcdsaSignature(buffer))
        } else {
            Err(Error::NoAuthoritySignerSeed)
        }
    }

    /// ethereum seed
    pub fn has_ethereum_seed(&self) -> bool {
        self.1.ethereum_seed.is_some()
    }
}

#[test]
fn test_ecdsa() {
    let hash =
        array_bytes::hex2bytes("71e2f60faf6c7264cca14fb1a01260a787b4d18039cd8cd680aaff1e118c711d")
            .unwrap();
    let hash = hash.as_slice();
    let web3 = Web3::new(
        Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap(),
    );
    let private_key =
        array_bytes::hex2bytes("8bd012fd2433d4fea852f437d6bb22d1e57dee7657cc1e703460ddeaae1a67ca")
            .unwrap();
    let secret_key = SecretKey::from_slice(&private_key).unwrap();
    let signature = web3
        .accounts()
        .sign(hash, SecretKeyRef::new(&secret_key))
        .signature;
    let mut buffer = [0u8; 65];
    buffer.copy_from_slice(signature.0.as_slice());
    println!("{:x?}", buffer);
}
