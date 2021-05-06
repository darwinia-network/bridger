use crate::DarwiniaAccount;
use substrate_subxt::Runtime;

/// Account
#[derive(Clone)]
pub struct Account<R: Runtime>(pub DarwiniaAccount<R>);

impl<R: Runtime> Account<R> {
	/// Create a new Account
	pub fn new(darwinia_account: DarwiniaAccount<R>) -> Self {
		Self(darwinia_account)
	}
}
