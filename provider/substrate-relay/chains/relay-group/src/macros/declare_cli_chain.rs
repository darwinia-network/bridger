/// declare s2s relay cli chain
#[macro_export]
macro_rules! declare_relay_cli_chain {
    ($relay_chain_name:ident, $relay_chain_runtime:ident) => {
        paste::item! {
            impl CliChain for [<$relay_chain_name>] {
                const RUNTIME_VERSION: RuntimeVersion = [<$relay_chain_runtime>]::VERSION;

                type KeyPair = sp_core::sr25519::Pair;
            }
        }
    };
}
