use lifeline::error::TakeResourceError;
use relay_s2s::error::RelayError;
use thiserror::Error as ThisError;

use support_lifeline::error::SupportLifelineError;

pub type BinS2SResult<T> = Result<T, BinS2SError>;

#[derive(ThisError, Debug)]
pub enum BinS2SError {
    #[error(transparent)]
    SupportLifeline(#[from] SupportLifelineError),
    #[error(transparent)]
    LifelineTakeResource(#[from] TakeResourceError),
    #[error(transparent)]
    Relay(#[from] RelayError),
    #[error("Custom: {0}")]
    Custom(String),
}

impl From<BinS2SError> for SupportLifelineError {
    fn from(e: BinS2SError) -> Self {
        Self::Custom(format!("{:?}", e))
    }
}
