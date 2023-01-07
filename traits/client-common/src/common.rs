use bp_runtime::Chain;

pub trait ClientCommon: 'static + Send + Sync + Clone {
    /// chain name
    const CHAIN: &'static str;

    /// chain types
    type Chain: Chain;
}
