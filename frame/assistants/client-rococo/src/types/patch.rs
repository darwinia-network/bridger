use subxt::DefaultExtra;

use crate::config::RococoSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<RococoSubxtConfig>;
