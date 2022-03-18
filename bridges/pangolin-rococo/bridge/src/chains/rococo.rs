pub use s2s_const::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use client_rococo::RococoChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for RococoChain {
        const RUNTIME_VERSION: RuntimeVersion =  RuntimeVersion {
            spec_name: sp_version::create_runtime_str!("rococo"),
            impl_name: sp_version::create_runtime_str!("parity-rococo-v2.0"),
            authoring_version: 0,
            spec_version: 9142,
            impl_version: 0,
            apis: sp_version::create_apis_vec![[]],
            transaction_version: 0
        };

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}
