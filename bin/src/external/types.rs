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

impl CompileChannel {
    pub fn name(&self) -> &'static str {
        match self {
            CompileChannel::Debug => "debug",
            CompileChannel::Release => "release",
        }
    }
}
