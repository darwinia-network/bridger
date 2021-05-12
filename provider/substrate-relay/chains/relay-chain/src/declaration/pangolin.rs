use relay_pangolin_client::PangolinRelayChain;
use sp_version::RuntimeVersion;

use crate::*;

pub use pangolin_runtime;

declare_cli_chain!(PangolinRelayChain, pangolin_runtime);

declare_relay_chain!(Pangolin, {
    const CHAIN_NAME: &'static str = "Pangolin";
    const RUNTIME_VERSION: RuntimeVersion = pangolin_runtime::VERSION;
    type Runtime = pangolin_runtime::Runtime;
    type HeaderId = relay_pangolin_client::HeaderId;
    type Chain = relay_pangolin_client::PangolinRelayChain;
    type SigningParams = relay_pangolin_client::SigningParams;
    type SyncHeader = relay_pangolin_client::SyncHeader;
});
