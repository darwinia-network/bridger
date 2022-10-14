use subxt::DefaultExtra;

use crate::config::MoonbaseSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<MoonbaseSubxtConfig>;
