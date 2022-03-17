pub use s2s_const::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use client_rococo::RococoChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for RococoChain {
        const RUNTIME_VERSION: RuntimeVersion = rococo_runtime::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}
