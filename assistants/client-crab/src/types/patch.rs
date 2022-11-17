use subxt::DefaultExtra;

use crate::config::CrabSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<CrabSubxtConfig>;
