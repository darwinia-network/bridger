use relay_millau_client::Millau;
use sp_version::RuntimeVersion;

use crate::*;

pub use millau_runtime;

declare_cli_chain!(Millau, millau_runtime);

declare_relay_chain!(Millau, {
    const CHAIN_NAME: &'static str = "Millau";
    const RUNTIME_VERSION: RuntimeVersion = millau_runtime::VERSION;
    type Runtime = millau_runtime::Runtime;
    type HeaderId = relay_millau_client::HeaderId;
    type Chain = relay_millau_client::Millau;
    type SigningParams = relay_millau_client::SigningParams;
    type SyncHeader = relay_millau_client::SyncHeader;
});
