use sp_version::RuntimeVersion;

use chain_pangolin::{
    relay_client::PangolinRelayChain as RelayClientPangolin, runtime as pangolin_runtime,
};

use crate::*;

declare_relay_cli_chain!(RelayClientPangolin, pangolin_runtime);
