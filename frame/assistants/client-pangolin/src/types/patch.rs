use subxt::DefaultExtra;

use crate::config::PangolinSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<PangolinSubxtConfig>;
