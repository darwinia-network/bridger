use serde::{Deserialize, Serialize};

use component_subscan::SubscanConfig;

/// feemarket config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeemarketConfig {
    /// left chain subscan config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscan_left: Option<SubscanConfig>,
    /// right chain subscan config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscan_right: Option<SubscanConfig>,
}
