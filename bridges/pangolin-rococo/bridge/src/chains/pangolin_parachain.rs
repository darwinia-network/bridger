pub use s2s_const::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use relay_pangolin_parachain_client::PangolinParachainChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangolinParachainChain {
        const RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
            spec_name: sp_runtime::create_runtime_str!("Pangolin Parachain"),
            impl_name: sp_runtime::create_runtime_str!("Pangolin Parachain"),
            authoring_version: 1,
            spec_version: 3,
            impl_version: 1,
            apis: sp_version::create_apis_vec![[]],
            transaction_version: 1,
        };

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}
