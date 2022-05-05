/// Bridge-supported network definition.
///
/// Used to abstract away CLI commands.
pub trait CliChain: relay_substrate_client::Chain {
    /// Chain's current version of the runtime.
    const RUNTIME_VERSION: sp_version::RuntimeVersion;

    /// Crypto keypair type used to send messages.
    ///
    /// In case of chains supporting multiple cryptos, pick one used by the CLI.
    type KeyPair: sp_core::crypto::Pair;
}
