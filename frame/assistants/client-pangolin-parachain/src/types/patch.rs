use subxt::DefaultExtra;

use crate::config::PangolinParachainSubxtConfig;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<PangolinParachainSubxtConfig>;
