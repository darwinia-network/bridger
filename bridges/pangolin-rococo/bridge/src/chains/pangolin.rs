pub use s2s_const::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use relay_pangolin_client::PangolinChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangolinChain {
        const RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
            spec_name: sp_runtime::create_runtime_str!("Pangolin"),
            impl_name: sp_runtime::create_runtime_str!("Pangolin"),
            authoring_version: 0,
            spec_version: 2_8_06_0,
            impl_version: 0,
            apis: sp_version::create_apis_vec![[]],
            transaction_version: 0,
        };

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}
