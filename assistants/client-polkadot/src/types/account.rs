use std::fmt::{Debug, Formatter};

use sp_core::{sr25519::Pair, Pair as PairTrait};
use subxt::tx::PairSigner;

use crate::config::PolkadotSubxtConfig;
use crate::error::{ClientError, ClientResult};

/// AccountId
pub type AccountId = <PolkadotSubxtConfig as subxt::Config>::AccountId;
/// Signer
pub type Signer = PairSigner<PolkadotSubxtConfig, Pair>;

/// Account
#[derive(Clone)]
pub struct PolkadotAccount {
    /// Account Id
    account_id: AccountId,
    /// signer of the account
    signer: Signer,
}

impl Debug for PolkadotAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("account: {},", self.account_id))?;
        f.write_str(" signer: <..>,")?;
        Ok(())
    }
}

impl PolkadotAccount {
    /// Create a new Account
    pub fn new(seed: String) -> ClientResult<Self> {
        // signer to sign darwinia extrinsic
        let pair =
            Pair::from_string(&seed, None).map_err(|e| ClientError::Seed(format!("{e:?}")))?; // if not a valid seed
        let signer = PairSigner::new(pair);
        let public = signer.signer().public().0;
        let account_id = AccountId::from(public);

        Ok(Self { account_id, signer })
    }
}

impl PolkadotAccount {
    /// get account id
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// get signer
    pub fn signer(&self) -> &Signer {
        &self.signer
    }
}
