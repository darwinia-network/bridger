pub use s2s_const::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use client_pangolin::PangolinChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangolinChain {
        const RUNTIME_VERSION: RuntimeVersion = pangolin_runtime::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}