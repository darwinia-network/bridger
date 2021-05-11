use sp_version::RuntimeVersion;

use chain_millau::{relay_client::Millau as RelayClientMillau, runtime as millau_runtime};

use crate::*;

declare_relay_cli_chain!(RelayClientMillau, millau_runtime);
