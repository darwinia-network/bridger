use frame_support::weights::Weight;

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

    // // may we don't need there functions.

    // /// Bridge Message Payload type.
    // ///
    // /// TODO [https://github.com/paritytech/parity-bridges-common/issues/854] This should be removed in favour of target-specifc types.
    // type MessagePayload;
    //
    // /// Numeric value of SS58 format.
    // fn ss58_format() -> u16;
    //
    // /// Construct message payload to be sent over the bridge.
    // fn encode_message(
    //     message: crate::cli::encode_message::MessagePayload,
    // ) -> Result<Self::MessagePayload, String>;
    //
    // /// Maximal extrinsic weight (from the runtime).
    // fn max_extrinsic_weight() -> Weight;
}
