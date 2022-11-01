use thiserror::Error as ThisError;

pub type SupportLifelineResult<T> = Result<T, SupportLifelineError>;

#[derive(ThisError, Debug)]
pub enum SupportLifelineError {
    #[error("Custom: {0}")]
    Custom(String),
}
