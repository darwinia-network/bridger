use subxt::DefaultExtra;

use crate::config::KusamaSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<KusamaSubxtConfig>;
