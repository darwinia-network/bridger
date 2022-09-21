use thiserror::Error as ThisError;

pub type E2EClientResult<T> = Result<T, E2EClientError>;

#[derive(ThisError, Debug)]
pub enum E2EClientError {
    #[error("RPC: {0}")]
    RPC(String),
    #[error("Custom: {0}")]
    Custom(String),
}
