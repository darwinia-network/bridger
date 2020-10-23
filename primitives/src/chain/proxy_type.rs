use codec::{Decode, Encode};

/// A value defining the subset of calls that it is allowed to make.
#[derive(Clone, Encode, Decode)]
pub enum ProxyType {
    /// Any
    Any,
    /// NonTransfer
    NonTransfer,
    /// Governance
    Governance,
    /// Staking
    Staking,
    /// IdentityJudgement
    IdentityJudgement,
    /// EthereumBridge
    EthereumBridge,
}

/// default value
impl Default for ProxyType {
    fn default() -> ProxyType {
        ProxyType::Any
    }
}
