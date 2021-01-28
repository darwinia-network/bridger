use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::{
    sp_core::Pair as PairTrait, 
    PairSigner,
    system::System,
};

use primitives::{
	runtime::{DarwiniaRuntime},
	frame::{
		sudo::KeyStoreExt,
    },
};

use crate::{
    error::Result,
    darwinia::Darwinia,
};

/// AccountId
pub type AccountId = <DarwiniaRuntime as System>::AccountId;

/// Account
pub struct DarwiniaAccount {
	/// darwinia client
	pub darwinia: Darwinia,
	/// Account Id
	pub account_id: AccountId,
	/// signer of the account
	pub signer: PairSigner<DarwiniaRuntime, Pair>,
	/// proxy real
	pub real: Option<AccountId>,
}

impl Clone for DarwiniaAccount {
    fn clone(&self) -> Self {
        Self {
            darwinia: self.darwinia.clone(),
            account_id: self.account_id.clone(),
            signer: self.signer.clone(),
            real: self.real.clone(),
        }
    }
}

impl DarwiniaAccount {
	/// Create a new Account
	pub fn new(
		seed: String,
		real: Option<String>,
		darwinia: Darwinia,
	) -> DarwiniaAccount {
		// signer to sign darwinia extrinsic
		let pair = Pair::from_string(&seed, None).unwrap(); // if not a valid seed
		let signer = PairSigner::<DarwiniaRuntime, Pair>::new(pair);
		let public = signer.signer().public().0;
		let account_id = AccountId::from(public);

		// real account, convert to account id
		let real = real.map(|real| {
			let real = hex::decode(real).unwrap(); // if decode fail
			let mut data: [u8; 32] = [0u8; 32];
			data.copy_from_slice(&real[..]);
			AccountId::from(data)
		});

		DarwiniaAccount {
			darwinia,
			account_id,
			signer,
			real,
		}
	}

    fn real(&self) -> &AccountId {
        if let Some(real_account_id) = &self.real {
            real_account_id
        } else {
            &self.account_id
        }
    }

    /// is_sudo_key
    pub async fn is_sudo_key(&self) -> Result<bool> {
        let sudo = self.darwinia.subxt.key(None).await?;
        Ok(&sudo == self.real())
    }

    /// role
    pub async fn role(&self) -> Result<Vec<String>> {
        let mut roles = vec!["Normal".to_string()];
        if self.is_sudo_key().await? {
            roles.push("Sudo".to_string());
        }
        Ok(roles)
    }
}

