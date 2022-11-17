use subxt::DefaultExtra;

use crate::config::DarwiniaSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<DarwiniaSubxtConfig>;
