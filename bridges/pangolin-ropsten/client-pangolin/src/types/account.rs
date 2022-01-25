use crate::config::PangolinSubxtConfig;
use std::fmt::{Debug, Formatter};
use subxt::{
    sp_core::{sr25519::Pair, Pair as PairTrait},
    system::System,
    DefaultExtra, PairSigner,
};

use crate::darwinia::runtime::DarwiniaRuntime;
use crate::types::NodeRuntimeSignedExtra;

/// AccountId
pub type AccountId = <PangolinSubxtConfig as subxt::Config>::AccountId;

/// Account
pub struct DarwiniaAccount {
    /// Account Id
    pub account_id: AccountId,
    /// signer of the account
    pub signer: PairSigner<PangolinSubxtConfig, NodeRuntimeSignedExtra, Pair>,
    /// proxy real
    pub real: Option<AccountId>,
}

impl Debug for DarwiniaAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("account: {},", self.account_id))?;
        f.write_str(" signer: <..>,")?;
        f.write_str(&format!(" real: {:?}", self.real))?;
        Ok(())
    }
}

impl Clone for DarwiniaAccount {
    fn clone(&self) -> Self {
        Self {
            account_id: self.account_id.clone(),
            signer: self.signer.clone(),
            real: self.real.clone(),
        }
    }
}

impl DarwiniaAccount {
    /// Create a new Account
    pub fn new(seed: String, real: Option<String>) -> DarwiniaAccount {
        // signer to sign darwinia extrinsic
        let pair = Pair::from_string(&seed, None).unwrap(); // if not a valid seed
        let signer = PairSigner::new(pair);
        let public = signer.signer().public().0;
        let account_id = AccountId::from(public);

        // real account, convert to account id
        let real = real.map(|real| AccountId::from(array_bytes::hex2array_unchecked(real)));

        DarwiniaAccount {
            account_id,
            signer,
            real,
        }
    }

    /// get the real account
    pub fn real(&self) -> &AccountId {
        if let Some(real_account_id) = &self.real {
            real_account_id
        } else {
            &self.account_id
        }
    }
}
