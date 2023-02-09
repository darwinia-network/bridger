use std::fmt::{Debug, Formatter};

use subxt::{
    sp_core::{sr25519::Pair, Pair as PairTrait},
    PairSigner,
};

use crate::config::PangoroSubxtConfig;
use crate::error::{ClientError, ClientResult};
use crate::types::NodeRuntimeSignedExtra;

/// AccountId
pub type AccountId = <PangoroSubxtConfig as subxt::Config>::AccountId;
/// Signer
pub type Signer = PairSigner<PangoroSubxtConfig, NodeRuntimeSignedExtra, Pair>;

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
            Pair::from_string(&seed, None).map_err(|e| ClientError::Seed(format!("{e:?}")))?; // if not a valid seed
        let signer = PairSigner::new(pair);
        let public = signer.signer().public().0;
        let account_id = AccountId::from(public);

        // real account, convert to account id
        let real =
            real.map(|real| AccountId::from(array_bytes::hex_n_into_unchecked::<String, AccountId, 20>(real)));

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
