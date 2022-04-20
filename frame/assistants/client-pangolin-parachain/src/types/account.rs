use pangolin_parachain_subxt::api::runtime_types::darwinia_claims::EcdsaSignature;
use secp256k1::SecretKey;
use std::fmt::{Debug, Formatter};

use subxt::{
    sp_core::{sr25519::Pair, Pair as PairTrait},
    PairSigner,
};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::Web3;

use crate::config::PangolinParachainSubxtConfig;
use crate::error::{ClientError, ClientResult};

/// AccountId
pub type AccountId = <PangolinParachainSubxtConfig as subxt::Config>::AccountId;
/// Signer
pub type Signer = PairSigner<PangolinParachainSubxtConfig, Pair>;

/// Account
#[derive(Clone)]
pub struct DarwiniaAccount {
    /// Account Id
    account_id: AccountId,
    /// signer of the account
    signer: Signer,
    /// proxy real
    real: Option<AccountId>,
}

impl Debug for DarwiniaAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("account: {},", self.account_id))?;
        f.write_str(" signer: <..>,")?;
        f.write_str(&format!(" real: {:?}", self.real))?;
        Ok(())
    }
}

impl DarwiniaAccount {
    /// Create a new Account
    pub fn new(seed: String, real: Option<String>) -> ClientResult<Self> {
        // signer to sign darwinia extrinsic
        let pair =
            Pair::from_string(&seed, None).map_err(|e| ClientError::Seed(format!("{:?}", e)))?; // if not a valid seed
        let signer = PairSigner::new(pair);
        let public = signer.signer().public().0;
        let account_id = AccountId::from(public);

        // real account, convert to account id
        let real = real.map(|real| AccountId::from(array_bytes::hex2array_unchecked(real)));

        Ok(Self {
            account_id,
            signer,
            real,
        })
    }
}

impl DarwiniaAccount {
    /// get account id
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// get signer
    pub fn signer(&self) -> &Signer {
        &self.signer
    }

    /// get real account
    pub fn real(&self) -> &Option<AccountId> {
        &self.real
    }

    /// get raw real account
    pub fn real_account(&self) -> &AccountId {
        if let Some(real_account_id) = &self.real {
            real_account_id
        } else {
            &self.account_id
        }
    }
}

/// ethereum account
#[derive(Clone)]
pub struct EthereumAccount {
    url: String,
    seed: Option<String>,
}

impl Debug for EthereumAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("url: {},", self.url))?;
        f.write_str(" seed: <..>")?;
        Ok(())
    }
}

impl EthereumAccount {
    /// Create new ethereum account
    pub fn new(url: String, seed: Option<String>) -> Self {
        Self { url, seed }
    }
}

impl EthereumAccount {
    /// sign
    pub fn ecdsa_sign(&self, message: &[u8]) -> ClientResult<EcdsaSignature> {
        let web3 = Web3::new(Http::new(&self.url)?);
        if let Some(ethereum_seed) = &self.seed {
            let private_key = array_bytes::hex2bytes(&ethereum_seed[2..])
                .map_err(|_| ClientError::Hex2Bytes("ethereum_seed[2..]".into()))?;
            let secret_key = SecretKey::from_slice(&private_key)?;
            let signature = web3
                .accounts()
                .sign(message, SecretKeyRef::new(&secret_key))
                .signature;
            let mut buffer = [0u8; 65];
            buffer.copy_from_slice(signature.0.as_slice());
            Ok(EcdsaSignature(buffer))
        } else {
            Err(ClientError::NoAuthoritySignerSeed)
        }
    }
}
