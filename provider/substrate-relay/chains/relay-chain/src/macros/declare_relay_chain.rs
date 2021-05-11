/// declare relay chain
#[macro_export]
macro_rules! declare_relay_chain {
    ($chain_name:ident, $({ $($impl_code:tt)* });*) => {
        paste::item! {
            pub struct [<RelayChain $chain_name>];
            impl RelayChain for [<RelayChain $chain_name>] {
                $($($impl_code)*)*
            }
        }
    };
}
