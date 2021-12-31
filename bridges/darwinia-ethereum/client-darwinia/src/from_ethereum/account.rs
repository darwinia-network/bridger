use crate::account::DarwiniaAccount;

/// Account
#[derive(Clone)]
pub struct Account(pub DarwiniaAccount);

impl Account {
    /// Create a new Account
    pub fn new(darwinia_account: DarwiniaAccount) -> Self {
        Self(darwinia_account)
    }
}
