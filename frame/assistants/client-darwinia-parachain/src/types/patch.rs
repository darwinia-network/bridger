use subxt::DefaultExtra;

use crate::config::DarwiniaParachainSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<DarwiniaParachainSubxtConfig>;
