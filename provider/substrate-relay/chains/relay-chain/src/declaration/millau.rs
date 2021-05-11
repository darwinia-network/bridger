use sp_version::RuntimeVersion;

use relay_millau_client::Millau;

use crate::*;

declare_cli_chain!(Millau, millau_runtime);

declare_relay_chain!(millau, {
    const CHAIN_NAME: &'static str = "millau";
    const RUNTIME_VERSION: RuntimeVersion = millau_runtime::VERSION;
    type Runtime = millau_runtime::Runtime;
    type HeaderId = relay_millau_client::HeaderId;
    type Chain = relay_millau_client::Millau;
    type SigningParams = relay_millau_client::SigningParams;
    type SyncHeader = relay_millau_client::SyncHeader;
});
