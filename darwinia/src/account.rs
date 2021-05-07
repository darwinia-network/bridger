use sp_keyring::sr25519::sr25519::Pair;
use substrate_subxt::sp_runtime::traits::{IdentifyAccount, Verify};
use substrate_subxt::{
	sp_core::Pair as PairTrait, system::System, PairSigner, Runtime, SignedExtension, SignedExtra,
	Signer,
};

/// AccountId
pub type AccountId<T> = <T as System>::AccountId;

/// Account
pub struct DarwiniaAccount<R: Runtime> {
	/// Account Id
	pub account_id: <R as System>::AccountId,
	/// signer of the account
	pub signer: PairSigner<R, Pair>,
	/// proxy real
	pub real: Option<<R as System>::AccountId>,
}

impl<R: Runtime + Clone> Clone for DarwiniaAccount<R> {
	fn clone(&self) -> Self {
		Self {
			account_id: self.account_id.clone(),
			signer: self.signer.clone(),
			real: self.real.clone(),
		}
	}
}

impl<R> DarwiniaAccount<R>
where
	R: Runtime,
	<R as Runtime>::Signature: From<sp_keyring::sr25519::sr25519::Signature>,
	<<R as Runtime>::Signature as Verify>::Signer: From<sp_keyring::sr25519::sr25519::Public>,
{
	/// Create a new Account
	pub fn new(seed: String, real: Option<String>) -> DarwiniaAccount<R>
	where
		<<R as Runtime>::Signature as Verify>::Signer:
			IdentifyAccount<AccountId = <R as System>::AccountId>,
		<R as System>::AccountId: Into<<R as System>::Address>,
		<<<R as Runtime>::Extra as SignedExtra<R>>::Extra as SignedExtension>::AdditionalSigned:
			std::marker::Send,
	{
		// signer to sign darwinia extrinsic
		let pair = Pair::from_string(&seed, None).unwrap(); // if not a valid seed
		let signer = PairSigner::<R, Pair>::new(pair);
		let account_id = signer.account_id().clone();

		// real account, convert to account id
		let real = real.map(|real| {
			let r = array_bytes::hex2array_unchecked!(real, 32);
			let r = sp_keyring::sr25519::sr25519::Public::from_raw(r);
			<R::Signature as Verify>::Signer::from(r).into_account()
		});

		DarwiniaAccount {
			account_id,
			signer,
			real,
		}
	}

	/// get the real account
	pub fn real(&self) -> &<R as System>::AccountId {
		if let Some(real_account_id) = &self.real {
			real_account_id
		} else {
			&self.account_id
		}
	}
}
