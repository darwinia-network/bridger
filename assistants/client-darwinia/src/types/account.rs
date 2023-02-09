use subxt::tx::PairSigner;

use ecdsa_pair::crypto::ethereum::Pair;

use crate::config::DarwiniaSubxtConfig;

pub use self::darwinia::*;

/// AccountId
pub type AccountId = <DarwiniaSubxtConfig as subxt::Config>::AccountId;
/// Signer
pub type Signer = PairSigner<DarwiniaSubxtConfig, Pair>;

mod darwinia {
    use std::fmt::{Debug, Formatter};

    use sp_core::Pair as TraitPair;
    use subxt::tx::PairSigner;

    use crate::error::{ClientError, ClientResult};

    use super::AccountId;
    use super::Pair;
    use super::Signer;

    /// Account
    #[derive(Clone)]
    pub struct DarwiniaAccount {
        /// signer of the account
        signer: Signer,
        /// proxy real
        real: Option<Signer>,
    }

    impl Debug for DarwiniaAccount {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("account: {},", self.signer.account_id()))?;
            f.write_str(" signer: <..>,")?;
            f.write_str(&format!(
                " real: {:?}",
                self.real.clone().map(|v| *v.account_id())
            ))?;
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

            let mut real_signer = None;
            if let Some(real_seed) = real {
                let pair = Pair::from_string(&real_seed, None)
                    .map_err(|e| ClientError::Seed(format!("{e:?}")))?;
                real_signer = Some(PairSigner::new(pair))
            };
            Ok(Self {
                signer,
                real: real_signer,
            })
        }
    }

    impl DarwiniaAccount {
        /// get account id
        pub fn account_id(&self) -> &AccountId {
            self.signer.account_id()
        }

        /// get signer
        pub fn signer(&self) -> &Signer {
            &self.signer
        }

        /// get raw real account
        pub fn real_account(&self) -> &AccountId {
            if let Some(real_signer) = &self.real {
                real_signer.account_id()
            } else {
                self.signer.account_id()
            }
        }
    }
}
