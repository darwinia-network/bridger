use crate::{
    DarwiniaAccount,
    Error,
    error::Result,
};

use primitives::{
	frame::{
		bridge::relay_authorities::{
            AuthoritiesToSignStoreExt,
			AuthoritiesStoreExt,
            MmrRootsToSignStoreExt,
        }
    },
	runtime::{EcdsaMessage, EcdsaSignature},
};

use secp256k1::SecretKey;
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::Web3;

#[derive(Clone)]
pub struct EthereumAccount {
	/// ethereum url
	pub ethereum_url: String,
	/// authority signer raw ethereum seed
	pub ethereum_seed: Option<String>,
}

/// Account
#[derive(Clone)]
pub struct Account {
    pub darwinia_account: DarwiniaAccount,
    pub ethereum_account: EthereumAccount,
}

impl Account {
	/// Create a new Account
	pub fn new(
        darwinia_account: DarwiniaAccount,
		ethereum_seed: Option<String>,
		ethereum_url: String,
	) -> Account {
		Account {
            darwinia_account,
            ethereum_account: EthereumAccount {
                ethereum_url,
                ethereum_seed,
            },
		}
	}

    /// Print Detail
    pub async fn detail(&self) -> Result<()> {
        let mut roles = self.darwinia_account.role().await?;
        if self.is_authority().await? {
            roles.push("Authority".to_string());
        }
        match &self.darwinia_account.real {
            None => {
                info!("🧔 Relayer({:?}): 0x{:?}", roles, self.darwinia_account.account_id);
            }
            Some(real_account_id) => {
                info!("🧔 Proxy Relayer: 0x{:?}", self.darwinia_account.account_id);
                info!("👴 Real Account({:?}): 0x{:?}", roles, real_account_id);
            }
        }
        Ok(())
    }

    /// is authority
    pub async fn is_authority(&self) -> Result<bool> {
        let authorities = self
            .darwinia_account
			.darwinia
            .subxt
			.authorities(None)
			.await?
			.iter()
			.map(|a| a.account_id.clone())
			.collect::<Vec<_>>();
		if let Some(real_account_id) = &self.darwinia_account.real {
            Ok(authorities.contains(real_account_id))
        } else {
            Ok(authorities.contains(&self.darwinia_account.account_id))
        }
    }

    /// sign
	pub fn ecdsa_sign(&self, message: &[u8]) -> Result<EcdsaSignature> {
		let web3 = Web3::new(Http::new(&self.ethereum_account.ethereum_url)?);
		if let Some(ethereum_seed) = &self.ethereum_account.ethereum_seed {
			let private_key = hex::decode(&ethereum_seed[2..])?;
			let secret_key = SecretKey::from_slice(&private_key)?;
			let signature = web3
				.accounts()
				.sign(message, SecretKeyRef::new(&secret_key))
				.signature;
			let mut buffer = [0u8; 65];
			buffer.copy_from_slice(signature.0.as_slice());
			Ok(EcdsaSignature(buffer))
		} else {
			Err(Error::NoAuthoritySignerSeed.into())
		}
	}

    /// need_to_sign_authorities
	pub async fn need_to_sign_authorities(&self, message: EcdsaMessage) -> Result<bool> {
		let ret = self
            .darwinia_account
            .darwinia
            .subxt
            .authorities_to_sign(None).await?;
		match ret {
			None => Ok(false),
			Some(r) => {
				if r.0 == message {
					let includes = r.1.iter().any(|a| a.0 == self.darwinia_account.account_id);
					Ok(!includes)
				} else {
					Ok(false)
				}
			}
		}
	}

    /// need_to_mmr_root_of
	pub async fn need_to_sign_mmr_root_of(&self, block_number: u32) -> bool {
		match self
            .darwinia_account
            .darwinia
            .subxt
            .mmr_roots_to_sign(block_number, None)
            .await {
			Ok(mmr_roots_to_sign) => match mmr_roots_to_sign {
				None => false,
				Some(items) => {
					let includes = items.iter().any(|a| a.0 == self.darwinia_account.account_id);
					!includes
				}
			},
			Err(err) => {
				error!(
					"An error was encountered when trying to get storage MMRRootsToSign: {:?}",
					err
				);
				false
			}
		}
	}
}

#[test]
fn test_ecdsa() {
	let hash =
		hex::decode("71e2f60faf6c7264cca14fb1a01260a787b4d18039cd8cd680aaff1e118c711d").unwrap();
	let hash = hash.as_slice();
	let web3 = Web3::new(
		Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap(),
	);
	let private_key =
		hex::decode("8bd012fd2433d4fea852f437d6bb22d1e57dee7657cc1e703460ddeaae1a67ca").unwrap();
	let secret_key = SecretKey::from_slice(&private_key).unwrap();
	let signature = web3
		.accounts()
		.sign(hash, SecretKeyRef::new(&secret_key))
		.signature;
	let mut buffer = [0u8; 65];
	buffer.copy_from_slice(signature.0.as_slice());
	println!("{:x?}", buffer);
}
