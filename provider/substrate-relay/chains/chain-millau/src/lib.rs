pub use millau_runtime as runtime;
pub use relay_millau_client as relay_client;
use sp_version::RuntimeVersion;

use relay_chain::RelayChain;

pub struct RelayChainMillau;

impl RelayChain for RelayChainMillau {
    const CHAIN_NAME: &'static str = "millau";
    const RUNTIME_VERSION: RuntimeVersion = millau_runtime::VERSION;
    type Runtime = millau_runtime::Runtime;
    type HeaderId = relay_millau_client::HeaderId;
    type Chain = relay_millau_client::Millau;
    type SigningParams = relay_millau_client::SigningParams;
    type SyncHeader = relay_millau_client::SyncHeader;
}
