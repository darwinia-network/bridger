use support_lifeline::error::SupportLifelineError;
use thiserror::Error as ThisError;

pub type BinS2SResult<T> = Result<T, BinS2SError>;

#[derive(ThisError, Debug)]
pub enum BinS2SError {
    #[error(transparent)]
    SupportLifeline(#[from] SupportLifelineError),
    #[error("Custom: {0}")]
    Custom(String),
}
