use sp_version::RuntimeVersion;

pub trait RelayChain {
    const CHAIN_NAME: &'static str;
    const RUNTIME_VERSION: RuntimeVersion;
    /// Chain runtime object
    type Runtime;
    /// Chain header id.
    type HeaderId;
    type Chain: relay_substrate_client::ChainBase
        + relay_substrate_client::Chain
        + relay_substrate_client::ChainWithBalances
        + relay_substrate_client::TransactionSignScheme;
    /// Chain signing params.
    type SigningParams;
    /// Chain header type used in headers sync.
    type SyncHeader;
}
