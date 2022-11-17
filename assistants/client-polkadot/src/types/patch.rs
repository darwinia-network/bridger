use subxt::DefaultExtra;

use crate::config::PolkadotSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<PolkadotSubxtConfig>;
