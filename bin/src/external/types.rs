use serde::{Deserialize, Serialize};

/// Compile channel
#[derive(
    Clone, Debug, Deserialize, Eq, PartialEq, Serialize, strum::EnumString, strum::EnumVariantNames,
)]
pub enum CompileChannel {
    /// Debug
    Debug,
    /// Release
    Release,
}
