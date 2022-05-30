pub use s2s_const::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use relay_kusama_client::Kusama;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for Kusama {
        const RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
            spec_name: sp_version::create_runtime_str!("kusama"),
            impl_name: sp_version::create_runtime_str!("parity-kusama-v2.0"),
            authoring_version: 0,
            spec_version: 9142,
            impl_version: 0,
            apis: sp_version::create_apis_vec![[]],
            transaction_version: 0,
        };

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}
