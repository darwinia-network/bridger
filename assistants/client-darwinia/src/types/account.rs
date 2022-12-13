use sp_core::ecdsa::Pair;
use subxt::tx::PairSigner;

use crate::config::DarwiniaSubxtConfig;

pub use self::darwinia::*;

/// AccountId
pub type AccountId = <DarwiniaSubxtConfig as subxt::Config>::AccountId;
/// Signer
pub type Signer = PairSigner<DarwiniaSubxtConfig, Pair>;

mod darwinia {
    use std::fmt::{Debug, Formatter};

    use sp_core::{ecdsa::Pair, Pair as TraitPair};
    use subxt::tx::PairSigner;

    use crate::error::{ClientError, ClientResult};

    use super::AccountId;
    use super::Signer;

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
            let pair = Pair::from_string(&seed, None)
                .map_err(|e| ClientError::Seed(format!("{:?}", e)))?; // if not a valid seed
            let signer = PairSigner::new(pair);
            let account_id = AccountId::from(array_bytes::hex2array_unchecked(seed.as_ref()));

            // real account, convert to account id
            let real =
                real.map(|real| AccountId::from(array_bytes::hex2array_unchecked(real.as_ref())));
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
}
