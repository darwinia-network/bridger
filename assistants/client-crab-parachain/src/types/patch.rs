use subxt::DefaultExtra;

use crate::config::CrabParachainSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<CrabParachainSubxtConfig>;
