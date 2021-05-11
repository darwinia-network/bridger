pub use pangolin_runtime as runtime;
pub use relay_pangolin_client as relay_client;
use sp_version::RuntimeVersion;

use relay_chain::RelayChain;

pub struct RelayChainPangolin;

impl RelayChain for RelayChainPangolin {
    const CHAIN_NAME: &'static str = "pangolin";
    const RUNTIME_VERSION: RuntimeVersion = pangolin_runtime::VERSION;
    type Runtime = pangolin_runtime::Runtime;
    type HeaderId = relay_pangolin_client::HeaderId;
    type Chain = relay_pangolin_client::PangolinRelayChain;
    type SigningParams = relay_pangolin_client::SigningParams;
    type SyncHeader = relay_pangolin_client::SyncHeader;
}
